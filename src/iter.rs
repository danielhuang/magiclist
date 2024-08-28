use std::iter::FusedIterator;

use crate::MagicList;

impl<T> IntoIterator for MagicList<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

pub struct IntoIter<T> {
    list: MagicList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.list.is_empty() {
            None
        } else {
            Some(self.list.remove(0))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len(), Some(self.list.len()))
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.list.is_empty() {
            None
        } else {
            Some(self.list.pop())
        }
    }
}

impl<T> FusedIterator for IntoIter<T> {}
