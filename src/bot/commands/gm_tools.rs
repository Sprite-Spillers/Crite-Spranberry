use crate::bot::utils::*;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    utils::parse_role,
};
use std::error::Error;

/// Tools for GMs/DMs

/// Create a new game
#[command]
pub async fn create(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: Creates a role with provided name and adds user to list of GMs for the role
    let http = &ctx.http;
    let guild = msg.guild_id.expect("Failed to get guild ID!");
    let name = args.single::<String>().unwrap();
    let role = guild
        .create_role(http, |r| r.hoist(true).name(name))
        .await
        .expect("Failed to get role!");

    msg.channel_id
        .say(&ctx.http, "Created <game> role!")
        .await?;

    Ok(())
}

/// Add a player to a game
#[command]
pub async fn invite(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: Add player with mentions for role and user

    args.quoted();

    let role_name = args.single::<String>().unwrap();
    let role_option = parse_role(&role_name);
    let guild_roles = ctx.cache.guild_roles(msg.guild_id.unwrap()).await.unwrap();
    let mut found_role: Option<Role> = None;
    if let Some(i) = role_option {
        // If role mention found, use it
        let id = RoleId::from(i);
        found_role = Some(guild_roles.get(&id).unwrap().to_owned());
    } else {
        // Otherwise try to match by role name
        for (_, role) in guild_roles {
            if role.name.to_lowercase() == role_name.to_lowercase() {
                found_role = Some(role);
                break;
            }
        }
    }
    
    // Couldn't find role, 
    if let None = found_role {
        msg.channel_id
        .say(&ctx.http, format!("Couldn't find the role: {}!", role_name))
        .await?;
        // TODO: Return an error
        return Ok(());
    }

    println!("{}", &found_role.unwrap().name);


    let user: User = match args.single_quoted::<String>() {
        Ok(arg) => match parse_member(ctx, msg, arg).await {
            Some(m) => m.user,
            None => {
                reply(ctx, msg, &"Unable to locate user".to_string()).await;
                return Ok(());
            }
        },
        Err(_e) => msg.author.to_owned(),
    };

    let player = "player";
    msg.channel_id
        .say(&ctx.http, format!("Added {} to {}!", player, role_name))
        .await?;

    Ok(())
}

/// Remove a player from a game
#[command]
pub async fn remove(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: Remove player with mentions for role and user

    msg.channel_id
        .say(&ctx.http, "Removed <player> from <game>!")
        .await?;

    Ok(())
}

/// Rename a game, including role and channel category
#[command]
pub async fn rename(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: Change name of role and associated channel category, if it exists

    msg.channel_id
        .say(&ctx.http, "Renamed <old name> to <new name>!")
        .await?;

    Ok(())
}

/// Create channel category for a game and give GMs permissions
#[command]
pub async fn add_channels(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: If channel category exists, print error message. Otherwise create new category
    // with a text channel with the game name and a

    msg.channel_id
        .say(&ctx.http, "Created new channel category for <game>!")
        .await?;

    Ok(())
}
