use std::marker::PhantomData;

use crate::{direction::Axis, Tree, TreeInterface};

pub struct LayerIter<'a, T, U> {
    nodes: &'a [&'a T],
    layer: usize,
    axis: Axis,
    row_size: usize,
    boo: PhantomData<U>,
}

impl<'a, T, U> LayerIter<'a, T, U> {
    pub fn from_tree<const SIZE: usize>(tree: &Tree<T, SIZE>, depth: usize) -> Self
    where
        Tree<T, SIZE>: TreeInterface,
    {
        // let range = Tree::<T, SIZE>::layer_range(depth);
        // let nodes = tree[];
        // let layer_start = tree.layer_start(depth);
        // let nodes = tree[]
        todo!()
    }
}
