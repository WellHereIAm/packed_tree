use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Add, Index, IndexMut, Range, Sub};

use crate::{LayerIndex, LayerPosition, TreeInterface};

/// Absolute index of [`Node`](crate::Node) inside a [`Tree`](crate::Tree).
///
/// This structure always expects to have valid data inside and in debug panics if that is not true.
#[derive(Debug)]
pub struct NodeIndex<T> {
    index: usize,
    /// Associated [`Tree`](crate::Tree).
    boo: PhantomData<T>,
}

/// [`Clone`] is implemented manually, so there is no requirement on `T` to also implement [`Clone`].
impl<T> Clone for NodeIndex<T> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            boo: PhantomData,
        }
    }
}

/// [`Copy`] is implemented manually, so there is no requirement on `T` to also implement [`Clone`].
impl<T> Copy for NodeIndex<T> {}

/// [`PartialEq`] is implemented manually, so there is no requirement on `T` to also implement [`PartialEq`].
impl<T> PartialEq for NodeIndex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

/// [`PartialEq`] is implemented manually, so there is no requirement on `T` to also implement [`PartialEq`]
/// and comparison to [`usize`] is possible.
impl<T> PartialEq<usize> for NodeIndex<T> {
    fn eq(&self, other: &usize) -> bool {
        self.index == *other
    }
}

/// [`PartialOrd`] is implemented manually, so there is no requirement on `T` to also implement [`PartialOrd`].
impl<T> PartialOrd for NodeIndex<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.index.cmp(&other.index))
    }
}

/// [`PartialOrd`] is implemented manually, so there is no requirement on `T` to also implement [`PartialOrd`]
/// and comparison to [`usize`] is possible.
impl<T> PartialOrd<usize> for NodeIndex<T> {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        Some(self.index.cmp(other))
    }
}

/// [`Display`] shows the biggest row of associated [`Tree`](crate::Tree) and `index`.
impl<T> Display for NodeIndex<T>
where
    T: TreeInterface,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeIndex::<{}>( {} )", T::BIGGEST_ROW_SIZE, self.index)
    }
}

impl<T> From<NodePosition<T>> for NodeIndex<T>
where
    T: TreeInterface,
{
    fn from(value: NodePosition<T>) -> Self {
        Self::from(LayerPosition::from(value))
    }
}

impl<T> From<LayerPosition<T>> for NodeIndex<T>
where
    T: TreeInterface,
{
    fn from(value: LayerPosition<T>) -> Self {
        let (mut index, depth) = LayerIndex::from(value).get_raw();

        for row_size in &T::rows_sizes()[0..depth] {
            let layer_size = row_size * row_size * row_size;
            index += layer_size;
        }
        NodeIndex::new(index)
    }
}

impl<T> From<LayerIndex<T>> for NodeIndex<T>
where
    T: TreeInterface,
{
    fn from(value: LayerIndex<T>) -> Self {
        Self::from(LayerPosition::from(value))
    }
}

impl<T, U> Index<NodeIndex<T>> for Vec<U>
where
    T: TreeInterface,
{
    type Output = U;

    fn index(&self, index: NodeIndex<T>) -> &Self::Output {
        &self[index.index]
    }
}

impl<T, U> IndexMut<NodeIndex<T>> for Vec<U>
where
    T: TreeInterface,
{
    fn index_mut(&mut self, index: NodeIndex<T>) -> &mut Self::Output {
        &mut self[index.index]
    }
}

impl<T, U, const N: usize> Index<NodeIndex<T>> for [U; N]
where
    T: TreeInterface,
{
    type Output = U;

    fn index(&self, index: NodeIndex<T>) -> &Self::Output {
        &self[index.index]
    }
}

impl<T, U, const N: usize> IndexMut<NodeIndex<T>> for [U; N]
where
    T: TreeInterface,
{
    fn index_mut(&mut self, index: NodeIndex<T>) -> &mut Self::Output {
        &mut self[index.index]
    }
}

