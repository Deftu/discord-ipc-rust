use serde::{Deserialize, Serialize};

// TODO: move this to somewhere else
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "evt", content = "args")]
pub enum SubscribeableEvent {
    GuildStatus { guild_id: String },
    GuildCreate(GuildCreateData),
    ChannelCreate(ChannelCreateData),
    VoiceChannelSelect(VoiceChannelSelectData),
    VoiceStateCreate { channel_id: String },
    VoiceStateUpdate { channel_id: String },
    VoiceStateDelete { channel_id: String },
    VoiceSettingsUpdate, // No args
    VoiceConnectionStatus,
    SpeakingStart { channel_id: String },
    SpeakingStop { channel_id: String },
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    NotificationCreate,
    ActivityJoin,
    ActivitySpectate,
    ActivityJoinRequest,
    CurrentUserUpdate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildCreateData {
    /// string - Guild ID
    pub id: String,
    /// string - Name of the guild
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelCreateData {
    /// string - Channel ID
    pub id: String,
    /// string - Name of the channel
    pub name: String,
    /// integer - Channel type (guild text: 0, guild voice: 2, DM: 1, group DM: 3)
    #[serde(rename = "type")]
    pub channel_type: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceChannelSelectData {
    /// string - Channel ID
    pub channel_id: String,
    /// string - Guild ID
    pub guild_id: String,
}
