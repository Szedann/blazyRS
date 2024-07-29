use crate::Context;
use crate::Error;
use poise::serenity_prelude as serenity;
use poise::CreateReply;

#[poise::command(
    slash_command,
    description_localized("en-US", "get info about a member"),
)]
pub async fn member(
    ctx: Context<'_>,
    #[description = "Member to get info about"] member: serenity::Member,
) -> Result<(), Error> {
    let avatar_url = member.avatar_url().or(member.user.avatar_url()).unwrap_or_else(||member.user.default_avatar_url());
    let embed = serenity::CreateEmbed::default()
        .thumbnail(avatar_url)
        .title(member.display_name())
        .field("username", &member.user.name, true)
        .field("id", member.user.id.to_string(), true);
    let reply = CreateReply {
        embeds: vec![embed],
        ..Default::default()
    };
    ctx.send(reply).await?;
    Ok(())
}
