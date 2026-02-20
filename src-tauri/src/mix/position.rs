use crate::{mix::error::MixError, utils::ClampedValue, wing::WingChannel};

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
pub struct PositionId(u32);

impl PositionId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl From<u32> for PositionId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for PositionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// Panning. 0.0 = left, 0.5 = center, 1.0 = right
    pan: ClampedValue,
}

impl Default for Position {
    fn default() -> Self {
        Self { pan: 0.5.into() }
    }
}

impl Position {
    pub fn apply(&self, channel: &WingChannel) -> Result<(), MixError> {
        channel.set_pan(self.pan.as_f32())?;
        Ok(())
    }
}
