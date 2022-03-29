use serenity::builder::{
    CreateInteractionResponseData, 
    CreateEmbed
};

use serenity::model::prelude::{
    InteractionApplicationCommandCallbackDataFlags
};


pub struct ResponseStruct {
    pub responsedata: CreateInteractionResponseData,
}

pub trait ResponseTrait {
    fn set_content(&self, content: String) -> ResponseStruct;
    fn add_flag(&self, flag: InteractionApplicationCommandCallbackDataFlags) -> ResponseStruct;
    fn add_embed(&self, embed: CreateEmbed) -> ResponseStruct;
    fn add_embeds(&self, embeds: Vec<CreateEmbed>) -> ResponseStruct;
}

impl ResponseTrait for ResponseStruct {
    fn set_content(&self, content: String) -> ResponseStruct {
        let mut resp = self.responsedata.clone();
        resp.content(content);
        return ResponseStruct {
            responsedata: resp,
        }
    }

    fn add_flag(&self, flag: InteractionApplicationCommandCallbackDataFlags) -> ResponseStruct {
        let mut resp = self.responsedata.clone();
        resp.flags(flag);
        return ResponseStruct {
            responsedata: resp,
        }
    }

    fn add_embed(&self, embed: CreateEmbed) -> ResponseStruct {
        let mut resp = self.responsedata.clone();
        resp.add_embed(embed);
        return ResponseStruct {
            responsedata: self.responsedata.clone()
        }
    }

    fn add_embeds(&self, embeds: Vec<CreateEmbed>) -> ResponseStruct {
        let mut resp = self.responsedata.clone();
        for embed in embeds {
            resp.add_embed(embed);
        }
        return ResponseStruct {
            responsedata: self.responsedata.clone()
        }
    }
}