use std::fmt::Display;
use std::marker::PhantomData;

use crate::{NodeIndex, NodePosition, TreeParameters};

/// Index of [`Node`](crate::Node) in specific layer.
///
/// Compared to [`NodeIndex`] this starts from 0 in each layer.
///
/// This structure always expects to have valid data inside
/// and in debug panics if that is not true.
#[derive(Debug)]
pub struct LayerIndex<T> {
    /// In-layer index.
    index: usize,
    /// Layer in [`Tree`](crate::Tree).
    ///
    /// The shallowest layer is the biggest in size and size of the deepest is 1.
    depth: usize,
    /// Associated [`Tree`](crate::Tree).
    boo: PhantomData<T>,
}

/// [`Clone`] is implemented manually, so there is no requirement on `T` to also implement [`Clone`].
impl<T> Clone for LayerIndex<T> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            depth: self.depth,
            boo: PhantomData,
        }
    }
}

/// [`Display`] shows the biggest row of associated [`Tree`](crate::Tree), `index` and `depth`.
impl<T> Display for LayerIndex<T>
where
    T: TreeParameters,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeIndex::<{}>{{ index: {}, depth: {} }}",
            T::BIGGEST_ROW_SIZE,
            self.index,
            self.depth
        )
    }
}

/// [`Copy`] is implemented manually, so there is no requirement on `T` to also implement [`Clone`].
impl<T> Copy for LayerIndex<T> {}

/// [`PartialEq`] is implemented manually, so there is no requirement on `T` to also implement [`PartialEq`].
impl<T> PartialEq for LayerIndex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.depth == other.depth
    }
}

impl<T> From<NodeIndex<T>> for LayerIndex<T>
where
    T: TreeParameters,
{
    fn from(value: NodeIndex<T>) -> Self {
        LayerPosition::from(value).into()
    }
}

impl<T> From<NodePosition<T>> for LayerIndex<T>
where
    T: TreeParameters,
{
    fn from(value: NodePosition<T>) -> Self {
        LayerPosition::from(value).into()
    }
}

impl<T> From<LayerPosition<T>> for LayerIndex<T>
where
    T: TreeParameters,
{
    fn from(value: LayerPosition<T>) -> Self {
        let row_size = T::row_size(value.depth);
        let index = value.x + (value.y * row_size) + (value.z * row_size * row_size);
        Self::new(index, value.depth)
    }
}

impl<T> LayerIndex<T>
where
    T: TreeParameters,
{
    /// Creates a new [LayerIndex].
    ///
    /// Validity of provided `index` and `depth` is checked only in debug mode.
    pub fn new(index: usize, depth: usize) -> Self {
        debug_assert!(Self::is_valid_index_depth(index, depth));
        Self {
            index,
            depth,
            boo: PhantomData,
        }
    }

    /// Returns `true` if an `depth` is less than [MAX_DEPTH_INDEX](TreeParameters::MAX_DEPTH_INDEX)
    /// of an associated [`Tree`](crate::Tree)
    /// and `index` is less than .
    pub fn is_valid_index_depth(index: usize, depth: usize) -> bool {
        depth <= T::MAX_DEPTH_INDEX && index < T::layers_sizes()[depth]
    }

    /// Returns `true` if call to [is_valid_index_depth](LayerIndex::is_valid_index_depth)
    /// on inner values returns `true`.
    pub fn is_valid(self) -> bool {
        Self::is_valid_index_depth(self.index, self.depth)
    }

    /// Returns `depth`.
    pub fn depth(self) -> usize {
        self.depth
    }

    /// Returns a tuple containing `index` and `depth` in this order.
    pub fn get_raw(self) -> (usize, usize) {
        (self.index, self.depth)
    }
}

/// Position of [`Node`](crate::Node) in specific layer.
///
/// Compared to [`NodePosition`] this takes into account row size of specific layer,
/// i.e. deeper the layer, the less nodes are in it and the smaller the position is.
#[derive(Debug)]
pub struct LayerPosition<T> {
    /// Amount of nodes from an tree origin on `x` asix in layer.
    pub x: usize,
    /// Amount of nodes from an tree origin on `y` asix in layer.
    pub y: usize,
    /// Amount of nodes from an tree origin on `z` asix in layer.
    pub z: usize,
    /// Layer in [`Tree`](crate::Tree).
    ///
    /// The shallowest layer is the biggest in size and size of the deepest is 1.
    pub depth: usize,
    boo: PhantomData<T>,
}

/// [`Clone`] is implemented manually, so there is no requirement on `T` to also implement [`Clone`].
impl<T> Clone for LayerPosition<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
            depth: self.depth,
            boo: PhantomData,
        }
    }
}

