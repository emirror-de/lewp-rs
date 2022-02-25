use {
    crate::{Node, NodeExt},
    html5ever::{namespace_url, ns, tendril::Tendril, LocalName, QualName},
    langtag::LanguageTag,
    log::error,
    rcdom::{ContentCategory, ContentModel, NodeData, TypedProperties},
    std::cell::RefCell,
};

/// Helper function to create a tag node.
fn new_typed_element(
    tag_name: &str,
    typed_properties: TypedProperties,
    children: Vec<Node>,
) -> Node {
    let node = rcdom::Node::new_typed(
        rcdom::NodeData::Element {
            name: QualName::new(None, ns!(html), LocalName::from(tag_name)),
            attrs: RefCell::new(vec![]),
            template_contents: RefCell::new(None),
            mathml_annotation_xml_integration_point: false,
        },
        typed_properties,
    );
    node.append_children(children);
    node
}

/// Creates a html element.
pub fn html() {}

/// Creates a head element.
pub fn head(children: Vec<Node>) -> Node {
    new_typed_element(
        "head",
        TypedProperties {
            content_model: |_p, c| {
                // check if multiple titles are present
                let multiple_titles_check = c
                    .children()
                    .borrow()
                    .iter()
                    .map(|e| match e.tag_name() {
                        None => false,
                        Some(n) => &n == "title",
                    })
                    .collect::<Vec<bool>>()
                    .iter()
                    .filter(|e| **e == true)
                    .count()
                    > 1;
                vec![
                    ContentModel::Specific(multiple_titles_check),
                    ContentModel::Category(ContentCategory::Metadata),
                ]
            },
            content_categories: |_n| vec![],
        },
        children,
    )
}

/// Creates a title element.
pub fn title(title: &str) -> Node {
    new_typed_element(
        "title",
        TypedProperties {
            content_model: |_p, c| vec![ContentModel::Specific(false)],
            content_categories: |_n| vec![ContentCategory::Text],
        },
        vec![],
    )
}

/// Creates an anchor element.
pub fn a(children: Vec<Node>) -> Node {
    let anchor = new_typed_element(
        "a",
        TypedProperties {
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
    //let anchor = rcdom::Node::new_typed(
    //    rcdom::NodeData::Element {
    //        name: QualName::new(None, ns!(html), LocalName::from("a")),
    //        attrs: RefCell::new(vec![]),
    //        template_contents: RefCell::new(None),
    //        mathml_annotation_xml_integration_point: false,
    //    },
    //    rcdom::TypedProperties {
    //        content_model: |p, c| {
    //            vec![
    //                ContentModel::Specific(
    //                    !c.categories().contains(&ContentCategory::Interactive),
    //                ),
    //                ContentModel::Specific(c.tag_name() != p.tag_name()),
    //                ContentModel::Specific(
    //                    c.find_attribute("tabindex").is_none(),
    //                ),
    //                ContentModel::Category(ContentCategory::Transparent),
    //            ]
    //        },
    //        content_categories: |n| {
    //            use rcdom::ContentCategory::*;
    //            let mut v = vec![Flow, Phrasing, Palpable];
    //            if n.find_attribute("href").is_some() {
    //                v.push(Interactive);
    //            }
    //            v
    //        },
    //    },
    //);
    //anchor.append_children(children);
    anchor
}

/// Creates a text node.
pub fn text(text: &str) -> Node {
    rcdom::Node::new_typed(
        NodeData::Text {
            contents: RefCell::new(Tendril::from(text)),
        },
        TypedProperties {
            content_model: |_p, _c| vec![ContentModel::Specific(false)],
            content_categories: |_n| {
                use rcdom::ContentCategory::*;
                vec![Phrasing]
            },
        },
    )
}
