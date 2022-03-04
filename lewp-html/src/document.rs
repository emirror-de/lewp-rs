//! Document definition.

use {
    html5ever::serialize::{serialize, SerializeOpts},
    rcdom::{RcDom, SerializableHandle},
};

/// An HTML5 document.
pub type Document = RcDom;

impl crate::DocumentExt for Document {
    fn into_html(self) -> String {
        let mut bytes = vec![];
        let document: SerializableHandle = self.document.into();
        serialize(&mut bytes, &document, SerializeOpts::default()).unwrap();
        String::from_utf8(bytes).unwrap()
    }
}
