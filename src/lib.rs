use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseData, CreateEmbed};
use serenity::model::prelude::{InteractionApplicationCommandCallbackDataFlags, InteractionResponseType};

struct ResponseStruct {
    response: CreateInteractionResponse,
    responsecontent: CreateInteractionResponseData,
    embed: Option<CreateEmbed>
}

trait ResponseTrait {
    fn set_content(&self, content: String) -> ResponseStruct;
    fn add_flag(&self, flag: InteractionApplicationCommandCallbackDataFlags) -> ResponseStruct;
    fn change_kind(&self, kind: InteractionResponseType) -> ResponseStruct;
    fn set_embed(&self, embed: CreateEmbed) -> ResponseStruct;
}

impl ResponseTrait for ResponseStruct {
    fn set_content(&self, content: String) -> ResponseStruct {
        let mut resp = self.responsecontent.clone();
        resp.content(content);
        return ResponseStruct {
            response: self.response.clone(),
            responsecontent: resp,
            embed: self.embed.clone()
        }
    }

    fn add_flag(&self, flag: InteractionApplicationCommandCallbackDataFlags) -> ResponseStruct {
        let mut resp = self.responsecontent.clone();
        resp.flags(flag);
        return ResponseStruct {
            response: self.response.clone(),
            responsecontent: resp,
            embed: self.embed.clone()
        }
    }

    fn change_kind(&self, kind: InteractionResponseType) -> ResponseStruct {
        let mut resp = self.response.clone();
        resp.kind(kind);
        return ResponseStruct {
            response: resp,
            responsecontent: self.responsecontent.clone(),
            embed: self.embed.clone()
        }
    }

  fn set_embed(&self, embed: CreateEmbed) -> ResponseStruct {
      return ResponseStruct {
          response: self.response.clone(),
          responsecontent: self.responsecontent.clone(),
          embed: Some(embed)
      }
  }
}