impl<T> Add for NodeIndex<T>
where
    T: TreeInterface,
{
    type Output = Self;

    fn add(mut self, rhs: NodeIndex<T>) -> Self::Output {
        self.index += rhs.raw();
        assert!(self.is_valid());
        self
    }
}

impl<T> Add<usize> for NodeIndex<T>
where
    T: TreeInterface,
{
    type Output = Self;

    fn add(mut self, rhs: usize) -> Self::Output {
        self.index += rhs;
        assert!(self.is_valid());
        self
    }
}

impl<T> Sub for NodeIndex<T>
where
    T: TreeInterface,
{
    type Output = Self;

    fn sub(mut self, rhs: NodeIndex<T>) -> Self::Output {
        self.index -= rhs.raw();
        assert!(self.is_valid());
        self
    }
}

impl<T> Sub<usize> for NodeIndex<T>
where
    T: TreeInterface,
{
    type Output = Self;

    fn sub(mut self, rhs: usize) -> Self::Output {
        self.index -= rhs;
        assert!(self.is_valid());
        self
    }
}

impl<T> From<NodeIndex<T>> for usize {
    fn from(value: NodeIndex<T>) -> Self {
        value.index
    }
}

impl<T> NodeIndex<T>
where
    T: TreeInterface,
{
    /// Creates a new [NodeIndex].
    ///
    /// Validity of provided `index` is checked only in debug mode. If provided index could be
    /// invalid, use [`new_checked`](NodeIndex::new_checked).
    pub fn new(index: usize) -> Self {
        debug_assert!(Self::is_valid_index(index));
        Self {
            index,
            boo: PhantomData,
        }
    }

    /// Creates a new [NodeIndex] if provided `index` is valid, otherwise [`Err`] is returned.
    #[allow(clippy::result_unit_err)]
    pub fn new_checked(index: usize) -> Result<Self, ()> {
        if !Self::is_valid_index(index) {
            return Err(());
        }
        Ok(Self {
            index,
            boo: PhantomData,
        })
    }

    /// Returns `true` if `index` is less than [`tree size`](TreeParameters::SIZE).
    pub fn is_valid_index(index: usize) -> bool {
        index < T::SIZE
    }

    /// Returns `true` if `index` is less than [`tree size`](TreeParameters::SIZE).
    pub fn is_valid(self) -> bool {
        Self::is_valid_index(self.index)
    }

    /// Replaces the index inside with provided `index`
    /// and returns an index previously stored.
    pub fn set(&mut self, index: usize) -> usize {
        debug_assert!(Self::is_valid_index(index));
        let mut new = Self::new(index);
        std::mem::swap(self, &mut new);
        new.into()
    }

    /// Calculates depth of `index` inside associated [`Tree`](crate::Tree).
    pub fn depth(self) -> usize {
        let mut depth = 0;
        let mut layer_max_index = 0;
        for row_size in T::rows_sizes() {
            layer_max_index += row_size * row_size * row_size;
            if self.index < layer_max_index {
                break;
            }
            depth += 1;
        }
        depth
    }

    /// Returs an `index` as [`usize`].
    pub fn raw(self) -> usize {
        self.index
    }
}

/// Stores absolute position of [`Node`](crate::Node) in [`Tree`](crate::Tree).
///
/// Position is always calculated from an origin point which is bottom front left
/// corner of shallowest layer.
///
/// This structure always expects to have valid data inside
/// and in debug panics if that is not true.
#[derive(Debug)]
pub struct NodePosition<T> {
    /// Amount of nodes from an tree origin on `x` asix.
    pub x: usize,
    /// Amount of nodes from an tree origin on `y` asix.
    pub y: usize,
    /// Amount of nodes from an tree origin on `z` asix.
    pub z: usize,
    /// Layer in [`Tree`](crate::Tree).
    ///
    /// The shallowest layer is the biggest in size and size of the deepest is 1.
    pub depth: usize,
    /// Associated [`Tree`](crate::Tree).
    boo: PhantomData<T>,
}

