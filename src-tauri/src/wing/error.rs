use std::ops::RangeInclusive;

#[derive(Debug)]
pub enum WingError {
    IdOutOfBounds(u32, RangeInclusive<u32>),
    LibWingError(libwing::Error),
    NodeDataRequestTimeout(i32),
}

impl WingError {
    pub fn id_out_of_bounds(id: impl Into<u32>, from: impl Into<u32>, to: impl Into<u32>) -> Self {
        Self::IdOutOfBounds(id.into(), (from.into())..=(to.into()))
    }
}

impl std::fmt::Display for WingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdOutOfBounds(id, bounds) => {
                write!(
                    f,
                    "Wing ID {} is out of bounds. Allowed values from {} to {}",
                    id,
                    bounds.start(),
                    bounds.end()
                )
            }
            Self::NodeDataRequestTimeout(node_id) => {
                write!(f, "Node data request timed out for node ID {}", node_id)
            }
            Self::LibWingError(error) => write!(f, "libwing error: {}", error),
        }
    }
}

impl From<libwing::Error> for WingError {
    fn from(value: libwing::Error) -> Self {
        Self::LibWingError(value)
    }
}
