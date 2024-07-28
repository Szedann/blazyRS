use crate::Context;
use crate::Error;
use ::serenity::all::CreateEmbed;
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
    let embed = CreateEmbed::new()
        .title(member.display_name())
        .field("username", member.user.name, true)
        .field("id", member.user.id.to_string(), true);
    let reply = CreateReply {
        embeds: vec![embed],
        ..Default::default()
    };
    ctx.send(reply).await?;
    Ok(())
}
