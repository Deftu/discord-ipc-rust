use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Result;

use super::rpc_event::RPCEvent;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "cmd", content = "args")]
pub enum RPCCommand {
    Dispatch,
    Authorize,
    Authenticate { access_token: String },
    GetGuild,
    GetGuilds,
    GetChannel,
    GetChannels,
    Subscribe(RPCEvent),
    Unsubscribe(RPCEvent),
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

impl RPCCommand {
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
