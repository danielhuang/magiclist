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

impl<T> MagicList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            list: self,
            i: 0,
            j: self.len(),
        }
    }
}

impl<'a, T> IntoIterator for &'a MagicList<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, T> {
    list: &'a MagicList<T>,
    i: usize,
    j: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
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

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
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

impl<'a, T> FusedIterator for Iter<'a, T> {}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

impl<T> FromIterator<T> for MagicList<T> {
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
        let mut iter: Iter<_> = list.iter();
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
