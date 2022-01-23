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
        self.children.borrow_mut().append(nodes);
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
