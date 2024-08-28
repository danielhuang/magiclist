use std::mem::{replace, take};

use crate::B;

#[derive(Debug, Clone)]
pub(crate) struct Tree<T> {
    pub(crate) total_len: usize,
    pub(crate) children: Vec<Node<T>>,
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self {
            total_len: 0,
            children: vec![],
        }
    }
}

impl<T> Tree<T> {
    fn extend(&mut self, other: Self) {
        self.total_len += other.total_len;
        self.children.extend(other.children);
    }

    fn merge_children(&mut self, left_i: usize) {
        let to_merge = self.children.remove(left_i + 1);
        self.children[left_i].extend_equal_level(to_merge);
    }

    fn split_child(&mut self, i: usize) {
        let new_child = self.children[i].split_off_half();
        self.children.insert(i + 1, new_child);
    }

    fn rotate_left(&mut self, left_i: usize) {
        let to_move = self.children[left_i + 1].pop_child_left();
        self.children[left_i].extend_equal_level(to_move);
    }

    fn rotate_right(&mut self, left_i: usize) {
        let to_move = self.children[left_i].pop_child_right();
        self.children[left_i + 1].prepend_equal_level(to_move);
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Node<T> {
    Leaf(Vec<T>),
    Tree(Tree<T>),
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Self::Leaf(vec![])
    }
}

impl<T> Node<T> {
    pub(crate) fn len(&self) -> usize {
        match self {
            Node::Leaf(x) => x.len(),
            Node::Tree(Tree { total_len, .. }) => *total_len,
        }
    }

    fn children_count(&self) -> usize {
        match self {
            Node::Leaf(x) => x.len(),
            Node::Tree(Tree { children, .. }) => children.len(),
        }
    }

    pub(crate) fn split_off_half(&mut self) -> Node<T> {
        match self {
            Node::Leaf(x) => {
                let i = x.len() / 2;
                Node::Leaf(x.split_off(i))
            }
            Node::Tree(Tree {
                total_len,
                children,
            }) => {
                let i = children.len() / 2;
                let right = children.split_off(i);
                let right_len = right.iter().map(|x| x.len()).sum::<usize>();
                *total_len -= right_len;
                Node::Tree(Tree {
                    total_len: right_len,
                    children: right,
                })
            }
        }
    }

    pub(crate) fn canon(self) -> Self {
        match self {
            Node::Leaf(x) => Node::Leaf(x),
            Node::Tree(mut x) => {
                if x.children.is_empty() {
                    Node::Leaf(vec![])
                } else if x.children.len() == 1 {
                    x.children.pop().unwrap().canon()
                } else {
                    Node::Tree(x)
                }
            }
        }
    }

    pub(crate) fn is_overfull(&self) -> bool {
        self.children_count() > B * 2
    }

    fn is_underfull(&self) -> bool {
        self.children_count() < B
    }

    fn extend_equal_level(&mut self, other: Node<T>) {
        if other.len() == 0 {
            return;
        }
        match (&mut *self, other) {
            (Node::Leaf(a), Node::Leaf(b)) => a.extend(b),
            (Node::Tree(a), Node::Tree(b)) => a.extend(b),
            _ => unreachable!("must be same type"),
        }
    }

    fn prepend_equal_level(&mut self, other: Node<T>) {
        if other.len() == 0 {
            return;
        }
        match (&mut *self, other) {
            (Node::Leaf(a), Node::Leaf(b)) => {
                a.splice(0..0, b);
            }
            (Node::Tree(a), Node::Tree(b)) => {
                a.total_len += b.total_len;
                a.children.splice(0..0, b.children);
            }
            _ => unreachable!("must be same type"),
        }
    }

    fn real_len(&self) -> usize {
        match self {
            Node::Leaf(x) => x.len(),
            Node::Tree(x) => x.children.iter().map(|x| x.real_len()).sum(),
        }
    }

