use serenity::builder::CreateEmbed;

/// Vec<(content: String, description: String, inline: bool)>
pub fn add_embeds(embeds: Vec<(String, String, bool)>) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.fields(embeds);
    return embed;
}