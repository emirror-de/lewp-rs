//! Traits and functions required for rendering a page.

use {
    super::Assembler,
    crate::dom::Nodes,
    html5ever::{serialize, serialize::SerializeOpts},
    markup5ever_rcdom::SerializableHandle,
};

/// Functions to render `self` as String.
pub trait Render {
    /// Renders the page.
    fn render(&self, modules: Vec<Nodes>) -> String
    where
        Self: Assembler,
    {
        let mut bytes = vec![];
        let document: SerializableHandle = <Self as Assembler>::full(self, modules).document.into();
        serialize(&mut bytes, &document, SerializeOpts::default()).unwrap();
        String::from_utf8(bytes).unwrap()
    }
}