/// [`Copy`] is implemented manually, so there is no requirement on `T` to also implement [`Clone`].
impl<T> Copy for LayerPosition<T> {}

/// [`PartialEq`] is implemented manually, so there is no requirement on `T` to also implement [`PartialEq`].
impl<T> PartialEq for LayerPosition<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.depth == other.depth
    }
}

/// [`Display`] shows the biggest row of associated [`Tree`](crate::Tree), `position` and `depth`.
impl<T> Display for LayerPosition<T>
where
    T: TreeParameters,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LayerPosition::<{}>( x: {}, y: {}, z: {}, depth: {} )",
            T::BIGGEST_ROW_SIZE,
            self.x,
            self.y,
            self.z,
            self.depth
        )
    }
}

impl<T> From<NodeIndex<T>> for LayerPosition<T>
where
    T: TreeParameters,
{
    fn from(value: NodeIndex<T>) -> Self {
        let depth = value.depth();
        let rows_sizes = T::rows_sizes();
        let remainder: usize = rows_sizes[0..depth]
            .iter()
            .map(|row_size| row_size * row_size * row_size)
            .sum();
        let layer_index = LayerIndex::new(value.raw() - remainder, depth);
        layer_index.into()
    }
}

impl<T> From<NodePosition<T>> for LayerPosition<T>
where
    T: TreeParameters,
{
    fn from(value: NodePosition<T>) -> Self {
        let row_size = T::row_size(value.depth);
        let divisor = T::BIGGEST_ROW_SIZE / row_size;
        let x = value.x / divisor;
        let y = value.y / divisor;
        let z = value.z / divisor;

        LayerPosition::new(x, y, z, value.depth)
    }
}

impl<T> From<LayerIndex<T>> for LayerPosition<T>
where
    T: TreeParameters,
{
    fn from(value: LayerIndex<T>) -> Self {
        let row_size = T::row_size(value.depth);

        let z = value.index / (row_size * row_size);
        let index = value.index - (z * row_size * row_size);
        let y = index / row_size;
        let x = index % row_size;

        LayerPosition::new(x, y, z, value.depth)
    }
}

impl<T> LayerPosition<T>
where
    T: TreeParameters,
{
    /// Creates a new [LayerPosition].
    ///
    /// Validity of provided `position` and `depth` is checked only in debug mode.
    pub fn new(x: usize, y: usize, z: usize, depth: usize) -> Self {
        debug_assert!(Self::is_valid_position(x, y, z, depth));
        Self {
            x,
            y,
            z,
            depth,
            boo: PhantomData,
        }
    }

    /// Returns `true` if `x`, `y` and `z` are less than row size of specific layer
    /// and `depth` is less or equal to [MAX_DEPTH_INDEX](TreeParameters::MAX_DEPTH_INDEX).
    pub fn is_valid_position(x: usize, y: usize, z: usize, depth: usize) -> bool {
        let row_size = T::row_size(depth);

        x < row_size && y < row_size && z < row_size && depth <= T::MAX_DEPTH_INDEX
    }

    /// Returns `true` if call to [LayerPosition::is_valid_position] on inner values
    /// is evaluated to `true`.
    pub fn is_valid(self) -> bool {
        Self::is_valid_position(self.x, self.y, self.z, self.depth)
    }

    /// Returns a tuple of `x`, `y`, `z` and `depth` in this order.
    pub fn get_raw(self) -> (usize, usize, usize, usize) {
        (self.x, self.y, self.z, self.depth)
    }

    /// Returns [NodePosition] of parrent of this position if exists,
    /// otherwise [`None`] is returned.
    pub fn parrent_position(mut self) -> Option<Self> {
        if self.depth >= T::MAX_DEPTH_INDEX {
            return None;
        }

        self.depth += 1;
        // TODO: Remove this special case and replace with general solution.
        if self.depth == T::MAX_DEPTH_INDEX {
            return Some(Self::new(0, 0, 0, self.depth));
        }
        let row_size = T::row_size(self.depth);

        self.x /= row_size;
        self.y /= row_size;
        self.z /= row_size;

        Some(self)
    }
}

#[cfg(test)]
mod layer_index_tests {
    use crate::{LayerIndex, LayerPosition, NodeIndex, NodePosition, Tree};

    type TestTree = Tree<usize, 73>;
    type TestNodeIndex = NodeIndex<TestTree>;
    type TestNodePosition = NodePosition<TestTree>;
    type TestLayerPosition = LayerPosition<TestTree>;
    type TestLayerIndex = LayerIndex<TestTree>;

