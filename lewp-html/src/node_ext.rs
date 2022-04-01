//! Trait for DOM node interactions.

use {
    crate::Node,
    html5ever::{
        namespace_url,
        ns,
        tendril::Tendril,
        Attribute,
        LocalName,
        QualName,
    },
    rcdom::NodeData,
    std::cell::RefCell,
};

/// Methods for easy interaction with a `DOM` [Node].
pub trait NodeExt
where
    Self: Sized,
{
    /// The children of this node.
    fn children(&self) -> &RefCell<Vec<Node>>;
    /// The [NodeData].
    fn data(&self) -> &rcdom::NodeData;
    /// Appends the given children to the node.
    fn append_children(&self, children: Vec<Node>);
    /// Appends the given child to the node.
    fn append_child(&self, child: Node);
    /// Returns the tag name as string if available.
    fn tag_name(&self) -> Option<String> {
        match &self.data() {
            NodeData::Doctype { name, .. } => Some(name.to_string()),
            NodeData::Element { name, .. } => Some(name.local.to_string()),
            _ => None,
        }
    }
    /// Adds an Attribute with given name and value to the node.
    ///
    /// If the attribute is already present, it will be overridden.
    fn attr(self, name: &str, value: &str) -> Self {
        {
            let attrs = match &self.data() {
                NodeData::Element { attrs, .. } => attrs,
                _ => return self,
            };
            let mut attrs = attrs.borrow_mut();
            attrs.push(Attribute {
                name: QualName::new(None, ns!(), LocalName::from(name)),
                value: Tendril::from(value),
            });
        }
        self
    }
    /// Adds an Attribute with given name and value to the node without consuming self.
    ///
    /// If the attribute is already present, it will be overridden.
    fn borrow_attr(&self, name: &str, value: &str) {
        {
            let attrs = match &self.data() {
                NodeData::Element { attrs, .. } => attrs,
                _ => return,
            };
            let mut attrs = attrs.borrow_mut();
            attrs.push(Attribute {
                name: QualName::new(None, ns!(), LocalName::from(name)),
                value: Tendril::from(value),
            });
        }
    }
    /// Adds a list of attributes to the node.
    ///
    /// The attributes are tuples of name and value. Attributes that are already
    /// set are being overridden.
    fn attrs(mut self, attributes: Vec<(&str, &str)>) -> Self {
        for attr in attributes {
            self = self.attr(attr.0, attr.1);
        }
        self
    }
    /// Adds a list of attributes to the node without consuming self.
    ///
    /// The attributes are tuples of name and value. Attributes that are already
    /// set are being overridden.
    fn borrow_attrs(&self, attributes: Vec<(&str, &str)>) {
        for attr in attributes {
            self.borrow_attr(attr.0, attr.1);
        }
    }
    /// Returns the attribute index if present.
    fn find_attribute(&self, attribute_name: &str) -> Option<usize> {
        let attrs = match &self.data() {
            NodeData::Element { attrs, .. } => attrs,
            _ => return None,
        };
        for (idx, attr) in attrs.borrow().iter().enumerate() {
            if attr.name
                != QualName::new(None, ns!(), LocalName::from(attribute_name))
            {
                continue;
            }
            return Some(idx);
        }
        None
    }
    /// Checks if the given attribute matches the value.
    fn attribute_eq(&self, attribute_name: &str, value: &str) -> bool {
        if let Some(index) = self.find_attribute(attribute_name) {
            let attrs = match &self.data() {
                NodeData::Element { attrs, .. } => attrs.borrow_mut(),
                _ => return false,
            };
            return &*attrs[index].value == value;
        }
        false
    }

    /// Adds `value` to the `class` attribute. Adds the `class` attribute if
    /// it is not present.
    fn add_class(&self, class_value: &str) {
        let attribute_index = self.find_attribute("class");
        let attrs = match self.data() {
            NodeData::Element { attrs, .. } => attrs,
            _ => return,
        };
        let mut attrs = attrs.borrow_mut();
        match attribute_index {
            None => {
                attrs.push(Attribute {
                    name: QualName::new(None, ns!(), LocalName::from("class")),
                    value: Tendril::from(class_value),
                });
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

    /// Removes the given `value` from the `class` attribute.
    fn remove_class(&self, class_value: &str) {
        let attribute_index = self.find_attribute("class");
        let attrs = match self.data() {
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

    /// True if `value` is available in `class` attribute.
    fn has_class(&self, class_value: &str) -> bool {
        let attribute_index = self.find_attribute("class");
        let attrs = match self.data() {
            NodeData::Element { attrs, .. } => attrs,
            _ => return false,
        };
        let attrs = attrs.borrow();
        match attribute_index {
            None => false,
            Some(index) => {
                attrs[index].value.split(' ').any(|x| x == class_value)
            }
        }
    }

    /// Toggles the given `value` of the `class` attribute. Creates the class
    /// attribute if not set yet.
    fn toggle_class(&self, class_value: &str) {
        let attribute_index = self.find_attribute("class");
        let attrs = match self.data() {
            NodeData::Element { attrs, .. } => attrs,
            _ => return,
        };
        let mut attrs = attrs.borrow_mut();
        match attribute_index {
            None => {
                attrs.push(Attribute {
                    name: QualName::new(None, ns!(), LocalName::from("class")),
                    value: Tendril::from(class_value),
                });
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

    /// Removes the attribute with the given `name`. Does nothing
    /// if the attribute does not exist. This method does not compare its
    /// values.
    fn remove_attribute(&self, name: &str) {
        let attribute_index = self.find_attribute(name);
        let attrs = match self.data() {
            NodeData::Element { attrs, .. } => attrs,
            _ => return,
        };
        let mut attrs = attrs.borrow_mut();
        if let Some(index) = attribute_index {
            attrs.remove(index);
        };
    }
}
