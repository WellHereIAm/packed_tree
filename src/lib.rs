#![forbid(missing_docs)]

//! `packed_tree` provides [Tree] struct and different coordinate systems used to index into it.

mod absolute_position;
mod error;
mod layer_position;
mod node;
mod tree;

mod direction;
mod layer_iter;

pub use absolute_position::{Depth, NodeIndex, NodePosition};
pub use error::CoordinateError;
pub use layer_position::{LayerIndex, LayerPosition};
pub use node::{Node, NodesRaw};
pub use tree::{implemented_tree_sizes, Tree, TreeInterface};
