use std::{fmt::Debug, ops::Range};

use crate::{LayerPosition, Node, NodeIndex, NodePosition, NodesRaw};

/// Stores data in **non**-sparse octree.
///
/// This storage type allows to use benefits of linear storage as is fast insert
/// and also provides advantages of spatial datastructure for cost of memory efficiency.
#[derive(Debug, Clone, PartialEq)]
pub struct Tree<T, const SIZE: usize> {
    /// Stored data are in [boxed](Box) `array` as for bigger data sets stack would be insufficient.
    ///
    /// Constant sized `array` allows for constant modification speed and also signifies that size of
    /// this data will not change.
    stored: Box<[Node<T>; SIZE]>,
}

impl<T, const SIZE: usize> Default for Tree<T, SIZE>
where
    T: Debug + Clone,
{
    fn default() -> Self {
        Self {
            // `unwrap` will never fail as size of `vec` is guaranteed to be `SIZE`.
            stored: vec![Node::Empty; SIZE].try_into().unwrap(),
        }
    }
}

/// Prefered method of constructing a new [Tree] from [`nodes`](NodesRaw).
///
/// [NodesRaw] `length` must never exceed `SIZE`, otherwise conversion panics.
impl<T, const SIZE: usize> From<NodesRaw<T, Self>> for Tree<T, SIZE>
where
    Self: TreeInterface,
    T: Debug + Clone,
{
    fn from(value: NodesRaw<T, Self>) -> Self {
        debug_assert!(value.len() <= Self::SIZE);
        let mut vec: Vec<Node<T>> = value.into();
        vec.extend(vec![Node::Empty; SIZE - vec.len()]);
        Self::from_nodes(vec.try_into().expect("Invalid length."))
    }
}

/// Amount of stored elements in [Tree] with biggest row size of 128.  
pub const TREE_128: usize = 128 * 128 * 128
    + 64 * 64 * 64
    + 32 * 32 * 32
    + 16 * 16 * 16
    + 8 * 8 * 8
    + 4 * 4 * 4
    + 2 * 2 * 2
    + 1;
/// Amount of stored elements in [Tree] with biggest row size of 64.  
pub const TREE_64: usize =
    64 * 64 * 64 + 32 * 32 * 32 + 16 * 16 * 16 + 8 * 8 * 8 + 4 * 4 * 4 + 2 * 2 * 2 + 1;
/// Amount of stored elements in [Tree] with biggest row size of 32.  
pub const TREE_32: usize = 32 * 32 * 32 + 16 * 16 * 16 + 8 * 8 * 8 + 4 * 4 * 4 + 2 * 2 * 2 + 1;
/// Amount of stored elements in [Tree] with biggest row size of 16.  
pub const TREE_16: usize = 16 * 16 * 16 + 8 * 8 * 8 + 4 * 4 * 4 + 2 * 2 * 2 + 1;
/// Amount of stored elements in [Tree] with biggest row size of 8.  
pub const TREE_8: usize = 8 * 8 * 8 + 4 * 4 * 4 + 2 * 2 * 2 + 1;
/// Amount of stored elements in [Tree] with biggest row size of 4.  
pub const TREE_4: usize = 4 * 4 * 4 + 2 * 2 * 2 + 1;
/// Amount of stored elements in [Tree] with biggest row size of 2.  
pub const TREE_2: usize = 2 * 2 * 2 + 1;
/// Amount of stored elements in [Tree] with biggest row size of 1.  
pub const TREE_1: usize = 1;

/// All [Tree] sizes for which are [TreeParameters] implemented.
pub mod implemented_tree_sizes {
    pub use super::{TREE_1, TREE_128, TREE_16, TREE_2, TREE_32, TREE_4, TREE_64, TREE_8};
}

impl<T> TreeInterface for Tree<T, TREE_128> {
    const SIZE: usize = TREE_128;
    const BIGGEST_ROW_SIZE: usize = 128;
    const DEPTH: usize = 8;

    #[inline(always)]
    fn rows_sizes() -> Vec<usize> {
        vec![128, 64, 32, 16, 8, 4, 2, 1]
    }

    #[inline(always)]
    fn layers_sizes() -> Vec<usize> {
        vec![2097152, 262144, 32768, 4096, 512, 64, 8, 1]
    }
}