    #[test]
    fn new() {
        TestLayerIndex::new(0, 0);
        TestLayerIndex::new(1, 0);
        TestLayerIndex::new(63, 0);
        TestLayerIndex::new(0, 1);
        TestLayerIndex::new(1, 1);
        TestLayerIndex::new(7, 1);
        TestLayerIndex::new(0, 2);

        std::panic::catch_unwind(|| TestLayerIndex::new(64, 0)).unwrap_err();
        std::panic::catch_unwind(|| TestLayerIndex::new(8, 1)).unwrap_err();
        std::panic::catch_unwind(|| TestLayerIndex::new(1, 2)).unwrap_err();
    }

    #[test]
    fn from_node_index() {
        let index = TestNodeIndex::new(0);
        assert_eq!(TestLayerIndex::from(index), TestLayerIndex::new(0, 0));

        let index = TestNodeIndex::new(1);
        assert_eq!(TestLayerIndex::from(index), TestLayerIndex::new(1, 0));

        let index = TestNodeIndex::new(63);
        assert_eq!(TestLayerIndex::from(index), TestLayerIndex::new(63, 0));

        let index = TestNodeIndex::new(64);
        assert_eq!(TestLayerIndex::from(index), TestLayerIndex::new(0, 1));

        let index = TestNodeIndex::new(65);
        assert_eq!(TestLayerIndex::from(index), TestLayerIndex::new(1, 1));

        let index = TestNodeIndex::new(71);
        assert_eq!(TestLayerIndex::from(index), TestLayerIndex::new(7, 1));

        let index = TestNodeIndex::new(72);
        assert_eq!(TestLayerIndex::from(index), TestLayerIndex::new(0, 2));
    }

    #[test]
    fn from_node_position() {
        let position = TestNodePosition::new(0, 0, 0, 0);
        assert_eq!(TestLayerIndex::from(position), TestLayerIndex::new(0, 0));

        let position = TestNodePosition::new(1, 0, 0, 0);
        assert_eq!(TestLayerIndex::from(position), TestLayerIndex::new(1, 0));

        let position = TestNodePosition::new(3, 3, 3, 0);
        assert_eq!(TestLayerIndex::from(position), TestLayerIndex::new(63, 0));

        let position = TestNodePosition::new(0, 0, 0, 1);
        assert_eq!(TestLayerIndex::from(position), TestLayerIndex::new(0, 1));

        let position = TestNodePosition::new(2, 0, 0, 1);
        assert_eq!(TestLayerIndex::from(position), TestLayerIndex::new(1, 1));

        let position = TestNodePosition::new(2, 2, 2, 1);
        assert_eq!(TestLayerIndex::from(position), TestLayerIndex::new(7, 1));

        let position = TestNodePosition::new(0, 0, 0, 2);
        assert_eq!(TestLayerIndex::from(position), TestLayerIndex::new(0, 2));
    }

    #[test]
    fn from_layer_position() {
        let pos = TestLayerPosition::new(0, 0, 0, 0);
        assert_eq!(TestLayerIndex::from(pos), TestLayerIndex::new(0, 0));

        let pos = TestLayerPosition::new(1, 0, 0, 0);
        assert_eq!(TestLayerIndex::from(pos), TestLayerIndex::new(1, 0));

        let pos = TestLayerPosition::new(3, 3, 3, 0);
        assert_eq!(TestLayerIndex::from(pos), TestLayerIndex::new(63, 0));

        let pos = TestLayerPosition::new(0, 0, 0, 1);
        assert_eq!(TestLayerIndex::from(pos), TestLayerIndex::new(0, 1));

        let pos = TestLayerPosition::new(1, 0, 0, 1);
        assert_eq!(TestLayerIndex::from(pos), TestLayerIndex::new(1, 1));

        let pos = TestLayerPosition::new(1, 1, 1, 1);
        assert_eq!(TestLayerIndex::from(pos), TestLayerIndex::new(7, 1));

        let pos = TestLayerPosition::new(0, 0, 0, 2);
        assert_eq!(TestLayerIndex::from(pos), TestLayerIndex::new(0, 2));
    }
}

#[cfg(test)]
mod layer_position_tests {
    use crate::{LayerPosition, NodeIndex, NodePosition, Tree};

    type TestTree = Tree<usize, 73>;
    type TestNodeIndex = NodeIndex<TestTree>;
    type TestNodePosition = NodePosition<TestTree>;
    type TestLayerPosition = LayerPosition<TestTree>;

