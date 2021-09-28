//! DOM abstractions and helper functions such as [NodeCreator].

use {
    html5ever::{namespace_url, ns, tendril::Tendril, Attribute, LocalName, QualName},
    std::rc::Rc,
};

mod node_creator;

pub use {
    markup5ever_rcdom::{Handle, Node, NodeData, RcDom, SerializableHandle},
    node_creator::NodeCreator,
};

/// A set of DOM nodes.
pub type Nodes = Vec<Rc<Node>>;

/// Methods for easy interaction with a `DOM` [Node].
pub trait NodeExt {
    /// Adds `value` to the `class` attribute. Adds the `class` attribute if
    /// it is not present.
    fn add_class(&self, value: &str);
    /// Removes the given `value` from the `class` attribute.
    fn remove_class(&self, value: &str);
}

impl NodeExt for Node {
    fn add_class(&self, class_value: &str) {
        let attrs = match &self.data {
            NodeData::Element { attrs, .. } => attrs,
            _ => return,
        };
        for attr in attrs.borrow_mut().iter_mut() {
            if attr.name != QualName::new(None, ns!(), LocalName::from("class")) {
                continue;
            }
            let value = String::from(attr.value.clone());
            let mut value = value.split(' ').collect::<Vec<_>>();
            if value.contains(&class_value) {
                return;
            }
            value.push(class_value);
            attr.value = Tendril::from(value.join(" "));
            return;
        }
        attrs
            .borrow_mut()
            .push(NodeCreator::attribute("class", class_value));
    }

    fn remove_class(&self, class_value: &str) {
        let attrs = match &self.data {
            NodeData::Element { attrs, .. } => attrs,
            _ => return,
        };
        for attr in attrs.borrow_mut().iter_mut() {
            if attr.name != QualName::new(None, ns!(), LocalName::from("class")) {
                continue;
            }
            let value = String::from(attr.value.clone());
            let mut value = value.split(' ').collect::<Vec<_>>();
            value.retain(|x| x != &class_value);
            attr.value = Tendril::from(value.join(" "));
            return;
        }
    }
}
