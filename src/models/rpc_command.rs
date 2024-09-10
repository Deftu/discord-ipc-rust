use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Result;

use super::rpc_event::SentEvent;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "cmd", content = "args")]
pub enum SentCommand {
    Dispatch,
    Authorize(AuthorizeData),
    Authenticate { access_token: String },
    GetGuild(GetGuildData),
    GetGuilds,
    GetChannel,
    GetChannels,
    Subscribe(SentEvent),
    Unsubscribe(SentEvent),
    SetUserVoiceSettings,
    SelectVoiceChannel,
    GetSelectedVoiceChannel,
    SelectTextChannel,
    GetVoiceSettings,
    SetVoiceSettings,
    SetCertifiedDevices,
    SetActivity,
    SendActivityJoinInvite,
    CloseActivityRequest,
}

impl SentCommand {
    pub(crate) fn to_json(&self) -> Result<Value> {
        let command_json = match self {
            Self::Subscribe(event) => {
                let mut event_json = serde_json::to_value(event)?;
                match &mut event_json {
                    serde_json::Value::Object(object) => {
                        object.insert("cmd".to_string(), "SUBSCRIBE".into());
                        object
                    }
                    _ => panic!("Expected event to be an object"),
                };
                event_json
            }

            Self::Unsubscribe(event) => {
                let mut event_json = serde_json::to_value(event)?;
                match &mut event_json {
                    serde_json::Value::Object(object) => {
                        object.insert("cmd".to_string(), "UNSUBSCRIBE".into());
                        object
                    }
                    _ => panic!("Expected event to be an object"),
                };
                event_json
            }

            _ => {
                let value = serde_json::to_value(self)?;
                dbg!(&value);
                value
            }
        };
        println!("{}", serde_json::to_string_pretty(&command_json)?);
        Ok(command_json)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizeData {
    /// Array of OAuth2 scopes - Scopes to authorize
    pub scopes: Vec<String>,
    /// string - OAuth2 application ID
    pub client_id: String,
    /// string - One-time use RPC token
    pub rpc_token: String,
    /// string - Username to create a guest account with if the user does not have Discord
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGuildData {
    /// string - Guild ID
    pub guild_id: String,
    /// integer - Asynchronously get guild with time to wait before timing out
    pub timeout: i32,
}