impl<T> TreeInterface for Tree<T, TREE_64> {
    const SIZE: usize = TREE_64;
    const BIGGEST_ROW_SIZE: usize = 64;
    const DEPTH: usize = 7;

    #[inline(always)]
    fn rows_sizes() -> Vec<usize> {
        vec![64, 32, 16, 8, 4, 2, 1]
    }

    #[inline(always)]
    fn layers_sizes() -> Vec<usize> {
        vec![262144, 32768, 4096, 512, 64, 8, 1]
    }
}

impl<T> TreeInterface for Tree<T, TREE_32> {
    const SIZE: usize = TREE_32;
    const BIGGEST_ROW_SIZE: usize = 32;
    const DEPTH: usize = 6;

    #[inline(always)]
    fn rows_sizes() -> Vec<usize> {
        vec![32, 16, 8, 4, 2, 1]
    }

    #[inline(always)]
    fn layers_sizes() -> Vec<usize> {
        vec![32768, 4096, 512, 64, 8, 1]
    }
}

impl<T> TreeInterface for Tree<T, TREE_16> {
    const SIZE: usize = TREE_16;
    const BIGGEST_ROW_SIZE: usize = 16;
    const DEPTH: usize = 5;

    #[inline(always)]
    fn rows_sizes() -> Vec<usize> {
        vec![16, 8, 4, 2, 1]
    }

    #[inline(always)]
    fn layers_sizes() -> Vec<usize> {
        vec![4096, 512, 64, 8, 1]
    }
}

impl<T> TreeInterface for Tree<T, TREE_8> {
    const SIZE: usize = TREE_8;
    const BIGGEST_ROW_SIZE: usize = 8;
    const DEPTH: usize = 4;

    #[inline(always)]
    fn rows_sizes() -> Vec<usize> {
        vec![8, 4, 2, 1]
    }

    #[inline(always)]
    fn layers_sizes() -> Vec<usize> {
        vec![512, 64, 8, 1]
    }
}

impl<T> TreeInterface for Tree<T, TREE_4> {
    const SIZE: usize = TREE_4;
    const BIGGEST_ROW_SIZE: usize = 4;
    const DEPTH: usize = 3;

    #[inline(always)]
    fn rows_sizes() -> Vec<usize> {
        vec![4, 2, 1]
    }

    #[inline(always)]
    fn layers_sizes() -> Vec<usize> {
        vec![64, 8, 1]
    }
}

impl<T> TreeInterface for Tree<T, TREE_2> {
    const SIZE: usize = TREE_2;
    const BIGGEST_ROW_SIZE: usize = 2;
    const DEPTH: usize = 2;

    #[inline(always)]
    fn rows_sizes() -> Vec<usize> {
        vec![2, 1]
    }

    #[inline(always)]
    fn layers_sizes() -> Vec<usize> {
        vec![8, 1]
    }
}

impl<T> TreeInterface for Tree<T, TREE_1> {
    const SIZE: usize = TREE_1;
    const BIGGEST_ROW_SIZE: usize = 1;
    const DEPTH: usize = 1;

    #[inline(always)]
    fn rows_sizes() -> Vec<usize> {
        vec![1]
    }

    #[inline(always)]
    fn layers_sizes() -> Vec<usize> {
        vec![1]
    }
}

