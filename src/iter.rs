use std::iter::FusedIterator;

use crate::{container::NodeContainer, node::Node, MagicList};

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

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, T) -> B,
    {
        fn fold_owned<T, C: NodeContainer<T>, B, F>(node: Node<T, C>, init: B, f: &mut F) -> B
        where
            F: FnMut(B, T) -> B,
        {
            match node {
                Node::Leaf(values) => values.into_iter().fold(init, f),
                Node::Tree(tree) => tree
                    .children
                    .into_iter()
                    .fold(init, |acc, node| fold_owned(node, acc, f)),
            }
        }

        fold_owned(self.list.root, init, &mut f)
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

    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, &'a T) -> B,
    {
        fn fold_ref<'a, T, C: NodeContainer<T>, B, F>(
            node: &'a Node<T, C>,
            start: usize,
            end: usize,
            mut pos: usize,
            init: B,
            f: &mut F,
        ) -> (B, usize)
        where
            F: FnMut(B, &'a T) -> B,
        {
            match node {
                Node::Leaf(values) => {
                    let mut acc = init;
                    for value in values.iter() {
                        if pos >= end {
                            break;
                        }
                        if pos >= start {
                            acc = f(acc, value);
                        }
                        pos += 1;
                    }
                    (acc, pos)
                }
                Node::Tree(tree) => {
                    let mut acc = init;
                    for child in tree.children.iter() {
                        if pos >= end {
                            break;
                        }
                        let child_len = child.len();
                        if pos + child_len > start {
                            let (new_acc, new_pos) = fold_ref(child, start, end, pos, acc, f);
                            acc = new_acc;
                            pos = new_pos;
                        } else {
                            pos += child_len;
                        }
                    }
                    (acc, pos)
                }
            }
        }

        let (result, _) = fold_ref(&self.list.root, self.i, self.j, 0, init, &mut f);
        result
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
        iter.into_iter().for_each(|x| {
            list.push(x);
        });
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

    #[test]
    fn test2() {
        let list: MagicList<_> = (0..100).collect();
        let list2: MagicList<_> = list.clone().into_iter().collect();
        assert_eq!(list, list2);
    }

    #[test]
    fn test3() {
        let list: MagicList<_> = (0..100).collect();
        for i in 0..100 {
            for j in i..100 {
                let mut iter = list.iter();
                for _ in 0..i {
                    iter.next();
                }
                for _ in 0..(100 - j) {
                    iter.next_back();
                }
                let list2: MagicList<_> = iter.copied().collect();
                assert_eq!(list2, (i..j).collect());
            }
        }
    }
}
