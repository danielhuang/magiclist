use node::{Node, Tree};
use std::{
    fmt::Debug,
    mem::take,
    ops::{Index, IndexMut},
};

pub(crate) const B: usize = 12;

mod iter;
mod node;

#[derive(Debug, Clone)]
pub struct MagicList<T> {
    root: Node<T>,
}

impl<T> Default for MagicList<T> {
    fn default() -> Self {
        Self {
            root: Node::Leaf(vec![]),
        }
    }
}

impl<T> MagicList<T> {
    pub fn extend(&mut self, other: Self) {
        self.root.extend(other.root);
        if self.root.is_overfull() {
            let len = self.root.len();
            let right = self.root.split_off_half();
            let left = take(&mut self.root);
            self.root = Node::Tree(Tree {
                total_len: len,
                children: vec![left, right],
            })
        }
    }

    pub fn concat(mut self, right: Self) -> Self {
        self.extend(right);
        self
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, x: T) {
        self.extend(MagicList {
            root: Node::Leaf(vec![x]),
        })
    }

    fn make_canon(&mut self) {
        self.root = take(&mut self.root).canon();
    }

    fn canon(self) -> Self {
        Self {
            root: self.root.canon(),
        }
    }

    pub fn insert(&mut self, at: usize, x: T) {
        let right = self.root.split_off(at).canon();
        self.make_canon();
        self.push(x);
        self.extend(Self { root: right });
    }

    pub fn split_off(&mut self, i: usize) -> Self {
        let right = Self {
            root: self.root.split_off(i).canon(),
        };
        self.make_canon();
        right
    }

    pub fn split_at(mut self, i: usize) -> (Self, Self) {
        let right = self.split_off(i);
        (self.canon(), right.canon())
    }

    pub fn remove(&mut self, i: usize) -> T {
        let right = self.split_off(i + 1);
        let mid = self.split_off(i);
        self.extend(right);
        assert!(mid.len() == 1);
        let mid = mid.root.canon();
        match mid {
            Node::Leaf(mut x) => x.pop().unwrap(),
            Node::Tree(_) => unreachable!(),
        }
    }

    pub fn pop(&mut self) -> T {
        let i = self.len() - 1;
        self.remove(i)
    }
}

impl<T> Index<usize> for MagicList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len(), "out of bounds");
        let mut i = index;
        let mut node = &self.root;
        loop {
            match node {
                Node::Leaf(x) => return &x[i],
                Node::Tree(x) => {
                    let mut child_i = 0;
                    let mut j = 0;
                    while i - j >= x.children[child_i].len() {
                        j += x.children[child_i].len();
                        child_i += 1;
                    }
                    i -= j;
                    node = &x.children[child_i];
                }
            }
        }
    }
}

impl<T> IndexMut<usize> for MagicList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len(), "out of bounds");
        let mut i = index;
        let mut node = &mut self.root;
        loop {
            match node {
                Node::Leaf(x) => return &mut x[i],
                Node::Tree(x) => {
                    let mut child_i = 0;
                    let mut j = 0;
                    while i - j >= x.children[child_i].len() {
                        j += x.children[child_i].len();
                        child_i += 1;
                    }
                    i -= j;
                    node = &mut x.children[child_i];
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::SliceRandom;
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;

    use super::*;

    #[test]
    fn push_back() {
        for size in 0..200 {
            let mut list = MagicList::default();
            let v: Vec<_> = (0..size).collect();
            for &n in &v {
                list.push(n);
            }
            let v2: Vec<_> = list.into_iter().collect();
            assert_eq!(v, v2);
        }
    }

    #[test]
    fn push_front() {
        for size in 0..200 {
            let mut list = MagicList::default();
            let v: Vec<_> = (0..size).collect();
            for &n in &v {
                list.insert(0, n);
            }
            let v2: Vec<_> = list.into_iter().rev().collect();
            assert_eq!(v, v2);
        }
    }

    #[test]
    fn remove_and_insert() {
        for size in 0..200 {
            let mut list = MagicList::default();
            let v: Vec<_> = (0..size).collect();
            for &n in &v {
                list.insert(0, n);
            }
            for i in 0..list.len() {
                let removed = list.remove(i);
                list.insert(i, removed);
            }
            let v2: Vec<_> = list.into_iter().rev().collect();
            assert_eq!(v, v2);
        }
    }

    #[test]
    fn split_and_merge() {
        for size in 0..200 {
            let mut list = MagicList::default();
            for n in 0..size {
                list.push(n);
            }
            for i in 0..=list.len() {
                let (left, right) = list.split_at(i);
                list = left.concat(right);
            }
            assert!(list.into_iter().eq(0..size))
        }
    }

    #[test]
    fn binary_insertion_sort() {
        for size in 0..200 {
            let mut v: Vec<_> = (0..size).collect();
            let mut rng = ChaChaRng::seed_from_u64(size);
            v.shuffle(&mut rng);

            let mut list = MagicList::default();
            for x in v {
                if list.is_empty() {
                    list.push(x);
                } else {
                    let mut lo = 0;
                    let mut hi = list.len();
                    while hi > lo {
                        let mid = (lo + hi) / 2;
                        if x < list[mid] {
                            hi = mid;
                        } else {
                            lo = mid + 1;
                        }
                    }
                    list.insert(hi, x);
                }
            }

            dbg!(list.clone().into_iter().collect::<Vec<_>>());

            assert!(list.into_iter().eq(0..size));
        }
    }
}
