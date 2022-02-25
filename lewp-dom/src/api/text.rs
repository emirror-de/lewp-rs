use {
    crate::{Document, Node, NodeExt},
    rcdom::{ContentCategory, ContentModel, TypedProperties},
};

/// Creates a text node.
pub fn text(text: &str) -> Node {
    rcdom::Node::new_typed(
        rcdom::NodeData::Text {
            contents: RefCell::new(Tendril::from(text)),
        },
        rcdom::TypedProperties {
            content_model: |_p, _c| vec![ContentModel::Specific(false)],
            content_categories: |_n| {
                use rcdom::ContentCategory::*;
                vec![Phrasing]
            },
        },
    )
}
