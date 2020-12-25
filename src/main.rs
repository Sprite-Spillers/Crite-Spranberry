mod commands;

use async_std::fs;
use async_std::fs::File;
use async_std::path::Path;

use std::{collections::{HashMap, HashSet}, env, fmt::Write, sync::Arc};
use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        Args, CommandOptions, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, Reason, StandardFramework,
        // buckets::{RevertBucket, LimitedFor},
        macros::{command, group, help, check, hook},
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

use serenity::prelude::*;
use tokio::sync::Mutex;

use commands::{
    gm_tools::*,
    manage_gms::*,
};

struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[prefix="game"]
#[description = "Tools for GMs to manage their games"]
#[commands(create, invite, remove, rename)]
struct Game;

// Define functions for the framework

#[hook]
async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!("Got command '{}' by user '{}'", command_name, msg.author.name);

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

    // We will fetch your bot's owners and id
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
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
                   .prefix("~")
                   // In this case, if "," would be first, a message would never
                   // be delimited at ", ", forcing you to trim your arguments if you
                   // want to avoid whitespaces at the start of each.
                   .delimiters(vec![", ", ","])
                   // Sets the bot's owners. These will be used for commands that
                   // are owners only.
                   .owners(owners))

        .before(before)
        .unrecognised_command(unknown_command)
        .group(&GAME_GROUP);

    
    // Log in
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error while creating client!");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
