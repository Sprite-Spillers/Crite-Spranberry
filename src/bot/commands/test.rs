use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{content_safe, ContentSafeOptions};

/// Test commands

/// Print out all arguments passed
#[command]
async fn echo(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // if let Some(guild) = msg.guild() {
    //     let guild = guild.read().expect("Failed to acquire read lock");

    // }

    // let content = content_safe(&ctx.cache, &args.rest(), &settings).await;
    let user = &msg.mentions[0];
    // let player: Member =
    println!("{}", user.to_string());
    let game = args.current();

    // msg.channel_id.say(&ctx.http, &player).await?;

    // player.add_role(&ctx.http, game);

    // let content = &args.current().unwrap();
    // msg.channel_id.say(&ctx.http, &content).await?;

    Ok(())
}

#[command]
async fn guild(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    println!("{}", ctx.cache.shard_count().await);

    Ok(())
}
