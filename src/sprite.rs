use std::env;

use serenity::prelude::*;
use serenity::model::channel::Message;

pub(crate) async fn octopus_check(ctx: &Context, msg: &Message) {
    // Everything is octopus, Oliver
    let r = env::var("OCTOPUS");
    if let Ok(id) = r {
        if *msg.author.id.as_u64().to_string() == id || msg.mentions.iter().any(|u| u.id.to_string() == id) {
            let _ = msg.react(ctx, '🐙').await;
        };
    }
}

pub(crate) async fn groundhog_check(ctx: &Context, msg: &Message) {
    // Hope you don't get stuck in a time loop
    if msg.content.to_lowercase().contains("happy")
        && msg.content.to_lowercase().contains("groundhog")
        && msg.content.to_lowercase().contains("day") {
        let _ = msg.react(ctx, '🔄').await;
    }
}
