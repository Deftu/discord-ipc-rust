use serde::{Deserialize, Serialize};

use crate::models::shared::{voice_state::VoiceState, User};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "evt", content = "data")]
/// All events that come back from the discord RPC
pub enum ReturnedEvent {
    Ready(ReadyData),
    Error(ErrorData),
    GuildStatus,
    GuildCreate(GuildCreateData),
    ChannelCreate(ChannelCreateData),
    VoiceChannelSelect(VoiceChannelSelectData),
    VoiceStateCreate(VoiceState),
    VoiceStateUpdate(VoiceState),
    VoiceStateDelete(VoiceState),
    VoiceSettingsUpdate,
    VoiceConnectionStatus(VoiceConnectionStatusData),
    SpeakingStart(SpeakingData),
    SpeakingStop(SpeakingData),
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    NotificationCreate,
    ActivityJoin,
    ActivitySpectate,
    ActivityJoinRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyData {
    #[serde(rename = "v")]
    pub version: u32,
    pub config: ReadyConfig,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyConfig {
    pub cdn_host: String,
    pub api_endpoint: String,
    pub environment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorData {
    pub code: u32,
    pub message: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceConnectionStatusData {
    /// enum - One of the voice connection states
    pub state: VoiceConnectionState,
    /// string - Hostname of the connected voice server
    pub hostname: String,
    /// array of integers - Last 20 pings (in ms)
    pub pings: Vec<u64>,
    /// integer - Average ping (in ms)
    pub average_ping: u64,
    /// integer - Last ping (in ms)
    pub last_ping: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VoiceConnectionState {
    Disconnected,
    AwaitingEndpoint,
    Authenticating,
    Connecting,
    Connected,
    VoiceDisconnected,
    VoiceConnecting,
    VoiceConnected,
    NoRoute,
    IceChecking,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpeakingData {
    /// string - Channel ID
    pub channel_id: String,
    /// string - User ID
    pub user_id: String,
}
