use crate::wing::{WingChannelId, WingColor};

#[repr(transparent)]
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    specta::Type,
)]
pub struct ActorId(u32);

impl ActorId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl From<u32> for ActorId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for ActorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    name: String,
    channel: WingChannelId,
    color: Option<WingColor>,
}

impl Actor {
    pub fn new(channel: WingChannelId, name: String, color: Option<WingColor>) -> Self {
        Self {
            name,
            channel,
            color,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn channel(&self) -> &WingChannelId {
        &self.channel
    }

    pub fn color(&self) -> Option<WingColor> {
        self.color
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize, specta::Type, tauri_specta::Event)]
pub enum ActorEvent {
    Added(ActorId, Actor),
    Removed(ActorId),
}
