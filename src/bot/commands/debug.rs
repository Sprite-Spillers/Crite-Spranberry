use crate::bot::utils::*;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::{macros::command, Args, CommandResult};

/// Print out a list of all members in the server
#[command]
pub async fn list(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_members = ctx.cache.guild_field(msg.guild_id.unwrap(), |guild| guild.members.to_owned()).await.unwrap();
    for (id, member) in guild_members {
        println!("{}", member.display_name());
    }

    Ok(())
}