impl<T, const SIZE: usize> Tree<T, SIZE>
where
    Self: TreeInterface,
    T: Debug,
{
    /// Creates a new [`Tree`] with all [`nodes`](Node) set to [`Empty`](Node::Empty).
    pub fn new() -> Self
    where
        T: Clone + Debug,
    {
        Self::default()
    }

    /// Creates a new [`Tree`] from provided `nodes` without any modification to is.
    ///
    /// When createting a new [`Tree`] from existing nodes use of [`TryFrom<NodesRaw>`]
    /// is prefered as it provides more convinient usage.
    pub fn from_nodes(nodes: Box<[Node<T>; SIZE]>) -> Self {
        Self { stored: nodes }
    }

    /// Builds [`Tree`] from bottom up, determining [`Node`] state of each node by taking its
    /// children if present and appling `combine_rule`
    pub fn build<F>(&mut self, combine_rule: F)
    where
        F: FnOnce(&[&Node<T>]) -> Node<T> + Copy,
    {
        // This replaces four nested for loops.
        let iter = Self::rows_sizes()
            .into_iter()
            .enumerate()
            .flat_map(|(depth, row_size)| {
                (0..row_size).flat_map(move |z| {
                    (0..row_size).flat_map(move |y| {
                        (0..row_size).map(move |x| LayerPosition::new(x, y, z, depth))
                    })
                })
            });

        for position in iter {
            if let Some(children) = self.children(position) {
                let children_data = children
                    .into_iter()
                    .map(|index| self.get(index))
                    .collect::<Vec<&Node<T>>>();
                self.set(position, combine_rule(&children_data));
            }
        }
    }

    /// Returns a reference to an [Node] on `position`.
    ///
    /// [NodeIndex] is expected to be always valid.
    pub fn get<P>(&self, position: P) -> &Node<T>
    where
        P: Into<NodeIndex<Self>>,
    {
        let index: NodeIndex<Self> = position.into();
        &self.stored[index]
    }

    /// Returns a mutable reference to an [Node] on `position`.
    ///
    /// [NodeIndex] is expected to be always valid.
    pub fn get_mut<P>(&mut self, position: P) -> &mut Node<T>
    where
        P: Into<NodeIndex<Self>>,
    {
        let index = position.into();
        &mut self.stored[index]
    }

    /// Returns an [`index`](NodeIndex) of parrent of [`Node`] on `position`
    /// if such node has a parrent, i.e. does not have `depth` equal to [TreeParameters::MAX_DEPTH_INDEX],
    /// in that case [`None`] is returned.
    pub fn parrent<P>(&self, position: P) -> Option<NodeIndex<Self>>
    where
        P: Into<NodeIndex<Self>>,
    {
        let index: NodeIndex<Self> = position.into();
        Some(LayerPosition::from(index).parrent_position()?.into())
    }

    /// Returns an [`indexes`](NodeIndex) of children of [`Node`] on `position`
    /// if such node has a children, i.e. does not have `depth` equal to zero,
    /// in which case [`None`] is returned.
    ///
    /// Returned indexes are ordered from front to back first, then bottom to top
    /// and lastly from left to right, i.e. if first child index is (0, 0, 0) and row size is 4
    /// the children positions will be following series:
    ///
    /// `(0, 0, 0)`, `(1, 0, 0)`, `(0, 1, 0)`, `(1, 1, 0)`, `(0, 0, 1)`, `(1, 0, 1)`, `(0, 1, 1)`, `(1, 1, 1)`
    ///
    // TODO: Maybe replace [NodeIndex; 8] with some datastructure.
    pub fn children<P>(&self, position: P) -> Option<[NodeIndex<Self>; 8]>
    where
        P: Into<NodeIndex<Self>>,
    {
        let parrent_index: NodeIndex<Self> = position.into();
        // Position of an child in bottom front left corner of parrent node.
        let children_anchor: NodeIndex<Self> =
            NodePosition::from(parrent_index).child_position()?.into();
        // Row size of childrens layer.
        let row_size = Self::row_size(children_anchor.depth());

        let children: [NodeIndex<Self>; 8] = (0..2)
            .flat_map(|z| {
                (0..2).flat_map(move |y| {
                    (0..2).map(move |x| {
                        children_anchor + (x + (y * row_size) + (z * row_size * row_size))
                    })
                })
            })
            .collect::<Vec<NodeIndex<Self>>>()
            .try_into()
            .unwrap(); // Iterator bounds ensure that no panic will occur on this unwrap.

        Some(children)
    }

    /// Sets the node on `position` to provided [`node`](Node)
    /// and returns a [`Node`] previously stored on `position`.
    pub fn set<P>(&mut self, position: P, node: Node<T>) -> Node<T>
    where
        P: Into<NodeIndex<Self>>,
    {
        let mut node = node;
        let index = position.into();
        std::mem::swap(&mut self.stored[index], &mut node);
        node
    }
}

// TODO: find better name? Already changed from config and better documentation
/// Common tree parameters.
pub trait TreeInterface {
    /// [Tree] size, i.e. amount of elements that that tree will hold.
    const SIZE: usize;
    /// Size of the biggest row of tree.
    const BIGGEST_ROW_SIZE: usize;
    /// Amount of elements it the shallowest tree layer.
    const CHUNK_SIZE: usize =
        Self::BIGGEST_ROW_SIZE * Self::BIGGEST_ROW_SIZE * Self::BIGGEST_ROW_SIZE;
    /// Amount of elements it the shallowest tree layer.
    const SHALLOWEST_LAYER_SIZE: usize = Self::CHUNK_SIZE;
    /// Amount of layers tree has.
    const DEPTH: usize = tree_depth(Self::BIGGEST_ROW_SIZE);
    /// Index of deepest layer.
    const MAX_DEPTH_INDEX: usize = Self::DEPTH - 1;

