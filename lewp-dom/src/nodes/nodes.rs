use {
    crate::{Document, Node, NodeExt},
    html5ever::{namespace_url, ns, tendril::Tendril, LocalName, QualName},
    langtag::LanguageTag,
    log::error,
    rcdom::NodeData,
    std::cell::RefCell,
};

/// Creates a new HTML document. It contains the doctype and html tag by default.
pub fn document(language: LanguageTag, children: Vec<Node>) -> Document {
    let node = html(language, children);

    // create the document
    let dom = Document::default();
    let doctype = rcdom::Node::new(NodeData::Doctype {
        name: Tendril::from("html"),
        public_id: Tendril::from(""),
        system_id: Tendril::from(""),
    });

    dom.document.children.borrow_mut().push(doctype);
    dom.document.children.borrow_mut().push(node);
    dom
}

/// Creates an `html` tag. automatically created when using [document].
pub fn html(language: LanguageTag, children: Vec<Node>) -> Node {
    let mut html = basic_element("html");

    // process language
    let language = match language.language() {
        Some(v) => v.primary().as_str().to_owned(),
        None => {
            error!("Could not find language for creating <html> tag");
            "".to_owned()
        }
    };
    html = html.attr("lang", &language);

    html = append_children(html, children);
    html
}

/// Creates a new `div` tag node.
pub fn div(children: Vec<Node>) -> Node {
    let node = basic_element("div");
    append_children(node, children)
}

/// Creates a text node.
pub fn text(content: &str) -> Node {
    rcdom::Node::new(NodeData::Text {
        contents: RefCell::new(Tendril::from(content)),
    })
}

/// Helper function to create a tag node.
fn basic_element(tag_name: &str) -> Node {
    rcdom::Node::new(NodeData::Element {
        name: QualName::new(None, ns!(html), LocalName::from(tag_name)),
        attrs: RefCell::new(vec![]),
        template_contents: None,
        mathml_annotation_xml_integration_point: false,
    })
}

/// Appends all children to the given parent node.
fn append_children(parent: Node, children: Vec<Node>) -> Node {
    let mut children = children;
    let mut parent = parent;
    parent.append_children(&mut children);
    parent
}
