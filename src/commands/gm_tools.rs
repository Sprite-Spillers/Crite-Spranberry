use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

/// Tools for GMs/DMs

/// Create a new game
#[command]
pub async fn create(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: Creates a role with provided name and adds user to list of GMs for the role

    msg.channel_id.say(&ctx.http, "Created <game> role!").await?;

    Ok(())
}

/// Add a player to a game
#[command]
pub async fn invite_player(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: Add player with mentions for role and user

    msg.channel_id.say(&ctx.http, "Added <player> to <game>!").await?;

    Ok(())
}

/// Remove a player from a game
#[command]
pub async fn remove_player(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: Remove player with mentions for role and user

    msg.channel_id.say(&ctx.http, "Removed <player> from <game>!").await?;

    Ok(())
}

/// Rename a game, including role and channel category
#[command]
pub async fn rename_game(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: Change name of role and associated channel category, if it exists

    msg.channel_id.say(&ctx.http, "Renamed <old name> to <new name>!").await?;

    Ok(())
}

/// Create channel category for a game and give GMs permissions
#[command]
pub async fn add_channels(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: If channel category exists, do nothing. Otherwise create new category
    // with a text channel with the game name and a 

    msg.channel_id.say(&ctx.http, "Created new channel category for <game>!").await?;

    Ok(())
}
