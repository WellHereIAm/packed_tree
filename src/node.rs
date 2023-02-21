use std::marker::PhantomData;

use crate::{NodeIndex, TreeInterface};

/// Data inside a [`Tree`](crate::Tree).
#[derive(Debug, Clone, PartialEq)]
pub enum Node<T> {
    /// Node which by combination rules became filled, i.e. it is expected that most of the children are filled as well.
    Filled(T),
    /// Node which children are not all empty, but this became empty by combination rules.
    Reduced,
    /// Marks node which all children are empty as well.
    Empty,
}

/// Helper struct to ease building [`Tree`] from data.
#[derive(Debug)]
pub struct NodesRaw<T, U> {
    nodes: Vec<Node<T>>,
    boo: PhantomData<U>,
}

/// Constructs [`NodesRaw`] from [`Vec`] of [`nodes`](Node),
/// if length of `nodes` is greater than associated [`tree`](crate::Tree),
/// then `nodes` beyond tree size are trimmed.
impl<T, U> From<Vec<Node<T>>> for NodesRaw<T, U>
where
    U: TreeInterface,
    T: Clone,
{
    fn from(mut value: Vec<Node<T>>) -> Self {
        if value.len() > U::SIZE {
            value = value[0..U::SIZE].to_vec();
        }

        Self {
            nodes: value,
            boo: PhantomData,
        }
    }
}

impl<T, U> From<NodesRaw<T, U>> for Vec<Node<T>>
where
    U: TreeInterface,
{
    fn from(value: NodesRaw<T, U>) -> Self {
        value.nodes
    }
}

impl<T, U> Default for NodesRaw<T, U> {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            boo: Default::default(),
        }
    }
}

impl<T, U> NodesRaw<T, U>
where
    U: TreeInterface,
{
    /// Creates a new empty [NodesRaw] struct.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a `node` to the back of a collection.
    pub fn push(&mut self, node: Node<T>) {
        debug_assert!(self.nodes.len() < U::SIZE);
        self.nodes.push(node)
    }

    /// Returns a reference to stored `nodes`.
    pub fn get(&self) -> &Vec<Node<T>> {
        &self.nodes
    }

    /// Returns `true` if [len](NodesRaw::len) is equal to [tree size](TreeParameters::SIZE).
    pub fn is_filled(&self) -> bool {
        self.nodes.len() == U::SIZE
    }

    /// Returns the number of `nodes` in the collection.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns `true` if number of `nodes` inside is equal to 0.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Sets the node on `position` to provided [`node`](Node)
    /// and returns a [`Node`] previously stored on  `position`.
    pub fn set(&mut self, index: NodeIndex<U>, mut value: Node<T>) -> Node<T> {
        debug_assert!(index < self.len());
        std::mem::swap(&mut self.nodes[index], &mut value);
        value
    }
}