    #[test]
    fn new() {
        TestLayerPosition::new(0, 0, 0, 0);
        TestLayerPosition::new(1, 0, 0, 0);
        TestLayerPosition::new(3, 3, 3, 0);

        TestLayerPosition::new(0, 0, 0, 1);
        TestLayerPosition::new(1, 0, 0, 1);
        TestLayerPosition::new(1, 1, 1, 1);

        TestLayerPosition::new(0, 0, 0, 2);

        std::panic::catch_unwind(|| TestLayerPosition::new(4, 0, 0, 0)).unwrap_err();
        std::panic::catch_unwind(|| TestLayerPosition::new(2, 0, 0, 1)).unwrap_err();
        std::panic::catch_unwind(|| TestLayerPosition::new(2, 0, 0, 2)).unwrap_err();
    }

    #[test]
    fn parrent_position() {
        let pos = TestLayerPosition::new(0, 0, 0, 0);
        assert_eq!(
            pos.parrent_position(),
            Some(TestLayerPosition::new(0, 0, 0, 1))
        );

        let pos = TestLayerPosition::new(1, 0, 0, 0);
        assert_eq!(
            pos.parrent_position(),
            Some(TestLayerPosition::new(0, 0, 0, 1))
        );

        let pos = TestLayerPosition::new(2, 0, 0, 0);
        assert_eq!(
            pos.parrent_position(),
            Some(TestLayerPosition::new(1, 0, 0, 1))
        );

        let pos = TestLayerPosition::new(3, 3, 3, 0);
        assert_eq!(
            pos.parrent_position(),
            Some(TestLayerPosition::new(1, 1, 1, 1))
        );

        let pos = TestLayerPosition::new(0, 0, 0, 1);
        assert_eq!(
            pos.parrent_position(),
            Some(TestLayerPosition::new(0, 0, 0, 2))
        );

        let pos = TestLayerPosition::new(1, 0, 0, 1);
        assert_eq!(
            pos.parrent_position(),
            Some(TestLayerPosition::new(0, 0, 0, 2))
        );

        let pos = TestLayerPosition::new(1, 1, 1, 1);
        assert_eq!(
            pos.parrent_position(),
            Some(TestLayerPosition::new(0, 0, 0, 2))
        );

        let pos = TestLayerPosition::new(0, 0, 0, 2);
        assert_eq!(pos.parrent_position(), None);
    }

    #[test]
    fn from_node_index() {
        let index = TestNodeIndex::new(0);
        assert_eq!(
            TestLayerPosition::from(index),
            TestLayerPosition::new(0, 0, 0, 0)
        );

        let index = TestNodeIndex::new(1);
        assert_eq!(
            TestLayerPosition::from(index),
            TestLayerPosition::new(1, 0, 0, 0)
        );

        let index = TestNodeIndex::new(63);
        assert_eq!(
            TestLayerPosition::from(index),
            TestLayerPosition::new(3, 3, 3, 0)
        );

        let index = TestNodeIndex::new(64);
        assert_eq!(
            TestLayerPosition::from(index),
            TestLayerPosition::new(0, 0, 0, 1)
        );

        let index = TestNodeIndex::new(65);
        assert_eq!(
            TestLayerPosition::from(index),
            TestLayerPosition::new(1, 0, 0, 1)
        );

        let index = TestNodeIndex::new(71);
        assert_eq!(
            TestLayerPosition::from(index),
            TestLayerPosition::new(1, 1, 1, 1)
        );

        let index = TestNodeIndex::new(72);
        assert_eq!(
            TestLayerPosition::from(index),
            TestLayerPosition::new(0, 0, 0, 2)
        );
    }

    #[test]
    fn from_node_position() {
        let pos = TestNodePosition::new(0, 0, 0, 0);
        assert_eq!(
            TestLayerPosition::from(pos),
            TestLayerPosition::new(0, 0, 0, 0)
        );

        let pos = TestNodePosition::new(1, 0, 0, 0);
        assert_eq!(
            TestLayerPosition::from(pos),
            TestLayerPosition::new(1, 0, 0, 0)
        );

        let pos = TestNodePosition::new(3, 3, 3, 0);
        assert_eq!(
            TestLayerPosition::from(pos),
            TestLayerPosition::new(3, 3, 3, 0)
        );

        let pos = TestNodePosition::new(0, 0, 0, 1);
        assert_eq!(
            TestLayerPosition::from(pos),
            TestLayerPosition::new(0, 0, 0, 1)
        );

        let pos = TestNodePosition::new(2, 0, 0, 1);
        assert_eq!(
            TestLayerPosition::from(pos),
            TestLayerPosition::new(1, 0, 0, 1)
        );

        let pos = TestNodePosition::new(2, 2, 2, 1);
        assert_eq!(
            TestLayerPosition::from(pos),
            TestLayerPosition::new(1, 1, 1, 1)
        );

        let pos = TestNodePosition::new(0, 0, 0, 2);
        assert_eq!(
            TestLayerPosition::from(pos),
            TestLayerPosition::new(0, 0, 0, 2)
        );
    }
}
