#![deny(missing_docs)]

//! Straightforward HTML document building.
//!
//! ## Write valid HTML documents
//! When compiled in debug mode, lewp-dom sends errors to the log and skips the
//! append if not permitted content is appended to a node. See
//! <https://developer.mozilla.org/en-US/docs/Web/HTML/Element> for specification.
//!
//! When compiled in release mode, the type checking mechanism is not included to
//! increase performance. Not permitted content is then appended.
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

///// Function definitions to create HTML nodes and attributes.
//mod nodes;

/// API function definitions.
pub mod api;

///// Helper macros for internal use. Used eg. for simple definition of typed nodes.
//#[macro_use]
//pub(crate) mod macros;

//pub mod html;

#[macro_use]
pub(crate) mod custom_log;

mod document;
mod document_ext;
mod node;
mod node_ext;

pub use {
    document::Document,
    document_ext::DocumentExt,
    langtag::LanguageTag,
    node::Node,
    node_ext::NodeExt,
};

///// Contains the actual error message that can be printed on screen.
//pub type ErrorMessage = String;
//
///// Describes the possible errors that can occur.
//pub enum DomError {
//    /// Validation errors. First node is the parent, second the child
//    /// (could also be a subchild).
//    Validation(ErrorMessage, Rc<rcdom::Node>, Rc<rcdom::Node>),
//    /// Raised when the node is not supported.
//    Unsupported(ErrorMessage, Rc<rcdom::Node>),
//}
//
//#[test]
//fn attributes() {
//    use crate::*;
//    let node = div(vec![]).attr("class", "container");
//    // get attributes of node
//    let attrs = match &node.data {
//        NodeData::Element { attrs, .. } => attrs,
//        _ => {
//            // this should not occur
//            assert_eq!(false, true);
//            return;
//        }
//    };
//    // attribute to expect
//    let expected = Attribute {
//        name: QualName::new(None, ns!(), LocalName::from("class")),
//        value: Tendril::from("container"),
//    };
//
//    assert_eq!(true, *attrs.borrow() == vec![expected,]);
//}
