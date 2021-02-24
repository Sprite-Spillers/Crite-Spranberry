use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::collections::{HashMap, HashSet};

use serenity::{
    prelude::*,
    async_trait,
    client::bridge::gateway::GatewayIntents,
    framework::standard::{
        Args, CommandOptions, Reason,
        macros::{check, group, hook},
        StandardFramework,
    },
    http::Http,
    model::{
        channel::Message,
        gateway::Ready,
        Permissions
    }
};


mod commands;
mod data;
mod sprite;
mod utils;

use commands::{admin::*, debug::*, roles::*};
use data::BotData;

const COMMAND_PREFIX: &str = "~";

#[group]
#[required_permissions("ADMINISTRATOR")]
#[commands(export)]
struct Admin;

#[group]
#[owners_only]
#[commands(list, list_roles, list_channels)]
struct Debug;

#[group]
#[prefix = "role"]
#[checks(ManageRoles)]
#[description = "Tools for managing roles"]
#[only_in("guilds")]
#[commands(create, invite, remove, rename)]
struct Game;


#[check]
#[name = "ManageRoles"]
async fn role_manager_check(ctx: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> Result<(), Reason> {
    if let Some(member) = &msg.member {
        for role in &member.roles {
            if let Some(role) = role.to_role_cached(&ctx.cache).await {
                if role.has_permission(Permissions::MANAGE_ROLES) || role.name == "DMs" {
                    return Ok(());
                }
            }
        }
    }

    Err(Reason::User("User doesn't have permission to manage roles.".to_string()))
}


// Event handler
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        sprite::octopus_check(&ctx, &msg).await;
        sprite::groundhog_check(&ctx, &msg).await;
    }
}


// Define functions for the framework
#[hook]
async fn before(_ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!(
        "Got command '{}' by user '{}'",
        command_name, msg.author.name
    );

    true // if `before` returns false, command processing doesn't happen.
}

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}


// Start the bot
#[tokio::main]
async fn main() {
    // Try getting environment variables
    let res = env::var("DISCORD_TOKEN");
    let dotenv_path = Path::new(".env");

    // Env vars not found, import from file
    if res.is_err() {
        println!("Environment variables not found, trying .env file");

        // If dotenv file not found, create empty file and exit
        if !dotenv_path.exists() {
            println!(".env file not found! Creating empty file and exiting");
            File::create(dotenv_path)
                .expect("Error while creating empty .env file!");
            std::process::exit(1);
        }

        dotenv::from_path(dotenv_path).expect("Error while loading environment variables!");
    }

    let token = env::var("DISCORD_TOKEN").expect("Error while getting token from environment!");
    println!("Found bot token!");

    let http = Http::new_with_token(&token);

    // Get bot owners and ID
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create framework for bot
    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix(COMMAND_PREFIX)
                // Sets the bot's owners. These will be used for commands that
                // are owners only.
                .owners(owners)
        })
        .before(before)
        .unrecognised_command(unknown_command)
        .group(&ADMIN_GROUP)
        .group(&DEBUG_GROUP)
        .group(&GAME_GROUP);

    // Set Gateway Intents
    let intents = GatewayIntents::all();

    // Log in
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .intents(intents)
        .await
        .expect("Error while creating client!");

    // Get data if it exists, otherwise create empty map.
    // Inside a block to minimize the scope of the rwlock
    {
        let path = Path::new("data/bot.json");
        let mut data = client.data.write().await;
        if let Ok(bot_data) = utils::import_json(path).await {
            // Try local file first
            data.insert::<BotData>(Arc::new(RwLock::new(bot_data)));
            println!("Successfully imported existing data from file!")
        } else if let Ok(bot_data) = utils::import_from_github().await {
            // Try github
            data.insert::<BotData>(Arc::new(RwLock::new(bot_data)));
            println!("Successfully imported existing data from Github!")
        } else {
            // Otherwise create empty map
            data.insert::<BotData>(Arc::new(RwLock::new(HashMap::new())));
            println!("Couldn't find data to import, initializing with empty map")
        }
    }

    // Create folder for exports
    if let Err(e) = fs::create_dir_all("exports") {
        println!("Error creating export dir: {:?}", e)
    }

    // Start client
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