/// [`Clone`] is implemented manually, so there is no requirement on `T` to also implement [`Clone`].
impl<T> Clone for NodePosition<T> {
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
impl<T> Copy for NodePosition<T> {}

/// [`Display`] shows the biggest row of associated [`Tree`](crate::Tree), `position` and `depth`.
impl<T> Display for NodePosition<T>
where
    T: TreeInterface,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodePosition::<{}>( x: {}, y: {}, z: {}, depth: {} )",
            T::BIGGEST_ROW_SIZE,
            self.x,
            self.y,
            self.z,
            self.depth
        )
    }
}

/// [`PartialEq`] is implemented manually, so there is no requirement on `T` to also implement [`PartialEq`].
impl<T> PartialEq for NodePosition<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.depth == other.depth
    }
}

impl<T> From<NodeIndex<T>> for NodePosition<T>
where
    T: TreeInterface,
{
    fn from(value: NodeIndex<T>) -> Self {
        LayerPosition::from(value).into()
    }
}

impl<T> From<LayerPosition<T>> for NodePosition<T>
where
    T: TreeInterface,
{
    fn from(value: LayerPosition<T>) -> Self {
        let multiplier = T::BIGGEST_ROW_SIZE / T::row_size(value.depth);

        let x = value.x * multiplier;
        let y = value.y * multiplier;
        let z = value.z * multiplier;

        NodePosition::new(x, y, z, value.depth)
    }
}

impl<T> From<LayerIndex<T>> for NodePosition<T>
where
    T: TreeInterface,
{
    fn from(value: LayerIndex<T>) -> Self {
        LayerPosition::from(value).into()
    }
}

impl<T> NodePosition<T>
where
    T: TreeInterface,
{
    /// Creates a new [NodePosition].
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

    /// Returns `true` if `x`, `y`, `z` are less than
    /// [BIGGEST_ROW_SIZE](TreeParameters::BIGGEST_ROW_SIZE) of associated [`Tree`]
    /// and valid in provided `depth` and `depth` is less
    /// [MAX_DEPTH_INDEX](TreeParameters::MAX_DEPTH_INDEX) of associated [`Tree`].
    pub fn is_valid_position(x: usize, y: usize, z: usize, depth: usize) -> bool {
        let divisor = 2_usize.pow(depth as u32);

        depth <= T::MAX_DEPTH_INDEX
            && x % divisor == 0
            && x < T::BIGGEST_ROW_SIZE
            && y % divisor == 0
            && y < T::BIGGEST_ROW_SIZE
            && z % divisor == 0
            && z < T::BIGGEST_ROW_SIZE
    }

    /// Returns `true` if call to [NodePosition::is_valid_position] on inner values
    /// is evaluated to `true`.
    pub fn is_valid(self) -> bool {
        Self::is_valid_position(self.x, self.y, self.z, self.depth)
    }

    /// Returns [NodePosition] of child in bottom front left corner of parrent node
    /// if exists, otherwise [`None`] is returned.
    pub fn child_position(mut self) -> Option<Self> {
        if self.depth == 0 {
            return None;
        }
        self.depth -= 1;
        Some(self)
    }
}

#[cfg(test)]
pub(crate) mod node_index_tests {

    use std::ops::Add;

    use crate::{LayerIndex, LayerPosition, NodeIndex, NodePosition, Tree};

    type TestTree = Tree<usize, 73>;
    type TestNodeIndex = NodeIndex<TestTree>;
    type TestNodePosition = NodePosition<TestTree>;
    type TestLayerPosition = LayerPosition<TestTree>;
    type TestLayerIndex = LayerIndex<TestTree>;

