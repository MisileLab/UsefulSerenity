/// Vec<(content: String, description: String, inline: bool)>
fn embed_converter(embeds: Vec<(String, String, bool)>) -> serenity::CreateEmbed {
    let mut embed = serenity::CreateEmbed::default();
    embed.fields(embeds);
    return embed;
}