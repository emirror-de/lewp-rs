#![deny(missing_docs)]

//! Straightforward HTML document building. Write your HTML with the full power
//! of a programming language instead of creating messy and confusing
//! templates that contain typing errors.
//!
//! ## API example
//!
//! ```
//! use lewp_html::{api::*, LanguageTag, Script, DocumentExt, NodeExt};
//!
//! let valid_html = document(LanguageTag::parse("de-DE").unwrap(),
//!    head(vec![
//!        script(Script::Src("/my-javascript")),
//!        script(Script::Inline("console.log(\"hello world!\");")),
//!        link("text/css", "/static/css/main.css")
//!    ]),
//!    body(
//!        vec![
//!        h1(vec![text("Hello World")]),
//!        p(vec![text("This is a paragraph!")])
//!            .attrs(vec![
//!                ("class", "prelude"),
//!                ("id", "first-paragraph")
//!            ]),
//!        h2(vec![text("The elegant way to create a DOM!")]),
//!        p(vec![text("Creating DOM has never been easier.")])
//!            .attr("class", "highlighted")
//!        ])
//! ).into_html();
//!
//! let expected_html = "<!DOCTYPE html><html lang=\"de\"><head><script src=\"/my-javascript\"></script><script>console.log(\"hello world!\");</script><link href=\"/static/css/main.css\" type=\"text/css\"></head><body><h1>Hello World</h1><p class=\"prelude\" id=\"first-paragraph\">This is a paragraph!</p><h2>The elegant way to create a DOM!</h2><p class=\"highlighted\">Creating DOM has never been easier.</p></body></html>";
//!
//! assert_eq!(&valid_html, expected_html);
//! ```

/// API function definitions to create your html document. See the API example
/// above.
pub mod api;

mod document;
mod document_ext;
mod node;
mod node_ext;
mod types;

pub use {
    charsets::Charset,
    document::Document,
    document_ext::DocumentExt,
    langtag::LanguageTag,
    node::Node,
    node_ext::NodeExt,
    types::*,
};

/// A list of nodes.
pub type Nodes = Vec<Node>;

#[macro_export]
/// Error log with added prefix for this crate.
macro_rules! error {
    ($($arg:tt)*) => (
        log::error!("(LEWP-DOM) {}", format!($($arg)*));
    )
}
