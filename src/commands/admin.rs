//! Admin tools

use std::path::Path;
use serenity::{framework::standard::{macros::command, Args, CommandResult}, http::AttachmentType};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{data::*, utils};

/// Backup current GM data
#[command]
pub async fn export(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    // TODO: Creates a text file with game roles and their associated GMs

    let data_lock = {
        let data_read = ctx.data.read().await;
        data_read.get::<GameData>().expect("Couldn't find GameData in ctx.data").clone()
    };

    {
        let guard = data_lock.read().await;
        let filename = "exports/data.json";
        if let Err(e) = utils::export_json(&guard, filename).await {
            // Failed
            msg.channel_id.say(&ctx.http, format!("Failed to export to json! Error: {:?}", e)).await?;

            // TODO: Return an error
            return Ok(());
        } else {
            let msg = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("Data");
                m.add_file(AttachmentType::Path(Path::new(filename)));
                m
            }).await;

            if let Err(why) = msg {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    msg.channel_id
        .say(&ctx.http, "Sent you a backup of the data!")
        .await?;

    Ok(())
}
