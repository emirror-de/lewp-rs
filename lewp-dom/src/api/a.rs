use {
    crate::{Document, Node, NodeExt},
    rcdom::{ContentCategory, ContentModel, TypedProperties},
};

/// Creates an anchor element.
pub fn a(children: Vec<Node>) -> Node {
    let anchor = new_typed_element(
        "a",
        rcdom::TypedProperties {
            content_model: |p, c| {
                vec![
                    ContentModel::Specific(
                        !c.categories().contains(&ContentCategory::Interactive),
                    ),
                    ContentModel::Specific(c.tag_name() != p.tag_name()),
                    ContentModel::Specific(
                        c.find_attribute("tabindex").is_none(),
                    ),
                    ContentModel::Category(ContentCategory::Transparent),
                ]
            },
            content_categories: |n| {
                use rcdom::ContentCategory::*;
                let mut v = vec![Flow, Phrasing, Palpable];
                if n.find_attribute("href").is_some() {
                    v.push(Interactive);
                }
                v
            },
        },
        children,
    );
    anchor
}