    /// Returns a row_size in specified `depth`.
    ///
    /// Expects in-bounds `depth`.
    #[inline(always)]
    fn row_size(depth: usize) -> usize {
        debug_assert!(depth <= Self::MAX_DEPTH_INDEX);
        Self::rows_sizes()[depth]
    }

    /// Returns row sizes of tree, from the shallowest to the deepest.
    ///
    /// When implementing a for a new tree size it is better to implement this manually.
    // TODO change this to a `const ROWS_SIZES` once `[usize; Self::DEPTH]` is allowed.
    fn rows_sizes() -> Vec<usize> {
        let mut sizes = Vec::new();
        let mut row_size = Self::BIGGEST_ROW_SIZE;
        while row_size / 2 != 0 {
            sizes.push(row_size);
            row_size /= 2;
        }
        // Change last row size to be 1 instead of 0.
        sizes.push(1);
        sizes
    }

    /// Returns layers sizes of tree, i.e. amount of elements in each layer,
    /// from the shallowest to the deepest.
    ///
    /// When implementing a for a new tree size it is better to implement this manually.
    fn layers_sizes() -> Vec<usize> {
        let mut sizes = Vec::new();
        for row_size in Self::rows_sizes() {
            sizes.push(row_size * row_size * row_size);
        }
        sizes
    }
}

/// Calculates depth of tree from `row_size`.
const fn tree_depth(row_size: usize) -> usize {
    let mut depth = 0;
    let mut row_size = row_size;
    while row_size / 2 != 0 {
        depth += 1;
        row_size /= 2;
    }
    depth
}

#[cfg(test)]
mod tree_tests {

    use crate::{Node, NodeIndex, NodesRaw};

    use super::Tree;

    type TestTree = Tree<usize, 73>;

    fn nodes_raw(size: usize) -> NodesRaw<usize, TestTree> {
        let mut vec = Vec::new();
        for i in 0..size {
            vec.push(Node::Filled(i))
        }
        NodesRaw::from(vec)
    }

    #[test]
    fn new() {
        TestTree::new();
    }

    #[test]
    fn from_nodes_raw() {
        let nodes = nodes_raw(0);
        drop(TestTree::from(nodes));
        let nodes = nodes_raw(1);
        drop(TestTree::from(nodes));
        let nodes = nodes_raw(64);
        drop(TestTree::from(nodes));
        let nodes = nodes_raw(73);
        drop(TestTree::from(nodes));

        std::panic::catch_unwind(|| TestTree::from(NodesRaw::from(vec![Node::Empty; 74])))
            .unwrap_err();
    }

    #[test]
    fn children() {
        let nodes = nodes_raw(73);
        let tree = TestTree::from(nodes);
        assert_eq!(tree.children(NodeIndex::new(0)), None);
        assert_eq!(tree.children(NodeIndex::new(63)), None);

        assert_eq!(
            tree.children(NodeIndex::new(72)),
            Some([
                NodeIndex::new(64),
                NodeIndex::new(65),
                NodeIndex::new(66),
                NodeIndex::new(67),
                NodeIndex::new(68),
                NodeIndex::new(69),
                NodeIndex::new(70),
                NodeIndex::new(71),
            ])
        );

        assert_eq!(
            tree.children(NodeIndex::new(71)),
            Some([
                NodeIndex::new(42),
                NodeIndex::new(43),
                NodeIndex::new(46),
                NodeIndex::new(47),
                NodeIndex::new(58),
                NodeIndex::new(59),
                NodeIndex::new(62),
                NodeIndex::new(63),
            ])
        );

        assert_eq!(
            tree.children(NodeIndex::new(64)),
            Some([
                NodeIndex::new(0),
                NodeIndex::new(1),
                NodeIndex::new(4),
                NodeIndex::new(5),
                NodeIndex::new(16),
                NodeIndex::new(17),
                NodeIndex::new(20),
                NodeIndex::new(21),
            ])
        );

        assert_eq!(
            tree.children(NodeIndex::new(65)),
            Some([
                NodeIndex::new(2),
                NodeIndex::new(3),
                NodeIndex::new(6),
                NodeIndex::new(7),
                NodeIndex::new(18),
                NodeIndex::new(19),
                NodeIndex::new(22),
                NodeIndex::new(23),
            ])
        );
    }

