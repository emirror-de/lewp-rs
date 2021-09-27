//! DOM abstractions and helper functions such as [NodeCreator].

use std::rc::Rc;

mod node_creator;

pub use {
    markup5ever_rcdom::{Handle, Node, NodeData, RcDom, SerializableHandle},
    node_creator::NodeCreator,
};

/// A set of DOM nodes.
pub type Nodes = Vec<Rc<Node>>;
