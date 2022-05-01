//! Document definition.

use {
    html5ever::{
        parse_document,
        serialize::{serialize, SerializeOpts},
        tendril::TendrilSink,
        tree_builder::TreeBuilderOpts,
        ParseOpts,
    },
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

    fn from_string(s: String) -> Result<Self, std::io::Error> {
        let opts = ParseOpts {
            tree_builder: TreeBuilderOpts {
                drop_doctype: true,
                ..Default::default()
            },
            ..Default::default()
        };
        parse_document(RcDom::default(), opts)
            .from_utf8()
            .read_from(&mut s.as_bytes())
    }
}