    #[test]
    fn parrent() {
        let nodes = nodes_raw(73);
        let tree = TestTree::from(nodes);

        assert_eq!(tree.parrent(NodeIndex::new(0)), Some(NodeIndex::new(64)));
        assert_eq!(tree.parrent(NodeIndex::new(1)), Some(NodeIndex::new(64)));
        assert_eq!(tree.parrent(NodeIndex::new(2)), Some(NodeIndex::new(65)));
        assert_eq!(tree.parrent(NodeIndex::new(63)), Some(NodeIndex::new(71)));
        assert_eq!(tree.parrent(NodeIndex::new(64)), Some(NodeIndex::new(72)));
        assert_eq!(tree.parrent(NodeIndex::new(65)), Some(NodeIndex::new(72)));
        assert_eq!(tree.parrent(NodeIndex::new(71)), Some(NodeIndex::new(72)));
        assert_eq!(tree.parrent(NodeIndex::new(72)), None);
    }

    #[test]
    fn get() {
        let nodes = nodes_raw(64);
        let mut tree = TestTree::from(nodes);
        tree.set(NodeIndex::new(64), Node::Filled(64));

        assert_eq!(tree.get(NodeIndex::new(0)), &Node::Filled(0));
        assert_eq!(tree.get(NodeIndex::new(64)), &Node::Filled(64));
    }

    #[test]
    fn build() {
        let mut nodes = nodes_raw(64);
        nodes.set(NodeIndex::new(0), Node::Empty);
        nodes.set(NodeIndex::new(1), Node::Empty);
        nodes.set(NodeIndex::new(4), Node::Empty);
        nodes.set(NodeIndex::new(5), Node::Empty);
        nodes.set(NodeIndex::new(16), Node::Empty);
        nodes.set(NodeIndex::new(17), Node::Empty);
        nodes.set(NodeIndex::new(20), Node::Empty);
        nodes.set(NodeIndex::new(21), Node::Empty);

        let mut tree = TestTree::from(nodes);
        tree.build(|nodes| {
            let mut empty_count = 0;
            for node in nodes {
                match node {
                    Node::Filled(_) => {}
                    Node::Reduced | Node::Empty => empty_count += 1,
                }
            }

            if empty_count == 8 {
                return Node::Empty;
            } else if empty_count > 0 {
                return Node::Reduced;
            }
            Node::Filled(9999)
        });
        let test_nodes = nodes_raw(64);
        let mut test_tree = TestTree::from(test_nodes);
        test_tree.set(NodeIndex::new(0), Node::Empty);
        test_tree.set(NodeIndex::new(1), Node::Empty);
        test_tree.set(NodeIndex::new(4), Node::Empty);
        test_tree.set(NodeIndex::new(5), Node::Empty);
        test_tree.set(NodeIndex::new(16), Node::Empty);
        test_tree.set(NodeIndex::new(17), Node::Empty);
        test_tree.set(NodeIndex::new(20), Node::Empty);
        test_tree.set(NodeIndex::new(21), Node::Empty);
        test_tree.set(NodeIndex::new(64), Node::Empty);
        test_tree.set(NodeIndex::new(65), Node::Filled(9999));
        test_tree.set(NodeIndex::new(66), Node::Filled(9999));
        test_tree.set(NodeIndex::new(67), Node::Filled(9999));
        test_tree.set(NodeIndex::new(68), Node::Filled(9999));
        test_tree.set(NodeIndex::new(69), Node::Filled(9999));
        test_tree.set(NodeIndex::new(70), Node::Filled(9999));
        test_tree.set(NodeIndex::new(71), Node::Filled(9999));
        test_tree.set(NodeIndex::new(72), Node::Reduced);
        assert_eq!(tree, test_tree);
    }
}

#[cfg(test)]
mod tree_interface_tests {
    use crate::{
        implemented_tree_sizes::{
            TREE_1, TREE_128, TREE_16, TREE_2, TREE_32, TREE_4, TREE_64, TREE_8,
        },
        NodeIndex, Tree, TreeInterface,
    };
}
