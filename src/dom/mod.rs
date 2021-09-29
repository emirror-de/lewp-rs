//! DOM abstractions and helper functions such as [NodeCreator].

use {
    html5ever::{namespace_url, ns, tendril::Tendril, LocalName, QualName},
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
    /// True if `value` is available in `class` attribute.
    fn has_class(&self, value: &str) -> bool;
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

    fn has_class(&self, class_value: &str) -> bool {
        let attrs = match &self.data {
            NodeData::Element { attrs, .. } => attrs,
            _ => return false,
        };
        for attr in attrs.borrow().iter() {
            if attr.name != QualName::new(None, ns!(), LocalName::from("class")) {
                continue;
            }
            let value = String::from(attr.value.clone());
            return value.split(' ').any(|x| x == class_value);
        }
        false
    }
#[test]
fn has_class() {
    use crate::dom::NodeCreator;
    let class_value = "has-class-test";
    let non_existent_class_value = "non-existing-class-value";
    let elem = NodeCreator::element(
        "a",
        vec![NodeCreator::attribute("class", class_value)],
        None,
    );
    assert_eq!(true, elem.has_class(class_value));
    assert_eq!(false, elem.has_class(non_existent_class_value));
}

#[test]
fn add_class() {
    use crate::dom::NodeCreator;
    let class_value = "add-class-test";
    let elem = NodeCreator::element("a", vec![], None);
    elem.add_class(class_value);
    assert_eq!(true, elem.has_class(class_value));
}

#[test]
fn add_class_ignore_duplicate() {
    use crate::dom::NodeCreator;
    let class_value = "add-class-test";
    let elem = NodeCreator::element(
        "a",
        vec![NodeCreator::attribute("class", class_value)],
        None,
    );
    elem.add_class(class_value);
    let attrs = match &elem.data {
        NodeData::Element { attrs, .. } => attrs,
        _ => {
            assert_eq!(false, true);
            return;
        }
    };
    assert_eq!(
        true,
        *attrs.borrow() == vec![NodeCreator::attribute("class", class_value)]
    );
}

#[test]
fn remove_class() {
    use crate::dom::NodeCreator;
    let class_value = "has-class-test";
    let elem = NodeCreator::element(
        "a",
        vec![
            NodeCreator::attribute("class", class_value),
            NodeCreator::attribute("class", class_value),
        ],
        None,
    );
    elem.remove_class(class_value);
    // has_class can be used here because it is tested as well, see above
    assert_eq!(false, elem.has_class(class_value));
}
