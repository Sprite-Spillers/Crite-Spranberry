use log::{error, warn};
use regex::Regex;
use serenity::{model::prelude::ReactionType::Unicode, model::prelude::*, prelude::*, Error};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    utils::parse_role,
};

pub(crate) async fn reply(ctx: &Context, msg: &Message, content: &String) {
    if let Err(why) = msg.channel_id.say(&ctx.http, &content).await {
        warn!(
            "Failed to send message in #{} because\n{:?}",
            msg.channel_id, why
        );
    }
}

pub(crate) async fn find_member(
    ctx: &Context,
    msg: &Message,
    member_name: &String,
) -> Option<Member> {
    let member: Member;
    if let Ok(id) = member_name.parse::<u64>() {
        member = match msg.guild_id.unwrap().member(ctx, id).await {
            Ok(m) => m,
            Err(_e) => return None,
        };
        Some(member.to_owned())
    } else if member_name.starts_with("<@") && member_name.ends_with(">") {
        let re = Regex::new("[<@!>]").unwrap();
        let member_id = re.replace_all(&member_name, "").into_owned();

        member = match msg
            .guild_id
            .unwrap()
            .member(ctx, UserId(member_id.parse::<u64>().unwrap()))
            .await
        {
            Ok(m) => m,
            Err(_e) => return None,
        };

        Some(member.to_owned())
    } else {
        None
    }
}

pub(crate) async fn find_channel(ctx: &Context, channel_name: String) -> Option<Channel> {
    let channel: Channel;
    if let Ok(id) = channel_name.parse::<u64>() {
        let channel = match ctx.http.get_channel(id).await {
            Ok(c) => c,
            Err(_e) => return None,
        };
        Some(channel.to_owned())
    } else if channel_name.starts_with("<#") && channel_name.ends_with(">") {
        let re = Regex::new("[<#!>]").unwrap();
        let channel_id = re.replace_all(&channel_name, "").into_owned();
        channel = match ctx
            .http
            .get_channel(channel_id.parse::<u64>().unwrap())
            .await
        {
            Ok(m) => m,
            Err(_e) => return None,
        };
        Some(channel.to_owned())
    } else {
        None
    }
}

pub(crate) async fn find_role(
    ctx: &Context,
    msg: &Message,
    role_name: &String,
) -> Option<Role> {
    let role_option = parse_role(&role_name);
    let guild_roles = ctx.cache.guild_roles(msg.guild_id.unwrap()).await.unwrap();
    let found_role: Option<Role>;
    if let Some(i) = role_option {
        // If role mention found, use it
        let id = RoleId::from(i);
        return Some(guild_roles.get(&id).unwrap().to_owned());
    } else {
        // Otherwise try to match by role name
        for (_, role) in guild_roles {
            if role.name.to_lowercase() == role_name.to_lowercase() {
                return Some(role);
            }
        }
    }
    
    return None;
}
