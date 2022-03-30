use serenity::builder::{
    CreateInteractionResponseData, 
    CreateEmbed
};

use serenity::model::prelude::{
    InteractionApplicationCommandCallbackDataFlags
};


pub struct ResponseStruct<'a> {
    pub responsedata: &'a mut CreateInteractionResponseData,
}

pub trait ResponseTrait {
    fn set_content(&self, content: String) -> ResponseStruct;
    fn add_flag(&self, flag: InteractionApplicationCommandCallbackDataFlags) -> ResponseStruct;
    fn add_embed(&self, embed: CreateEmbed) -> ResponseStruct;
    fn add_embeds(&self, embeds: Vec<CreateEmbed>) -> ResponseStruct;
}

impl ResponseTrait for ResponseStruct<'_> {
    fn set_content(&self, content: String) -> ResponseStruct {
        let resp = self.responsedata;
        resp.content(content);
        return ResponseStruct {
            responsedata: resp
        }
    }

    fn add_flag(&self, flag: InteractionApplicationCommandCallbackDataFlags) -> ResponseStruct {
        let resp = self.responsedata;
        resp.flags(flag);
        return ResponseStruct {
            responsedata: resp
        }
    }

    fn add_embed(&self, embed: CreateEmbed) -> ResponseStruct {
        let resp = self.responsedata;
        resp.add_embed(embed);
        return ResponseStruct {
            responsedata: resp
        }
    }

    fn add_embeds(&self, embeds: Vec<CreateEmbed>) -> ResponseStruct {
        let resp = self.responsedata;
        for embed in embeds {
            resp.add_embed(embed);
        }
        return ResponseStruct {
            responsedata: resp
        }
    }
}