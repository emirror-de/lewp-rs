use {
    lewp::{
        config::ModuleConfig,
        dom::{NodeCreator, NodeExt, Nodes},
        module::{Module, RuntimeInformation},
        LewpError,
    },
    std::rc::Rc,
};

pub struct Header {
    config: ModuleConfig,
    head_tags: Nodes,
    headline: String,
    subtitle: String,
}

impl Header {
    pub fn new() -> Self {
        let mut instance = Self {
            config: ModuleConfig::new(),
            head_tags: Nodes::new(),
            headline: String::from("Lewp HTML5 dashboard!"),
            subtitle: String::from("I am a fully isolated module system!"),
        };
        instance.setup_head_tags();
        instance
    }

    fn setup_head_tags(&mut self) {
        let css_node = NodeCreator::element(
            "link",
            vec![
                NodeCreator::attribute("href", "/css/header"),
                NodeCreator::attribute("rel", "stylesheet"),
                NodeCreator::attribute("type", "text/css"),
                NodeCreator::attribute("defer", "defer"),
            ],
        );
        self.head_tags.push(css_node);
    }
}

impl Module for Header {
    fn head_tags(&self) -> &Nodes {
        &self.head_tags
    }

    fn id(&self) -> &str {
        "header"
    }

    fn config(&self) -> &ModuleConfig {
        &self.config
    }

    fn run(
        &mut self,
        runtime_information: Rc<RuntimeInformation>,
    ) -> Result<(), LewpError> {
        Ok(())
    }

    fn view(&self) -> Nodes {
        let header = NodeCreator::element("header", vec![]);
        let headline = NodeCreator::headline(1, &self.headline, vec![]);
        let subtitle = NodeCreator::headline(
            2,
            &self.subtitle,
            vec![NodeCreator::attribute("class", "subtitle")],
        );
        subtitle.add_class("title");
        subtitle.remove_class("subtitle");
        header.append_children(&mut vec![headline, subtitle]);
        //header.children.borrow_mut().push(headline);
        //header.children.borrow_mut().push(subtitle);
        vec![header]
    }
}
