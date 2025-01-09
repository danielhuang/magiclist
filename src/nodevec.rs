use crate::container::NodeContainer;
use crate::node::Node;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct NodeVec<T, const B: usize>(Vec<Node<T, B, Self>>);

impl<T, const B: usize> NodeContainer<T, B> for NodeVec<T, B> {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn push(&mut self, value: Node<T, B, Self>) {
        self.0.push(value)
    }

    fn pop(&mut self) -> Option<Node<T, B, Self>> {
        self.0.pop()
    }

    fn remove(&mut self, index: usize) -> Node<T, B, Self> {
        self.0.remove(index)
    }

    fn insert(&mut self, index: usize, value: Node<T, B, Self>) {
        self.0.insert(index, value)
    }

    fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0)
    }

    fn split_off(&mut self, at: usize) -> Self {
        Self(self.0.split_off(at))
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    fn prepend_many(&mut self, other: Self) {
        self.0.splice(0..0, other.0);
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Node<T, B, Self>> + 'a
    where
        T: 'a,
    {
        self.0.iter()
    }

    fn first_mut(&mut self) -> Option<&mut Node<T, B, Self>> {
        self.0.first_mut()
    }

    fn last_mut(&mut self) -> Option<&mut Node<T, B, Self>> {
        self.0.last_mut()
    }
}

impl<T, const B: usize> Default for NodeVec<T, B> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T, const B: usize> Extend<Node<T, B, NodeVec<T, B>>> for NodeVec<T, B> {
    fn extend<I: IntoIterator<Item = Node<T, B, Self>>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}

impl<T, const B: usize> FromIterator<Node<T, B, NodeVec<T, B>>> for NodeVec<T, B> {
    fn from_iter<I: IntoIterator<Item = Node<T, B, Self>>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<T, const B: usize> IntoIterator for NodeVec<T, B> {
    type Item = Node<T, B, Self>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T, const B: usize> Index<usize> for NodeVec<T, B> {
    type Output = Node<T, B, Self>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const B: usize> IndexMut<usize> for NodeVec<T, B> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