    #[test]
    fn is_valid_index() {
        let index = TestNodeIndex::is_valid_index(0);
        assert!(index);

        let index = TestNodeIndex::is_valid_index(72);
        assert!(index);

        let index = TestNodeIndex::is_valid_index(528);
        assert!(!index);
    }

    #[test]
    fn new() {
        TestNodeIndex::new(0);
        TestNodeIndex::new(72);
        std::panic::catch_unwind(|| TestNodeIndex::new(73)).unwrap_err();
    }

    #[test]
    fn set() {
        let mut index = TestNodeIndex::new(0);
        let inner = index.set(25);
        assert_eq!(inner, 0);
        assert_eq!(index, TestNodeIndex::new(25));
    }

    #[test]
    fn depth() {
        let index = TestNodeIndex::new(0);
        assert_eq!(index.depth(), 0);

        let index = TestNodeIndex::new(63);
        assert_eq!(index.depth(), 0);

        let index = TestNodeIndex::new(64);
        assert_eq!(index.depth(), 1);

        let index = TestNodeIndex::new(71);
        assert_eq!(index.depth(), 1);

        let index = TestNodeIndex::new(72);
        assert_eq!(index.depth(), 2);
    }

    #[test]
    fn from_node_position() {
        let pos = TestNodePosition::new(0, 0, 0, 0);
        assert_eq!(TestNodeIndex::from(pos), TestNodeIndex::new(0));

        let pos = TestNodePosition::new(1, 0, 0, 0);
        assert_eq!(TestNodeIndex::from(pos), TestNodeIndex::new(1));

        let pos = TestNodePosition::new(3, 3, 3, 0);
        assert_eq!(TestNodeIndex::from(pos), TestNodeIndex::new(63));

        let pos = TestNodePosition::new(0, 0, 0, 1);
        assert_eq!(TestNodeIndex::from(pos), TestNodeIndex::new(64));

        let pos = TestNodePosition::new(2, 0, 0, 1);
        assert_eq!(TestNodeIndex::from(pos), TestNodeIndex::new(65));

        let pos = TestNodePosition::new(2, 2, 2, 1);
        assert_eq!(TestNodeIndex::from(pos), TestNodeIndex::new(71));

        let pos = TestNodePosition::new(0, 0, 0, 2);
        assert_eq!(TestNodeIndex::from(pos), TestNodeIndex::new(72));
    }

    #[test]
    fn from_layer_position() {
        let pos = TestLayerPosition::new(0, 0, 0, 0);
        assert_eq!(TestNodeIndex::from(pos), TestNodeIndex::new(0));

        let pos = TestLayerPosition::new(1, 0, 0, 0);
        assert_eq!(TestNodeIndex::from(pos), TestNodeIndex::new(1));

        let pos = TestLayerPosition::new(3, 3, 3, 0);
        assert_eq!(TestNodeIndex::from(pos), TestNodeIndex::new(63));

        let pos = TestLayerPosition::new(0, 0, 0, 1);
        assert_eq!(TestNodeIndex::from(pos), TestNodeIndex::new(64));

        let pos = TestLayerPosition::new(1, 0, 0, 1);
        assert_eq!(TestNodeIndex::from(pos), TestNodeIndex::new(65));

        let pos = TestLayerPosition::new(1, 1, 1, 1);
        assert_eq!(TestNodeIndex::from(pos), TestNodeIndex::new(71));

        let pos = TestLayerPosition::new(0, 0, 0, 2);
        assert_eq!(TestNodeIndex::from(pos), TestNodeIndex::new(72));
    }

