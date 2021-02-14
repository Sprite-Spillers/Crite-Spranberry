use async_std::fs::File;
use async_std::path::Path;

use std::{collections::HashSet, env};

use serenity::prelude::*;
use serenity::{
    async_trait,
    client::bridge::gateway::GatewayIntents,
    framework::standard::{
        macros::{group, hook},
        StandardFramework,
    },
    http::Http,
    model::{
        channel::Message,
        gateway::Ready,
    }
};


mod commands;
mod data;
mod sprite;
mod utils;
use commands::{admin::*, debug::*, gm_tools::*};

const COMMAND_PREFIX: &str = "~";

#[group]
#[commands(backup)]
struct Admin;

#[group]
#[owners_only]
#[commands(list, list_roles, list_channels)]
struct Debug;

#[group]
#[prefix = "game"]
#[description = "Tools for GMs to manage their games"]
#[only_in("guilds")]
#[commands(create, invite, remove, rename)]
struct Game;


// Event handler
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        sprite::octopus_check(&ctx, &msg).await;
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
    // If token file not found, create empty file and exit
    let dotenv_path = Path::new(".env");
    if !dotenv_path.exists().await {
        File::create(dotenv_path)
            .await
            .expect("Error while creating empty .env file!");
        panic!(".env file not found! Creating empty file and exiting");
    }

    dotenv::from_path(dotenv_path).expect("Error while loading environment variables!");

    let token = env::var("DISCORD_TOKEN").expect("Error while getting token from environment!");

    let http = Http::new_with_token(&token);

    // Get bot owners and ID
    let (owners, bot_id) = match http.get_current_application_info().await {
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

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
