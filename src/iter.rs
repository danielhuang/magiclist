use std::iter::FusedIterator;

use crate::{container::NodeContainer, MagicList};

impl<T, C: NodeContainer<T>> IntoIterator for MagicList<T, C> {
    type Item = T;

    type IntoIter = IntoIter<T, C>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

pub struct IntoIter<T, C: NodeContainer<T>> {
    list: MagicList<T, C>,
}

impl<T, C: NodeContainer<T>> Iterator for IntoIter<T, C> {
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

impl<T, C: NodeContainer<T>> ExactSizeIterator for IntoIter<T, C> {}

impl<T, C: NodeContainer<T>> DoubleEndedIterator for IntoIter<T, C> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.list.is_empty() {
            None
        } else {
            Some(self.list.pop())
        }
    }
}

impl<T, C: NodeContainer<T>> FusedIterator for IntoIter<T, C> {}

impl<T, C: NodeContainer<T>> MagicList<T, C> {
    pub fn iter(&self) -> Iter<T, C> {
        Iter {
            list: self,
            i: 0,
            j: self.len(),
        }
    }
}

impl<'a, T, C: NodeContainer<T>> IntoIterator for &'a MagicList<T, C> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T, C>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, T, C: NodeContainer<T>> {
    list: &'a MagicList<T, C>,
    i: usize,
    j: usize,
}

impl<'a, T, C: NodeContainer<T>> Iterator for Iter<'a, T, C> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        assert!(self.i <= self.j);
        if self.i == self.j {
            None
        } else {
            let x = &self.list[self.i];
            self.i += 1;
            Some(x)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.j - self.i, Some(self.j - self.i))
    }
}

impl<'a, T, C: NodeContainer<T>> DoubleEndedIterator for Iter<'a, T, C> {
    fn next_back(&mut self) -> Option<Self::Item> {
        assert!(self.i <= self.j);
        if self.i == self.j {
            None
        } else {
            self.j -= 1;
            let x = &self.list[self.j];
            Some(x)
        }
    }
}

impl<'a, T, C: NodeContainer<T>> FusedIterator for Iter<'a, T, C> {}

impl<'a, T, C: NodeContainer<T>> ExactSizeIterator for Iter<'a, T, C> {}

impl<T, C: NodeContainer<T>> FromIterator<T> for MagicList<T, C> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = MagicList::default();
        for x in iter {
            list.push(x);
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let list: MagicList<_> = (0..100).collect();
        let mut iter: Iter<_, _> = list.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next_back(), Some(&99));
        assert_eq!(iter.next_back(), Some(&98));
        assert_eq!(iter.next_back(), Some(&97));
        assert!(iter.len() == 94);
        assert!(iter.copied().eq(3..97));
    }
}
