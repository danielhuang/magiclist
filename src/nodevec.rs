use crate::container::NodeContainer;
use crate::node::Node;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct NodeVec<T>(Vec<Node<T, Self>>);

impl<T> NodeContainer<T> for NodeVec<T> {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn push(&mut self, value: Node<T, Self>) {
        self.0.push(value)
    }

    fn pop(&mut self) -> Option<Node<T, Self>> {
        self.0.pop()
    }

    fn remove(&mut self, index: usize) -> Node<T, Self> {
        self.0.remove(index)
    }

    fn insert(&mut self, index: usize, value: Node<T, Self>) {
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

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Node<T, Self>> + 'a
    where
        T: 'a,
    {
        self.0.iter()
    }

    fn first_mut(&mut self) -> Option<&mut Node<T, Self>> {
        self.0.first_mut()
    }

    fn last_mut(&mut self) -> Option<&mut Node<T, Self>> {
        self.0.last_mut()
    }
}

impl<T> Default for NodeVec<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> Extend<Node<T, NodeVec<T>>> for NodeVec<T> {
    fn extend<I: IntoIterator<Item = Node<T, Self>>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}

impl<T> FromIterator<Node<T, NodeVec<T>>> for NodeVec<T> {
    fn from_iter<I: IntoIterator<Item = Node<T, Self>>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<T> IntoIterator for NodeVec<T> {
    type Item = Node<T, Self>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Index<usize> for NodeVec<T> {
    type Output = Node<T, Self>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for NodeVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
