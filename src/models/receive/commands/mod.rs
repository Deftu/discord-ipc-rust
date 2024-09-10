use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::shared::channel::Channel;

/// All command responses that come back from the discord RPC
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "cmd", content = "data")]
pub enum ReturnedCommand {
    GetSelectedVoiceChannel(Option<Channel>),
    SelectVoiceChannel(Channel),

    Subscribe(HashMap<String, String>),
    Dispatch(HashMap<String, String>),
}
