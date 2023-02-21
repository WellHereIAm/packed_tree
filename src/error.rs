use std::{error::Error, fmt::Display};

/// [Error] type for coordinate systems ([`NodeIndex`](crate::NodeIndex),
/// [`NodePostion`](crate::NodePosition), [`LayerIndex`](crate::LayerIndex),
/// [`LayerPosition`](crate::LayerPosition)) and [`Depth`](crate::Depth).
#[derive(Debug)]
pub enum CoordinateError {
    /// Marks an invalid `depth` provided.
    Depth,
    /// Marks an invalid `index` provided.
    Index,
    /// Marks an invalid `position` provided.
    Position,
}

impl Display for CoordinateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoordinateError::Depth => {
                write!(f, "CoordinateError: Invalid Depth")
            }
            CoordinateError::Index => {
                write!(f, "CoordinateError: Invalid Index")
            }
            CoordinateError::Position => {
                write!(f, "CoordinateError: Invalid Position")
            }
        }
    }
}

impl Error for CoordinateError {}

#[derive(Debug, Clone, Copy)]
pub enum TreeError {
    /// Returned when length of provided [`NodesRaw`](crate::NodesRaw)
    /// is greater than
    InvalidNodesLength,
}

impl Display for TreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeError::InvalidNodesLength => {
                write!(f, "TreeError: Invalid Nodes Length")
            }
        }
    }
}

impl Error for TreeError {}
