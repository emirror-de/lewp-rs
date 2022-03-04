//! Implementation of a generic HTML5 node.

use std::{cell::RefCell, rc::Rc};

/// A HTML5 node.
pub type Node = Rc<rcdom::Node>;

impl crate::NodeExt for Node {
    fn children(&self) -> &RefCell<Vec<Node>> {
        &self.children
    }
    fn data(&self) -> &rcdom::NodeData {
        &self.data
    }
    fn append_children(&self, children: Vec<Node>) {
        for child in children {
            self.append_child(child);
        }
    }
    fn append_child(&self, child: Node) {
        self.children.borrow_mut().push(child);
    }
}
