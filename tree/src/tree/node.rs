#![allow(dead_code)]
use std::{fmt, ptr::{addr_of, addr_of_mut, NonNull}};

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub(super) val: T,
    pub(super) parent: Option<NodeRef<T>>,
    pub(super) left: Option<NodeRef<T>>,
    pub(super) right: Option<NodeRef<T>>,
}

#[derive(Debug)]
pub struct NodeRef<T>(NonNull<Node<T>>);

impl<T> Clone for NodeRef<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for NodeRef<T> {}

impl<T> PartialEq for NodeRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for NodeRef<T> {}

pub struct Tree<T>(Option<NodeRef<T>>);

impl<T> Clone for Tree<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Tree<T> {}


impl<T: fmt::Display> Tree<T> {
    fn fmt_rec(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        if let Some(node) = self.0 {
            let node = node.node();
            Tree(node.right).fmt_rec(f, depth + 1)?;
            writeln!(f, "{:indent$}{}", "", node.val, indent = depth * 2)?;
            Tree(node.left).fmt_rec(f, depth + 1)?;
        }
        Ok(())
    }
}

impl<T: fmt::Display> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_rec(f, 0)
    }
}

use Direction::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Left => Right,
            Right => Left,
        }
    }
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Node {
            val,
            parent: None,
            left: None,
            right: None,
        }
    }

    pub fn child(&self, dir: Direction) -> Option<NodeRef<T>> {
        match dir {
            Left => self.left,
            Right => self.right,
        }
    }

    pub fn parent(&self) -> Option<NodeRef<T>> {
        self.parent
    }

    pub fn set_child(&mut self, dir: Direction, child: Option<NodeRef<T>>) -> Option<NodeRef<T>> {
        match dir {
            Left => {
                let tmp = self.left;
                self.left = child;
                tmp
            },
            Right => {
                let tmp = self.right;
                self.right = child;
                tmp
            },
        }
    }
}

impl<T> NodeRef<T> {
    pub fn new(node: Node<T>) -> Self {
        NodeRef(NonNull::from(Box::leak(Box::new(node))))
    }

    pub fn child(self, dir: Direction) -> Option<Self> {
        unsafe {
            match dir {
                Left => addr_of!((*self.0.as_ptr()).left).read(),
                Right => addr_of!((*self.0.as_ptr()).right).read(),
            }
        }
    }

    pub fn parent(self) -> Option<Self> {
        unsafe { addr_of!((*self.0.as_ptr()).parent).read() }
    }

    /// selfの変更前の子を返す
    pub fn set_child(&mut self, dir: Direction, child: Option<Self>) -> Option<Self> {
        unsafe {
            let ptr = match dir {
                Left => addr_of_mut!((*self.0.as_ptr()).left),
                Right => addr_of_mut!((*self.0.as_ptr()).right),
            };
            let ret = ptr.read();
            ptr.write(child);
            ret
        }
    }

    /// selfの変更前の子とchildの変更前の親を返す
    pub fn link_child(&mut self, dir: Direction, child: Option<Self>) -> (Option<Self>, Option<Self>) {
        (self.set_child(dir, child), child.and_then(|mut child| child.set_parent(Some(*self))))
    }

    /// selfの変更前の親を返す
    pub fn set_parent(&mut self, parent: Option<Self>) -> Option<Self> {
        unsafe {
            let ptr = addr_of_mut!((*self.0.as_ptr()).parent);
            let ret = ptr.read();
            ptr.write(parent);
            ret
        }
    }

    /// selfの変更前の親とparentの変更前の子を返す
    pub fn link_parent(&mut self, dir: Direction, parent: Option<Self>) -> (Option<Self>, Option<Self>) {
        (self.set_parent(parent), parent.and_then(|mut parent| parent.set_child(dir, Some(*self))))
    }

    pub fn is_root(self) -> bool {
        self.parent().is_none()
    }

    pub fn rotr(&mut self) -> Option<Self> {
        if let Some(mut left) = self.child(Left) {
            let (p, c) = self.link_parent(Right, Some(left));
            self.link_child(Left, c);
            if let Some(mut p) = p {
                p.link_child(p.direction(*self).unwrap(), Some(left));
            } else {
                left.set_parent(None);
            }
            Some(left)
        } else {
            None
        }
    }

