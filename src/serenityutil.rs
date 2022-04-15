use poise::serenity_prelude as serenity;

/// Vec<(content: String, description: String, inline: bool)>
pub fn add_fields(embeds: Vec<(String, String, bool)>) -> serenity::CreateEmbed {
    let mut embed = serenity::CreateEmbed::default();
    embed.fields(embeds);
    return embed;
}
