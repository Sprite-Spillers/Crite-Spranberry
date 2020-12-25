use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

/// Admin tools


/// Backup current GM data
#[command]
pub async fn backup(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // TODO: Creates a text file with game roles and their associated GMs

    msg.channel_id.say(&ctx.http, "Sent you a backup of the data!").await?;

    Ok(())
}
