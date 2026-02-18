use crate::{
    mix::{error::MixError, MixConfig},
    utils::ClampedValue,
    wing::Wing,
};

mod dca;
pub use dca::*;

mod position;
pub use position::*;

mod list;
pub use list::*;

#[derive(Clone)]
pub struct CueExecutionContext<'a> {
    pub config: &'a MixConfig,
    pub wing: &'a Wing,
}

#[derive(
    Copy, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, specta::Type,
)]
pub struct CueId {
    major: u32,
    minor: u32,
}

impl CueId {
    pub fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    pub fn next(&self) -> Self {
        Self {
            major: self.major + 1,
            minor: self.minor,
        }
    }
}

impl std::fmt::Display for CueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Cue {
    pub id: CueId,
    name: String,

    /// Fade time in seconds (>= 0)
    fade_time: f32,
    /// Snap percentage (0..=1.0) (i.e. when non-fade parameters are assigned -  DCAs, ...)
    snap: ClampedValue,

    dca: DcaAssignment,

    position: PositionAssignment,
}

impl Cue {
    pub fn new(id: CueId, name: String) -> Self {
        Self {
            id,
            name,
            fade_time: 0.0,
            snap: 0.0.into(),
            dca: DcaAssignment::default(),
            position: PositionAssignment::default(),
        }
    }

    pub async fn activate<'a>(&self, context: CueExecutionContext<'a>) -> Result<(), MixError> {
        if self.fade_time > 0.0 {
            // TODO: in fade
        }

        self.snap(&context).await?;

        Ok(())
    }

    async fn snap<'a>(&self, context: &CueExecutionContext<'a>) -> Result<(), MixError> {
        self.dca.apply(context).await?;
        self.position.apply(context)?;
        Ok(())
    }
}