    pub(crate) fn split_off(&mut self, i: usize) -> Node<T> {
        match self {
            Node::Leaf(x) => Node::Leaf(x.split_off(i)),
            Node::Tree(tree) => {
                if i == 0 {
                    return replace(self, Node::Leaf(vec![]));
                }
                if i == tree.total_len {
                    return Node::Leaf(vec![]);
                }
                let orig_len = tree.total_len;
                let mut child_i = 0;
                let mut total_before_child_i = 0;
                let right = loop {
                    dbg!(total_before_child_i, i);
                    if total_before_child_i == i {
                        let right = tree.children.split_off(child_i);
                        tree.total_len = i;
                        break right;
                    }
                    assert!(total_before_child_i < i);
                    if i < total_before_child_i + tree.children[child_i].len() {
                        let mut right = tree.children.split_off(child_i + 1);
                        tree.total_len = total_before_child_i + tree.children[child_i].len();
                        if i > total_before_child_i {
                            let extra = tree.children[child_i].split_off(i - total_before_child_i);
                            tree.total_len -= extra.len();
                            right.insert(0, extra);
                        }
                        assert_eq!(i, tree.total_len);
                        break right;
                    }
                    total_before_child_i += tree.children[child_i].len();
                    child_i += 1;
                };
                let right = Tree {
                    children: right,
                    total_len: orig_len - i,
                };
                let mut right = Node::Tree(right);
                right.cleanup(0);
                if self.len() > 0 {
                    self.cleanup(self.children_count() - 1);
                }
                debug_assert_eq!(self.len(), self.real_len());
                debug_assert_eq!(right.len(), right.real_len());
                assert_eq!(self.len() + right.len(), orig_len);
                right
            }
        }
    }

    fn depth(&self) -> usize {
        match self {
            Node::Leaf(_) => 0,
            Node::Tree(x) => {
                debug_assert!(x
                    .children
                    .iter()
                    .all(|y| y.depth() == x.children[0].depth()));
                x.children[0].depth() + 1
            }
        }
    }

    fn prepend(&mut self, other: Self) {
        match self.depth().cmp(&other.depth()) {
            std::cmp::Ordering::Less => {
                unreachable!("only call from extend")
            }
            std::cmp::Ordering::Equal => self.prepend_equal_level(other),
            std::cmp::Ordering::Greater => {
                let Node::Tree(x) = self else { unreachable!() };
                x.total_len += other.len();
                x.children.first_mut().unwrap().prepend(other);
                self.cleanup(0);
            }
        }
    }

    pub(crate) fn extend(&mut self, other: Self) {
        match self.depth().cmp(&other.depth()) {
            std::cmp::Ordering::Less => {
                let left = take(self);
                let mut right = other;
                right.prepend(left);
                right.cleanup(0);
                *self = right;
            }
            std::cmp::Ordering::Equal => {
                self.extend_equal_level(other);
            }
            std::cmp::Ordering::Greater => {
                let Node::Tree(x) = self else { unreachable!() };
                x.total_len += other.len();
                x.children.last_mut().unwrap().extend(other);
                self.cleanup(self.children_count() - 1);
            }
        }
    }

    fn cleanup(&mut self, i: usize) {
        match self {
            Node::Leaf(_) => {}
            Node::Tree(x) => {
                if x.children[i].is_underfull() {
                    if i > 0 && x.children[i - 1].children_count() > B {
                        x.rotate_right(i - 1);
                    } else if i + 1 < x.children.len() && x.children[i + 1].children_count() > B {
                        x.rotate_left(i)
                    } else if i > 0
                        && x.children[i - 1].children_count() + x.children[i].children_count()
                            <= 2 * B
                    {
                        x.merge_children(i - 1)
                    } else if i + 1 < x.children.len()
                        && x.children[i].children_count() + x.children[i + 1].children_count()
                            <= 2 * B
                    {
                        x.merge_children(i)
                    } else {
                        assert!(x.children.len() == 1)
                    }
                } else if x.children[i].is_overfull() {
                    if i > 0 && x.children[i - 1].children_count() < 2 * B {
                        x.rotate_left(i - 1);
                    } else if i + 1 < x.children.len() && x.children[i + 1].children_count() < 2 * B
                    {
                        x.rotate_right(i);
                    } else {
                        x.split_child(i);
                    }
                }
            }
        }
    }

    fn pop_child_left(&mut self) -> Self {
        match self {
            Node::Leaf(x) => Node::Leaf(vec![x.remove(0)]),
            Node::Tree(x) => {
                let left = x.children.remove(0);
                x.total_len -= left.len();
                Node::Tree(Tree {
                    total_len: left.len(),
                    children: vec![left],
                })
            }
        }
    }

    fn pop_child_right(&mut self) -> Self {
        match self {
            Node::Leaf(x) => Node::Leaf(vec![x.pop().unwrap()]),
            Node::Tree(x) => {
                let right = x.children.pop().unwrap();
                x.total_len -= right.len();
                Node::Tree(Tree {
                    total_len: right.len(),
                    children: vec![right],
                })
            }
        }
    }
}
