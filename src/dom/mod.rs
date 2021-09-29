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
    /// Toggles the given `value` of the `class` attribute. Creates the class
    /// attribute if not set yet.
    fn toggle_class(&self, value: &str);
    /// Adds an attribute with the given `name` and `value`. Does nothing
    /// if the attribute already exists. This method does not compare its
    /// values.
    fn add_attribute(&self, name: &str, value: &str);
    /// Removes the attribute with the given `name`. Does nothing
    /// if the attribute does not exist. This method does not compare its
    /// values.
    fn remove_attribute(&self, name: &str);
    /// Appends the given [Node] as child.
    fn append_child(&self, node: Rc<Node>);
}

impl NodeExt for Node {
    fn add_class(&self, class_value: &str) {
        let attribute_index = find_attribute(self, "class");
        let attrs = match &self.data {
            NodeData::Element { attrs, .. } => attrs,
            _ => return,
        };
        let mut attrs = attrs.borrow_mut();
        match attribute_index {
            None => {
                attrs.push(NodeCreator::attribute("class", class_value));
            }
            Some(index) => {
                let value = String::from(attrs[index].value.clone());
                let mut value = value.split(' ').collect::<Vec<_>>();
                if value.contains(&class_value) {
                    return;
                }
                value.push(class_value);
                attrs[index].value = Tendril::from(value.join(" "));
            }
        };
    }

    fn remove_class(&self, class_value: &str) {
        let attribute_index = find_attribute(self, "class");
        let attrs = match &self.data {
            NodeData::Element { attrs, .. } => attrs,
            _ => return,
        };
        let mut attrs = attrs.borrow_mut();
        if let Some(index) = attribute_index {
            let value = String::from(attrs[index].value.clone());
            let mut value = value.split(' ').collect::<Vec<_>>();
            value.retain(|x| x != &class_value);
            attrs[index].value = Tendril::from(value.join(" "));
        };
    }

    fn has_class(&self, class_value: &str) -> bool {
        let attribute_index = find_attribute(self, "class");
        let attrs = match &self.data {
            NodeData::Element { attrs, .. } => attrs,
            _ => return false,
        };
        let attrs = attrs.borrow();
        match attribute_index {
            None => false,
            Some(index) => attrs[index].value.split(' ').any(|x| x == class_value),
        }
    }

    fn toggle_class(&self, class_value: &str) {
        let attribute_index = find_attribute(self, "class");
        let attrs = match &self.data {
            NodeData::Element { attrs, .. } => attrs,
            _ => return,
        };
        let mut attrs = attrs.borrow_mut();
        match attribute_index {
            None => {
                attrs.push(NodeCreator::attribute("class", class_value));
            }
            Some(index) => {
                let value = String::from(attrs[index].value.clone());
                let mut value = value.split(' ').collect::<Vec<_>>();
                if value.contains(&class_value) {
                    return;
                }
                value.push(class_value);
                attrs[index].value = Tendril::from(value.join(" "));
            }
        };
    }

    fn append_child(&self, node: Rc<Node>) {
        self.children.borrow_mut().push(node);
    }

    fn add_attribute(&self, name: &str, value: &str) {
        let attribute_index = find_attribute(self, name);
        let attrs = match &self.data {
            NodeData::Element { attrs, .. } => attrs,
            _ => return,
        };
        let mut attrs = attrs.borrow_mut();
        if attribute_index.is_none() {
            attrs.push(NodeCreator::attribute(name, value));
        };
    }

    fn remove_attribute(&self, name: &str) {
        let attribute_index = find_attribute(self, name);
        let attrs = match &self.data {
            NodeData::Element { attrs, .. } => attrs,
            _ => return,
        };
        let mut attrs = attrs.borrow_mut();
        if let Some(index) = attribute_index {
            attrs.remove(index);
        };
    }
}

/// Private helper function that looks for the given attribute name in the given
/// node.
fn find_attribute(node: &Node, attribute_name: &str) -> Option<usize> {
    let attrs = match &node.data {
        NodeData::Element { attrs, .. } => attrs,
        _ => return None,
    };
    for (idx, attr) in attrs.borrow().iter().enumerate() {
        if attr.name != QualName::new(None, ns!(), LocalName::from(attribute_name)) {
            continue;
        }
        return Some(idx);
    }
    None
}

#[test]
fn find_class_attribute_index() {
    use crate::dom::NodeCreator;
    let class_value = "class-value";
    let elem = NodeCreator::element(
        "a",
        vec![NodeCreator::attribute("class", class_value)],
        None,
    );
    assert_eq!(Some(0), find_attribute(&elem, "class"));

    let elem = NodeCreator::element(
        "a",
        vec![
            NodeCreator::attribute("href", "/404/Not-Found"),
            NodeCreator::attribute("class", class_value),
        ],
        None,
    );
    assert_eq!(Some(1), find_attribute(&elem, "class"));

    let elem = NodeCreator::element("a", vec![], None);
    assert_eq!(None, find_attribute(&elem, "class"));
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
    let elem = NodeCreator::element("a", vec![], None);
    elem.remove_class(class_value);
    assert_eq!(false, elem.has_class(class_value));
}

#[test]
fn append_child() {
    use crate::dom::NodeCreator;
    let link = NodeCreator::element("a", vec![], None);
    let image = NodeCreator::element("img", vec![], None);
    link.append_child(image);
    assert_eq!(link.children.borrow().len(), 1);
}

#[test]
fn add_attribute() {
    use crate::dom::NodeCreator;
    let name = "href";
    let value = "/some/path";
    let elem = NodeCreator::element("a", vec![], None);
    elem.add_attribute(name, value);
    let attrs = match &elem.data {
        NodeData::Element { attrs, .. } => attrs,
        _ => {
            assert_eq!(false, true);
            return;
        }
    };
    assert_eq!(
        true,
        *attrs.borrow() == vec![NodeCreator::attribute(name, value)]
    );
}

#[test]
fn add_attribute_ignore_duplicate() {
    use crate::dom::NodeCreator;
    let name = "href";
    let value = "/some/path";
    let elem = NodeCreator::element("a", vec![NodeCreator::attribute(name, value)], None);
    elem.add_attribute(name, value);
    let attrs = match &elem.data {
        NodeData::Element { attrs, .. } => attrs,
        _ => {
            assert_eq!(false, true);
            return;
        }
    };
    assert_eq!(
        true,
        *attrs.borrow() == vec![NodeCreator::attribute(name, value)]
    );
}

#[test]
fn remove_attribute() {
    use crate::dom::NodeCreator;
    let attribute_name = "href";
    let elem = NodeCreator::element(
        "a",
        vec![
            NodeCreator::attribute("href", "/some/value"),
            NodeCreator::attribute("rel", "stylesheet"),
        ],
        None,
    );
    elem.remove_attribute(attribute_name);
    let attrs = match &elem.data {
        NodeData::Element { attrs, .. } => attrs,
        _ => {
            assert_eq!(false, true);
            return;
        }
    };
    assert_eq!(
        true,
        *attrs.borrow() == vec![NodeCreator::attribute("rel", "stylesheet"),]
    );
    let elem = NodeCreator::element("a", vec![], None);
    elem.remove_attribute(attribute_name);
    let attrs = match &elem.data {
        NodeData::Element { attrs, .. } => attrs,
        _ => {
            assert_eq!(false, true);
            return;
        }
    };
    assert_eq!(true, *attrs.borrow() == vec![]);
}
