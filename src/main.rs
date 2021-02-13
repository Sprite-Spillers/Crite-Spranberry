mod bot;

use async_std::fs;
use async_std::fs::File;
use async_std::path::Path;

use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager, GatewayIntents},
    framework::standard::{
        help_commands,
        // buckets::{RevertBucket, LimitedFor},
        macros::{check, command, group, help, hook},
        Args,
        CommandGroup,
        CommandOptions,
        CommandResult,
        DispatchError,
        HelpOptions,
        Reason,
        StandardFramework,
    },
    http::Http,
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
    // utils::{content_safe, ContentSafeOptions},
};
use std::{
    collections::{HashMap, HashSet},
    env,
    fmt::Write,
    sync::Arc,
};

use serenity::prelude::*;
use tokio::sync::Mutex;

use bot::commands::{admin::*, debug::*, gm_tools::*, test::*};

const COMMAND_PREFIX: &str = "~";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(backup)]
struct Admin;

#[group]
#[owners_only]
#[commands(list, list_roles, list_channels)]
struct Debug;

#[group]
#[commands(echo, guild)]
struct Test;

#[group]
#[prefix = "game"]
#[description = "Tools for GMs to manage their games"]
#[only_in("guilds")]
#[commands(create, invite, remove, rename)]
struct Game;

// Define functions for the framework

#[hook]
async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
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

#[tokio::main]
async fn main() {
    // If token file not found, create empty file and exit
    let token_path = Path::new("token.txt");
    if !token_path.exists().await {
        File::create(token_path)
            .await
            .expect("Error while creating empty token file!");
        println!("Token file not found! Creating empty file and exiting");
        return;
    }

    // Read token from file and quit if token is empty
    let token = fs::read_to_string(token_path)
        .await
        .expect("Error while reading token from file!");
    if token.is_empty() {
        println!("Token file is empty! Shutting down");
        return;
    }

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
        .group(&TEST_GROUP)
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
