use crate::{container::NodeContainer, node::Node};
use std::{
    ops::{Index, IndexMut},
    rc::Rc,
};

#[derive(Clone, Debug)]
pub struct NodeRcVec<T: Clone, const B: usize>(Rc<Vec<Node<T, B, Self>>>);

impl<T: Clone, const B: usize> NodeContainer<T, B> for NodeRcVec<T, B> {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn push(&mut self, value: Node<T, B, Self>) {
        Rc::make_mut(&mut self.0).push(value)
    }

    fn pop(&mut self) -> Option<Node<T, B, Self>> {
        Rc::make_mut(&mut self.0).pop()
    }

    fn remove(&mut self, index: usize) -> Node<T, B, Self> {
        Rc::make_mut(&mut self.0).remove(index)
    }

    fn insert(&mut self, index: usize, value: Node<T, B, Self>) {
        Rc::make_mut(&mut self.0).insert(index, value)
    }

    fn append(&mut self, other: &mut Self) {
        Rc::make_mut(&mut self.0).append(Rc::make_mut(&mut other.0))
    }

    fn split_off(&mut self, at: usize) -> Self {
        Self(Rc::new(Rc::make_mut(&mut self.0).split_off(at)))
    }

    fn clear(&mut self) {
        Rc::make_mut(&mut self.0).clear()
    }

    fn prepend_many(&mut self, other: Self) {
        let mut new_vec = (*other.0).clone();
        new_vec.extend(self.0.iter().cloned());
        self.0 = Rc::new(new_vec);
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Node<T, B, Self>> + 'a
    where
        T: 'a,
    {
        self.0.iter()
    }

    fn first_mut(&mut self) -> Option<&mut Node<T, B, Self>> {
        Rc::make_mut(&mut self.0).first_mut()
    }

    fn last_mut(&mut self) -> Option<&mut Node<T, B, Self>> {
        Rc::make_mut(&mut self.0).last_mut()
    }
}

impl<T: Clone, const B: usize> Default for NodeRcVec<T, B> {
    fn default() -> Self {
        Self(Rc::new(Vec::new()))
    }
}

impl<T: Clone, const B: usize> Extend<Node<T, B, Self>> for NodeRcVec<T, B> {
    fn extend<I: IntoIterator<Item = Node<T, B, Self>>>(&mut self, iter: I) {
        Rc::make_mut(&mut self.0).extend(iter)
    }
}

impl<T: Clone, const B: usize> FromIterator<Node<T, B, Self>> for NodeRcVec<T, B> {
    fn from_iter<I: IntoIterator<Item = Node<T, B, Self>>>(iter: I) -> Self {
        Self(Rc::new(Vec::from_iter(iter)))
    }
}

impl<T: Clone, const B: usize> IntoIterator for NodeRcVec<T, B> {
    type Item = Node<T, B, Self>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match Rc::try_unwrap(self.0) {
            Ok(vec) => vec.into_iter(),
            Err(rc) => (*rc).clone().into_iter(),
        }
    }
}

impl<T: Clone, const B: usize> Index<usize> for NodeRcVec<T, B> {
    type Output = Node<T, B, Self>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Clone, const B: usize> IndexMut<usize> for NodeRcVec<T, B> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut Rc::make_mut(&mut self.0)[index]
    }
}

#[cfg(test)]
mod tests {

    use std::cell::RefCell;

    use crate::MagicList;

    use super::*;

    struct CloneTracker(i32, Rc<RefCell<usize>>);

    impl Clone for CloneTracker {
        fn clone(&self) -> Self {
            *self.1.borrow_mut() += 1;
            Self(self.0, self.1.clone())
        }
    }

    #[test]
    fn lazy() {
        let tracker = Rc::new(RefCell::new(0));

        let list: MagicList<_, 12, NodeRcVec<_, 12>> = (0..10000)
            .map(|x| CloneTracker(x, tracker.clone()))
            .collect();

        let mut list2 = list.clone();

        assert_eq!(list.len(), list2.len());
        assert_eq!(*tracker.borrow(), 0);

        list2[0].0 = 100;

        assert!(*tracker.borrow() > 0);
        assert!(*tracker.borrow() < 10000);
        assert_ne!(list[0].0, list2[0].0);
    }

    #[test]
    fn blowup() {
        let list: MagicList<_, 12, NodeRcVec<_, 12>> = (0..9999).collect();
        let list: MagicList<_, 12, NodeRcVec<_, 12>> = (0..9999).map(|_| list.clone()).collect();
        let list: MagicList<_, 12, NodeRcVec<_, 12>> = (0..9999).map(|_| list.clone()).collect();
        let list: MagicList<_, 12, NodeRcVec<_, 12>> = (0..9999).map(|_| list.clone()).collect();
        assert!(!list.is_empty());
    }
}
