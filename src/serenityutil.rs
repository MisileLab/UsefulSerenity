use serenity::model::interactions::application_command::{ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue};

pub fn get_option(command: &ApplicationCommandInteraction, value: usize) -> &ApplicationCommandInteractionDataOptionValue {
    let a = command.data
        .options
        .get(value)
        .expect("Expected option")
        .resolved
        .as_ref()
        .expect("Expected object");
    return a;
}