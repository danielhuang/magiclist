use std::ops::{Index, IndexMut};

use crate::node::Node;

pub trait NodeContainer<T>:
    Default
    + Extend<Node<T, Self>>
    + FromIterator<Node<T, Self>>
    + IntoIterator<Item = Node<T, Self>>
    + Index<usize, Output = Node<T, Self>>
    + IndexMut<usize>
{
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn push(&mut self, value: Node<T, Self>);
    fn pop(&mut self) -> Option<Node<T, Self>>;
    fn remove(&mut self, index: usize) -> Node<T, Self>;
    fn insert(&mut self, index: usize, value: Node<T, Self>);
    fn append(&mut self, other: &mut Self);
    fn split_off(&mut self, at: usize) -> Self;
    fn clear(&mut self);
    fn prepend_many(&mut self, other: Self);
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Node<T, Self>> + 'a
    where
        T: 'a;
    fn first_mut(&mut self) -> Option<&mut Node<T, Self>>;
    fn last_mut(&mut self) -> Option<&mut Node<T, Self>>;
}
