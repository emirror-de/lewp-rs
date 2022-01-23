#![deny(missing_docs)]

//! Straightforward HTML document building.
//!
//! ## Write valid HTML documents
//! When compiled in debug mode, lewp-dom sends errors to the log if
//! not permitted content is appended to a node. See
//! [https://developer.mozilla.org/en-US/docs/Web/HTML/Element] for specification.
//!
//! ### Example
//! **NEEDS TO BE WRITTEN**
//!
//! ## API example
//!
//! ```ignore
//! document("de-DE", vec![
//!    head(vec![
//!        script("/something"),
//!        link("text/css", "/static/css/main.css")
//!    ]),
//!    body(
//!        vec![
//!        h1(vec![text("Hello World")]),
//!        p("This is a paragraph!")
//!            .attrs(vec![
//!                ("class", "prelude"),
//!                ("id", "first-paragraph")
//!            ]),
//!        h2(vec![text("The elegant way to create a DOM!")]),
//!        p("Creating DOM has never been easier.")
//!            .attr("class", "highlighted")
//!        ])
//! ])
//! ```

/// Function definitions to create HTML nodes and attributes.
mod nodes;

use {
    html5ever::{
        namespace_url,
        ns,
        serialize::{serialize, SerializeOpts},
        tendril::Tendril,
        Attribute,
        LocalName,
        QualName,
    },
    rcdom::{NodeData, RcDom, SerializableHandle},
    std::rc::Rc,
};

pub use {langtag::LanguageTag, nodes::*};

/// Contains the actual error message that can be printed on screen.
pub type ErrorMessage = String;

/// Describes the possible errors that can occur.
pub enum DomError {
    /// Validation errors. First node is the parent, second the child
    /// (could also be a subchild).
    Validation(ErrorMessage, Node, Node),
    /// Raised when the node is not supported.
    Unsupported(ErrorMessage, Node),
}

/// An HTML document.
pub type Document = RcDom;

/// Methods for easy handling of an HTML document.
pub trait DocumentExt {
    /// Converts the given node to an HTML string.
    fn into_html(self) -> String;
}

impl DocumentExt for Document {
    fn into_html(self) -> String {
        let mut bytes = vec![];
        let document: SerializableHandle = self.document.into();
        serialize(&mut bytes, &document, SerializeOpts::default()).unwrap();
        String::from_utf8(bytes).unwrap()
    }
}

/// A HTML node.
pub type Node = Rc<rcdom::Node>;

/// Methods for easy interaction with a `DOM` [Node].
pub trait NodeExt {
    /// Adds an Attribute with given name and value to the node.
    ///
    /// If the attribute is already present, it will be overridden.
    fn attr(self, name: &str, value: &str) -> Self;
    /// Adds a list of attributes to the node.
    ///
    /// The attributes are tuples of name and value. Attributes that are already
    /// set are being overridden.
    fn attrs(self, attributes: Vec<(&str, &str)>) -> Self;
    /// Appends the given [Vec]tor of [Node]s as children.
    fn append_children(&mut self, nodes: &mut Vec<Node>);
    /// Validates the given children recursively.
    ///
    /// Throws warnings in the log if children contains nodes that are not allowed.
    fn validate(&self, children: &Vec<Node>);
    /// Returns the attribute index if present.
    fn find_attribute(&self, attribute_name: &str) -> Option<usize>;
    /// Checks if the given attribute matches the value.
    fn attribute_eq(&self, attribute_name: &str, value: &str) -> bool;
}

impl NodeExt for Node {
    fn attr(self, name: &str, value: &str) -> Self {
        {
            let attrs = match &self.data {
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

    fn attrs(mut self, attributes: Vec<(&str, &str)>) -> Self {
        for attr in attributes {
            self = self.attr(attr.0, attr.1);
        }
        self
    }

    fn append_children(&mut self, nodes: &mut Vec<Node>) {
        #[cfg(debug_assertions)]
        self.validate(&nodes);
        self.children.borrow_mut().append(nodes);
    }

    fn validate(&self, children: &Vec<Node>) {
        for child in children {
            nodes::validator::validate(&self, &child);
        }
    }

    fn find_attribute(&self, attribute_name: &str) -> Option<usize> {
        let attrs = match &self.data {
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

    fn attribute_eq(&self, attribute_name: &str, value: &str) -> bool {
        if let Some(index) = self.find_attribute(attribute_name) {
            let mut attrs = match &self.data {
                NodeData::Element { attrs, .. } => attrs.borrow_mut(),
                _ => return false,
            };
            return &*attrs[index].value == value;
        }
        false
    }
}

#[test]
fn attributes() {
    use crate::*;
    let node = div(vec![]).attr("class", "container");
    // get attributes of node
    let attrs = match &node.data {
        NodeData::Element { attrs, .. } => attrs,
        _ => {
            // this should not occur
            assert_eq!(false, true);
            return;
        }
    };
    // attribute to expect
    let expected = Attribute {
        name: QualName::new(None, ns!(), LocalName::from("class")),
        value: Tendril::from("container"),
    };

    assert_eq!(true, *attrs.borrow() == vec![expected,]);
}
