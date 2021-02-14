use std::env;

use serenity::prelude::*;
use serenity::model::channel::Message;

pub(crate) async fn octopus_check(ctx: &Context, msg: &Message) {
    // Everything is octopus, Oliver
    let r = env::var("OCTOPUS");
    if let Ok(id) = r {
        if *msg.author.id.as_u64().to_string() == id {
            let _ = msg.react(ctx, '🐙').await;
        };
    }
}