    #[test]
    fn from_layer_index() {
        let index = TestLayerIndex::new(0, 0);
        assert_eq!(TestNodeIndex::from(index), TestNodeIndex::new(0));

        let index = TestLayerIndex::new(1, 0);
        assert_eq!(TestNodeIndex::from(index), TestNodeIndex::new(1));

        let index = TestLayerIndex::new(63, 0);
        assert_eq!(TestNodeIndex::from(index), TestNodeIndex::new(63));

        let index = TestLayerIndex::new(0, 1);
        assert_eq!(TestNodeIndex::from(index), TestNodeIndex::new(64));

        let index = TestLayerIndex::new(1, 1);
        assert_eq!(TestNodeIndex::from(index), TestNodeIndex::new(65));

        let index = TestLayerIndex::new(7, 1);
        assert_eq!(TestNodeIndex::from(index), TestNodeIndex::new(71));

        let index = TestLayerIndex::new(0, 2);
        assert_eq!(TestNodeIndex::from(index), TestNodeIndex::new(72));
    }

    #[test]
    #[should_panic]
    fn index() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let index = TestNodeIndex::new(0);

        assert_eq!(arr[index], 1);

        let index = TestNodeIndex::new(10);

        arr[index] = 5;
        assert_eq!(arr[index], 5);

        #[allow(clippy::no_effect)]
        arr[index];
    }

    #[test]
    fn copy() {
        let index = TestNodeIndex::new(0);
        let _ = index.add(25);
        assert_eq!(index.raw(), 0);
    }
}

#[cfg(test)]
pub(crate) mod node_position_tests {
    use crate::{LayerIndex, LayerPosition, NodeIndex, NodePosition, Tree};

    type TestTree = Tree<usize, 73>;
    type TestNodeIndex = NodeIndex<TestTree>;
    type TestNodePosition = NodePosition<TestTree>;
    type TestLayerPosition = LayerPosition<TestTree>;
    type TestLayerIndex = LayerIndex<TestTree>;

    #[test]
    fn new() {
        TestNodePosition::new(0, 0, 0, 0);
        TestNodePosition::new(3, 3, 3, 0);
        TestNodePosition::new(0, 0, 0, 1);
        TestNodePosition::new(2, 2, 2, 1);
        TestNodePosition::new(0, 0, 0, 2);

        std::panic::catch_unwind(|| TestNodePosition::new(4, 0, 0, 0)).unwrap_err();
        std::panic::catch_unwind(|| TestNodePosition::new(0, 4, 0, 0)).unwrap_err();
        std::panic::catch_unwind(|| TestNodePosition::new(0, 0, 4, 0)).unwrap_err();
        std::panic::catch_unwind(|| TestNodePosition::new(4, 4, 4, 0)).unwrap_err();
        std::panic::catch_unwind(|| TestNodePosition::new(1, 0, 1, 1)).unwrap_err();
        std::panic::catch_unwind(|| TestNodePosition::new(0, 3, 0, 1)).unwrap_err();
        std::panic::catch_unwind(|| TestNodePosition::new(1, 0, 1, 2)).unwrap_err();
    }

    #[test]
    fn child_position() {
        let pos = TestNodePosition::new(0, 0, 0, 0);
        assert_eq!(pos.child_position(), None);

        let pos = TestNodePosition::new(1, 0, 0, 0);
        assert_eq!(pos.child_position(), None);

        let pos = TestNodePosition::new(3, 3, 3, 0);
        assert_eq!(pos.child_position(), None);

        let pos = TestNodePosition::new(0, 0, 0, 1);
        assert_eq!(
            pos.child_position(),
            Some(TestNodePosition::new(0, 0, 0, 0))
        );

        let pos = TestNodePosition::new(2, 0, 0, 1);
        assert_eq!(
            pos.child_position(),
            Some(TestNodePosition::new(2, 0, 0, 0))
        );

        let pos = TestNodePosition::new(2, 2, 2, 1);
        assert_eq!(
            pos.child_position(),
            Some(TestNodePosition::new(2, 2, 2, 0))
        );

        let pos = TestNodePosition::new(0, 0, 0, 2);
        assert_eq!(
            pos.child_position(),
            Some(TestNodePosition::new(0, 0, 0, 1))
        );
    }

