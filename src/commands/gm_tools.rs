//! Tools for GMs/DMs

use std::collections::HashMap;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::{macros::command, Args, CommandResult};

use serde_json::{map::Map, value::Value};

use crate::data::*;
use crate::utils::*;

/// Create a new game and associated role, and assigns the role to the caller
/// 
/// Role name should be put in quotes if it contains spaces.
#[command]
pub async fn create(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    args.quoted();
    args.trimmed();

    let http = &ctx.http;
    let guild = msg.guild_id.expect("Failed to get guild ID!");
    let name = args.single::<String>().unwrap();
    let role = guild
        .create_role(http, |r| r.hoist(false).name(name))
        .await
        .expect("Failed to create role!");

    let mut creator = get_member(ctx, msg, &msg.author.name).await.unwrap();
    creator.add_role(&ctx.http, role.id).await?;

    let counter_lock = {
        let data_read = ctx.data.read().await;
        data_read.get::<GameData>().expect("Couldn't find GameData in ctx.data").clone()
    };

    {
        let mut guard = counter_lock.write().await;
        let server = guard.entry(msg.guild_id.unwrap()).or_insert(ServerData { games: HashMap::new() });
        server.new_game(role.name.clone(), creator, role);
    }

    msg.channel_id
        .say(&ctx.http, "Created <game> role!")
        .await?;

    Ok(())
}

/// Add a player to a game
/// 
/// Accepts either mentions or (case insensitive) names for both the role and 
/// the users. If the names contain spaces, they should be put in quotes.
#[command]
pub async fn invite(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    args.quoted();
    args.trimmed();

    let role_to_find = args.single::<String>().unwrap();
    let role_option = get_role(ctx, msg, &role_to_find).await;

    // Couldn't find role, print message and quit
    if role_option.is_none() {
        msg.channel_id
        .say(&ctx.http, format!("Couldn't find the role: {}!", role_to_find))
        .await?;
        // TODO: Return an error
        return Ok(());
    }

    let role = role_option.unwrap();
    let role_name = &role.name;

    for arg in args.iter::<String>() {
        let member_to_find = arg.unwrap();
        let member_option = get_member(ctx, msg, &member_to_find).await;
        
        if let Some(mut member) = member_option {
            // Add role to user
            member.add_role(&ctx.http, role.id).await?;
            msg.channel_id
            .say(&ctx.http, format!("Added {} to \"{}\"!", member.display_name(), role_name))
            .await?;
        } else {
            // Couldn't find user, go to next
            msg.channel_id
            .say(&ctx.http, format!("Couldn't find user: {}!", member_to_find))
            .await?;
            continue;
        }
    }

    Ok(())
}

/// Remove a player from a game
/// 
/// Accepts either mentions or (case insensitive) names for both the role and 
/// the users. If the names contain spaces, they should be put in quotes.
#[command]
pub async fn remove(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: Remove player with mentions for role and user
    args.quoted();
    args.trimmed();

    msg.channel_id
        .say(&ctx.http, "Removed <player> from <game>!")
        .await?;

    Ok(())
}

/// Rename a game, including role and channel category
#[command]
pub async fn rename(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: Change name of role and associated channel category, if it exists
    // Display error if run by someone who does not have GM perms for that game
    // and is not admin
    args.quoted();
    args.trimmed();

    msg.channel_id
        .say(&ctx.http, "Renamed <old name> to <new name>!")
        .await?;

    Ok(())
}

/// Create channel category for a game and give GMs permissions
#[command]
pub async fn add_channels(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: If channel category exists, print error message. Otherwise create new category
    // with a text channel with the game name and give caller manage_channels permission in
    // the category. Display error if run by someone who does not have GM perms for that game
    // and is not admin
    args.quoted();
    args.trimmed();

    let map: Map<String, Value> = Map::new();
    // TODO: Figure out create_channel
    // map.insert("my-test-channel".to_string(), ChannelType::Category.to_string());

    let _ = ctx.http
    .create_channel(*msg.guild_id.unwrap().as_u64(), &map)
    .await;

    msg.channel_id
        .say(&ctx.http, "Created new channel category for <game>!")
        .await?;

    Ok(())
}
