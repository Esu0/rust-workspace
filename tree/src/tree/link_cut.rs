use super::node::NodeRef;

pub struct LinkCutTree<T> {
    root: Option<NodeRef<T>>,
}

impl<T> LinkCutTree<T> {
    pub const fn new() -> Self {
        LinkCutTree { root: None }
    }

    pub fn expose(&mut self, mut node: NodeRef<T>) {
        node.splay();
    }
}

impl<T> Default for LinkCutTree<T> {
    fn default() -> Self {
        Self::new()
    }
}
