macro_rules! typed_node {
    (
        $(#[$outer:meta])*
        $name:ident
        $tag_name:expr
    ) => {
        $(#[$outer])*
        pub struct $name {
            children: std::cell::RefCell<Vec<Self>>,
            data: rcdom::NodeData,
        }

        impl $name {
            /// Creates a new instance.
            pub fn new() -> Self {
                use {
                    html5ever::{namespace_url, ns, LocalName, QualName},
                    std::cell::RefCell,
                };
                Self {
                    children: RefCell::new(vec![]),
                    data: rcdom::NodeData::Element {
                        name: QualName::new(None, ns!(html), LocalName::from($tag_name)),
                        attrs: RefCell::new(vec![]),
                        template_contents: None,
                        mathml_annotation_xml_integration_point: false,
                    }
                }
            }
        }

        impl NodeExt for $name {
            fn data(&self) -> &rcdom::NodeData {
                &self.data
            }
            fn children<T: NodeExt>(&self) -> &std::cell::RefCell<Vec<T>> {
                &self.children
            }
        }
    };
}