    pub fn rotl(&mut self) -> Option<Self> {
        if let Some(mut right) = self.child(Right) {
            let (p, c) = self.link_parent(Left, Some(right));
            self.link_child(Right, c);
            if let Some(mut p) = p {
                p.link_child(p.direction(*self).unwrap(), Some(right));
            } else {
                right.set_parent(None);
            }
            Some(right)
        } else {
            None
        }
    }

    pub fn rot(&mut self, dir: Direction) -> Option<Self> {
        match dir {
            Left => self.rotl(),
            Right => self.rotr(),
        }
    }

    pub fn parent_and_direction(self) -> Option<(Self, Direction)> {
        self.parent().and_then(|p| {
            p.direction(self).map(|dir| (p, dir))
        })
    }

    pub fn direction(self, child: Self) -> Option<Direction> {
        if Some(child) == self.child(Left) {
            Some(Left)
        } else if Some(child) == self.child(Right) {
            Some(Right)
        } else {
            None
        }
    }

    pub fn splay(&mut self) {
        self.splay_with(|node| node.parent_and_direction());
        // debug_assert!(self.is_root());
    }

    pub fn splay_with<F>(&mut self, mut f: F)
    where
        F: FnMut(Self) -> Option<(Self, Direction)>,
    {
        while let Some((mut parent, dir1)) = f(*self) {
            if let Some((mut grandparent, dir2)) = f(parent) {
                if dir1 == dir2 {
                    grandparent.rot(dir1.opposite());
                    parent.rot(dir1.opposite());
                } else {
                    parent.rot(dir2);
                    grandparent.rot(dir1);
                }
            } else {
                parent.rot(dir1.opposite());
                break;
            }
        }
    }

    pub fn node(&self) -> &Node<T> {
        unsafe { self.0.as_ref() }
    }

    pub fn insert_val(&mut self, dir: Direction, val: T) -> Self {
        let mut new_node = Node {
            val,
            parent: Some(*self),
            left: None,
            right: None,
        };
        let child = self.child(dir);
        new_node.set_child(dir, child);
        let new_node_ref = NodeRef::new(new_node);
        if let Some(mut child) = child {
            child.set_parent(Some(new_node_ref));
        }
        self.set_child(dir, Some(new_node_ref));
        new_node_ref
    }
}

impl<T: fmt::Debug> NodeRef<T> {
    fn debug_ancestor(self) {
        let mut current = self;
        print!("{:?}", current.node().val);
        while let Some(parent) = current.parent() {
            current = parent;
            print!(" -> {:?}", current.node().val);
        }
        println!();
    }
}

impl<'a, T> From<&'a mut Node<T>> for NodeRef<T> {
    fn from(value: &'a mut Node<T>) -> Self {
        NodeRef(NonNull::from(value))
    }
}

impl<T> From<NodeRef<T>> for Tree<T> {
    fn from(value: NodeRef<T>) -> Self {
        Tree(Some(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_test() {
        let mut root = NodeRef::new(Node::new(1));
        let mut node2 = NodeRef::new(Node::new(2));
        let mut node3 = NodeRef::new(Node::new(3));
        root.set_child(Left, Some(node2));
        root.set_child(Right, Some(node3));
        node2.set_parent(Some(root));
        node3.set_parent(Some(root));
        let mut node = NodeRef::new(Node::new(4));
        let mut node5 = NodeRef::new(Node::new(5));
        node.set_child(Left, Some(node5));
        root.set_parent(Some(node));
        node.set_child(Right, Some(root));
        node5.set_parent(Some(node));
        let tree = Tree::from(node);
        println!("{}", tree);
        node5.splay();

        println!("{}", tree);
        println!("{}", Tree::from(node5));

        let mut root = NodeRef::new(Node::new(5));
        let mut node3 = root.insert_val(Left, 4).insert_val(Left, 2).insert_val(Right, 3);
        root.insert_val(Right, 6);
        println!("{}", Tree::from(root));

        node3.debug_ancestor();
        node3.splay();
        println!("{}", Tree::from(node3));
        root = node3.insert_val(Left, 10);
        root.splay();
        println!("{}", Tree::from(root));
    }
}
