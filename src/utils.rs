use serenity::{
    model::prelude::ReactionType::Unicode, model::prelude::*, prelude::*, Error,
};

pub(crate) async fn parse_member(
    ctx: &Context,
    msg: &Message,
    member_name: String,
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

pub(crate) async fn parse_channel(
    ctx: &Context,
    channel_name: String,
) -> Option<Channel> {
    let channel: Channel;
    if let Ok(id) = channel_name.parse::<u64>() {
        let channel = match ctx.http.get_channel(id).await {
            Ok(c) => c,
            Err(_e) => return None,
        };;
        Some(channel.to_owned())
    } else if channel_name.starts_with("<#") && channel_name.ends_with(">") {
        let re = Regex::new("[<#!>]").unwrap();
        let channel_id = re.replace_all(&channel_name, "").into_owned();
        channel = match ctx.http.get_channel(channel_id.parse::<u64>().unwrap()).await
        {
            Ok(m) => m,
            Err(_e) => return None,
        };
        Some(channel.to_owned())
    } else {
        None
    }
}
