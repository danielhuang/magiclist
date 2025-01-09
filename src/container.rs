use std::ops::{Index, IndexMut};

use crate::node::Node;

pub trait NodeContainer<T, const B: usize>:
    Default
    + Extend<Node<T, B, Self>>
    + FromIterator<Node<T, B, Self>>
    + IntoIterator<Item = Node<T, B, Self>>
    + Index<usize, Output = Node<T, B, Self>>
    + IndexMut<usize>
{
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn push(&mut self, value: Node<T, B, Self>);
    fn pop(&mut self) -> Option<Node<T, B, Self>>;
    fn remove(&mut self, index: usize) -> Node<T, B, Self>;
    fn insert(&mut self, index: usize, value: Node<T, B, Self>);
    fn append(&mut self, other: &mut Self);
    fn split_off(&mut self, at: usize) -> Self;
    fn clear(&mut self);
    fn prepend_many(&mut self, other: Self);
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Node<T, B, Self>> + 'a
    where
        T: 'a;
    fn first_mut(&mut self) -> Option<&mut Node<T, B, Self>>;
    fn last_mut(&mut self) -> Option<&mut Node<T, B, Self>>;
}
