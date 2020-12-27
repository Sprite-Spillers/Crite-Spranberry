use crate::bot::utils::*;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

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
    if !msg.mentions.is_empty() {
        for user in &msg.mentions {
            ;
        }
    }

    let role: &Role;
    if !msg.mention_roles.is_empty() {
        let role_id = &msg.mention_roles[0];
        role = ctx.cache
                    .guild_roles(msg.guild_id.unwrap())
                    .await
                    .unwrap()
                    .get(role_id)
                    .unwrap();
    }

    
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

    msg.channel_id
        .say(&ctx.http, "Added <player> to <game>!")
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
