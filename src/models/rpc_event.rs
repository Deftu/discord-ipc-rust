use serde::{Deserialize, Serialize};

// TODO: move this to somewhere else
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "evt", content = "args")]
pub enum SentEvent {
    Ready,
    Error,
    GuildStatus,
    GuildCreate,
    ChannelCreate,
    VoiceChannelSelect,
    VoiceStateCreate,
    VoiceStateUpdate { channel_id: String },
    VoiceStateDelete,
    VoiceSettingsUpdate,
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
