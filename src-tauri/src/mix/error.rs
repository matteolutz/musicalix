use crate::{
    mix::{ActorId, GroupId, PositionId},
    wing::error::WingError,
};

#[derive(Debug)]
pub enum MixError {
    WingError(WingError),

    ActorNotFound(ActorId),
    GroupNotFound(GroupId),
    PositionNotFound(PositionId),
}

impl std::fmt::Display for MixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WingError(err) => write!(f, "Wing error: {}", err),

            Self::ActorNotFound(id) => write!(f, "Actor not found: {}", id),
            Self::GroupNotFound(id) => write!(f, "Group not found: {}", id),
            Self::PositionNotFound(id) => write!(f, "Position not found: {}", id),
        }
    }
}

impl From<WingError> for MixError {
    fn from(value: WingError) -> Self {
        Self::WingError(value)
    }
}
