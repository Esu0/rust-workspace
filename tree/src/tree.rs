use std::{borrow::Borrow, cmp};

mod node;
pub mod link_cut;
use node::{Node, NodeRef, Direction::*};

pub struct SearchTree<T> {
    root: Option<NodeRef<T>>
}

impl<T> SearchTree<T> {
    pub const fn new() -> Self {
        SearchTree { root: None }
    }
}
use std::cmp::Ordering::*;

impl<T: Ord> SearchTree<T> {
    pub fn find<Q>(&self, key: &Q) -> Option<(NodeRef<T>, cmp::Ordering)>
    where
        Q: ?Sized + Ord,
        T: Borrow<Q>,
    {
        self.root.map(|root| {
            let mut current = root;
            loop {
                let data = current.node();
                match key.cmp(data.val.borrow()) {
                    Equal => return (current, Equal),
                    Less => match data.left {
                        Some(left) => current = left,
                        None => return (current, Greater),
                    }
                    Greater => match data.right {
                        Some(right) => current = right,
                        None => return (current, Less),
                    }
                }
            }
        })
    }

    pub fn insert(&mut self, val: T) -> bool {
        let Some((mut node, ord)) = self.find(&val) else {
            self.root = Some(NodeRef::new(Node::new(val)));
            return true;
        };
        node.splay();
        let mut new_node = Node::new(val);
        match ord {
            Equal => {
                self.root = Some(node);
                return false;
            },
            Less => {
                new_node.set_child(Left, Some(node));
            }
            Greater => {
                new_node.set_child(Right, Some(node));
            }
        }
        let new_node_ref = NodeRef::new(new_node);
        node.set_parent(Some(new_node_ref));
        self.root = Some(new_node_ref);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut tree = SearchTree::new();
        assert!(tree.insert(1));
        assert!(tree.insert(2));
        assert!(tree.insert(3));
        assert!(!tree.insert(1));
        assert!(!tree.insert(2));
        assert!(!tree.insert(3));
        assert!(tree.insert(30));
        assert!(tree.insert(31));
        assert!(tree.insert(32));
        assert!(tree.insert(33));
        assert!(!tree.insert(33));
        assert!(!tree.insert(31));
        println!("{}", node::Tree::from(tree.root.unwrap()));
        tree.insert(3);
        println!("{}", node::Tree::from(tree.root.unwrap()));
        tree.insert(33);
        println!("{}", node::Tree::from(tree.root.unwrap()));
        tree.insert(1);
        println!("{}", node::Tree::from(tree.root.unwrap()));
        tree.insert(30);
        println!("{}", node::Tree::from(tree.root.unwrap()));
    }
}