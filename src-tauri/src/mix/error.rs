use crate::{mix::ActorId, wing::error::WingError};

#[derive(Debug)]
pub enum MixError {
    WingError(WingError),

    ActorNotFound(ActorId),
}

impl std::fmt::Display for MixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WingError(err) => write!(f, "Wing error: {}", err),

            Self::ActorNotFound(id) => write!(f, "Actor not found: {}", id),
        }
    }
}

impl From<WingError> for MixError {
    fn from(value: WingError) -> Self {
        Self::WingError(value)
    }
}
