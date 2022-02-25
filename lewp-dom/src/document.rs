//! Document definition.

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
    std::{cell::RefCell, ops::Deref, rc::Rc},
};

/// An HTML document.
pub type Document = RcDom;

impl crate::DocumentExt for Document {
    fn into_html(self) -> String {
        let mut bytes = vec![];
        let document: SerializableHandle = self.document.into();
        serialize(&mut bytes, &document, SerializeOpts::default()).unwrap();
        String::from_utf8(bytes).unwrap()
    }
}