    #[test]
    fn from_node_index() {
        let index = TestNodeIndex::new(0);
        assert_eq!(
            TestNodePosition::from(index),
            TestNodePosition::new(0, 0, 0, 0)
        );

        let index = TestNodeIndex::new(1);
        assert_eq!(
            TestNodePosition::from(index),
            TestNodePosition::new(1, 0, 0, 0)
        );

        let index = TestNodeIndex::new(63);
        assert_eq!(
            TestNodePosition::from(index),
            TestNodePosition::new(3, 3, 3, 0)
        );

        let index = TestNodeIndex::new(64);
        assert_eq!(
            TestNodePosition::from(index),
            TestNodePosition::new(0, 0, 0, 1)
        );

        let index = TestNodeIndex::new(65);
        assert_eq!(
            TestNodePosition::from(index),
            TestNodePosition::new(2, 0, 0, 1)
        );

        let index = TestNodeIndex::new(71);
        assert_eq!(
            TestNodePosition::from(index),
            TestNodePosition::new(2, 2, 2, 1)
        );

        let index = TestNodeIndex::new(72);
        assert_eq!(
            TestNodePosition::from(index),
            TestNodePosition::new(0, 0, 0, 2)
        );
    }

    #[test]
    fn from_layer_index() {
        let index = TestLayerIndex::new(0, 0);
        assert_eq!(
            TestNodePosition::from(index),
            TestNodePosition::new(0, 0, 0, 0)
        );

        let index = TestLayerIndex::new(1, 0);
        assert_eq!(
            TestNodePosition::from(index),
            TestNodePosition::new(1, 0, 0, 0)
        );

        let index = TestLayerIndex::new(63, 0);
        assert_eq!(
            TestNodePosition::from(index),
            TestNodePosition::new(3, 3, 3, 0)
        );

        let index = TestLayerIndex::new(0, 1);
        assert_eq!(
            TestNodePosition::from(index),
            TestNodePosition::new(0, 0, 0, 1)
        );

        let index = TestLayerIndex::new(1, 1);
        assert_eq!(
            TestNodePosition::from(index),
            TestNodePosition::new(2, 0, 0, 1)
        );

        let index = TestLayerIndex::new(7, 1);
        assert_eq!(
            TestNodePosition::from(index),
            TestNodePosition::new(2, 2, 2, 1)
        );

        let index = TestLayerIndex::new(0, 2);
        assert_eq!(
            TestNodePosition::from(index),
            TestNodePosition::new(0, 0, 0, 2)
        );
    }

    #[test]
    fn from_layer_position() {
        let pos = TestLayerPosition::new(0, 0, 0, 0);
        assert_eq!(
            TestNodePosition::from(pos),
            TestNodePosition::new(0, 0, 0, 0)
        );

        let pos = TestLayerPosition::new(1, 0, 0, 0);
        assert_eq!(
            TestNodePosition::from(pos),
            TestNodePosition::new(1, 0, 0, 0)
        );

        let pos = TestLayerPosition::new(3, 3, 3, 0);
        assert_eq!(
            TestNodePosition::from(pos),
            TestNodePosition::new(3, 3, 3, 0)
        );

        let pos = TestLayerPosition::new(0, 0, 0, 1);
        assert_eq!(
            TestNodePosition::from(pos),
            TestNodePosition::new(0, 0, 0, 1)
        );

        let pos = TestLayerPosition::new(1, 0, 0, 1);
        assert_eq!(
            TestNodePosition::from(pos),
            TestNodePosition::new(2, 0, 0, 1)
        );

        let pos = TestLayerPosition::new(1, 1, 1, 1);
        assert_eq!(
            TestNodePosition::from(pos),
            TestNodePosition::new(2, 2, 2, 1)
        );

        let pos = TestLayerPosition::new(0, 0, 0, 2);
        assert_eq!(
            TestNodePosition::from(pos),
            TestNodePosition::new(0, 0, 0, 2)
        );
    }
}
