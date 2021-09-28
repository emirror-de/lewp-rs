use {
    crate::{
        dom::{Handle, Node, NodeData},
        Charset, LanguageTag,
    },
    html5ever::{namespace_url, ns, tendril::Tendril, Attribute, LocalName, QualName},
    log::error,
    std::{cell::RefCell, rc::Rc},
};

/// Helper struct for creating nodes.
pub struct NodeCreator;

impl NodeCreator {
    /// Creates a HTML tag. If the third argument is given, a TextNode will be
    /// added as subelement to the tag.
    pub fn element(
        tag_name: &str,
        attributes: Vec<Attribute>,
        content: Option<String>,
    ) -> Rc<Node> {
        let node = Node::new(NodeData::Element {
            name: QualName::new(None, ns!(html), LocalName::from(tag_name)),
            attrs: RefCell::new(attributes),
            template_contents: None,
            mathml_annotation_xml_integration_point: false,
        });

        if content.is_none() {
            return node;
        }

        let content = Self::text(&content.unwrap());
        node.children.borrow_mut().push(content);
        node
    }

    /// Creates a HTML tag. Same as
    /// [NodeCreator::element]
    /// but with more options, see
    /// [markup5ever_rcdom::NodeData::Element].
    pub fn element_ext(
        tag_name: &str,
        attributes: Vec<Attribute>,
        template_contents: Option<Handle>,
        mathml_annotation_xml_integration_point: bool,
    ) -> Rc<Node> {
        Node::new(NodeData::Element {
            name: QualName::new(None, ns!(html), LocalName::from(tag_name)),
            attrs: RefCell::new(attributes),
            template_contents,
            mathml_annotation_xml_integration_point,
        })
    }

    /// Creates an attribute of an HTML element, see [html5ever::interface::Attribute].
    pub fn attribute(name: &str, value: &str) -> Attribute {
        Attribute {
            name: QualName::new(None, ns!(), LocalName::from(name)),
            value: Tendril::from(value),
        }
    }

    /// Creates a HTML text node.
    pub fn text(content: &str) -> Rc<Node> {
        Node::new(NodeData::Text {
            contents: RefCell::new(Tendril::from(content)),
        })
    }

    /// Creates the `<!doctype html>` tag node.
    pub fn doctype_html() -> Rc<Node> {
        Node::new(NodeData::Doctype {
            name: Tendril::from("html"),
            public_id: Tendril::from(""),
            system_id: Tendril::from(""),
        })
    }

    /// Creates a `<html>` tag node.
    pub fn html(language: LanguageTag) -> Rc<Node> {
        let language = match language.language() {
            Some(v) => v.primary().as_str().to_owned(),
            None => {
                error!("Could not find language for creating <html> tag");
                "".to_owned()
            }
        };
        NodeCreator::element(
            "html",
            vec![NodeCreator::attribute("lang", &language)],
            None,
        )
    }

    /// Creates a `<title>` tag node.
    pub fn title(title: &str) -> Rc<Node> {
        let title_tag = NodeCreator::element("title", vec![], None);
        title_tag
            .children
            .borrow_mut()
            .push(NodeCreator::text(title));
        title_tag
    }

    /// Creates a `<meta>` tag node with description.
    pub fn description(description: &str) -> Rc<Node> {
        NodeCreator::element(
            "meta",
            vec![
                NodeCreator::attribute("name", "description"),
                NodeCreator::attribute("content", description),
            ],
            None,
        )
    }

    /// Creates a `<meta>` viewport tag node.
    pub fn viewport() -> Rc<Node> {
        NodeCreator::element(
            "meta",
            vec![
                NodeCreator::attribute("name", "viewport"),
                NodeCreator::attribute(
                    "content",
                    "width=device-width, initial-scale=1.0, user-scalable=no",
                ),
            ],
            None,
        )
    }

    /// Creates a `<meta>` charset tag node.
    pub fn charset(charset: &Charset) -> Rc<Node> {
        NodeCreator::element(
            "meta",
            vec![NodeCreator::attribute("charset", &charset.to_string())],
            None,
        )
    }

    /// Creates a `hX` tag, where `X` equals the given level.
    /// If the level is bigger than 6, it will be set to 6.
    /// If 0 is given, 1 is used.
    pub fn headline(level: u8, content: &str, attributes: Vec<Attribute>) -> Rc<Node> {
        let level = match level {
            0 => 1,
            1..=6 => level,
            _ => 6,
        };
        Self::element(
            &format!("h{}", level),
            attributes,
            Some(content.to_string()),
        )
    }

    /// Creates a `p` tag with the given attributes and content.
    pub fn paragraph(content: &str, attributes: Vec<Attribute>) -> Rc<Node> {
        Self::element("p", attributes, Some(content.to_string()))
    }
}
