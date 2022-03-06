//! Traits and data structures to create, run, assemble and render a web page.

use {
    crate::{
        config::PageConfig,
        css::{Entireness, Register as CssRegister},
        fh::{ComponentInformation, ComponentType, Level},
        module::{ModulePtr, Modules, RuntimeInformation},
        Charset,
        LanguageTag,
    },
    html5ever::{serialize, serialize::SerializeOpts},
    lewp_html::{api::*, Document, Node, Nodes},
    markup5ever_rcdom::SerializableHandle,
    std::{rc::Rc, sync::Arc},
};

/// Main trait of a page.
pub trait Page {
    /// Should point to a member of type [Modules] in the implementing struct.
    ///
    /// This is the main storage of the modules that are added to the page using
    /// [add_module](Self::add_module).
    fn modules(&self) -> &Modules;

    /// The mutable version of the [modules](Self::modules) method. Is used by [add_module](Self::add_module) method
    /// to add modules to the page while the [run](Self::run) method is being executed.
    fn modules_mut(&mut self) -> &mut Modules;

    /// Adds the module to the page. The page is rendered FIFO.
    fn add_module(&mut self, module: ModulePtr) {
        self.modules_mut().push(module);
    }

    /// Title of the page. Will land in the `title` tag.
    fn title(&self) -> &str;

    /// The page description. Will land in a `meta` tag in the `head` of a page.
    fn description(&self) -> &str;

    /// Language of the page. Will be added as the `lang` attribute to the `doctype`.
    fn language(&self) -> LanguageTag {
        LanguageTag::parse("en-US").unwrap()
    }

    /// Contains the charset of the HTML page. Will be added to the `head` of the page.
    fn charset(&self) -> Charset {
        Charset::Utf8
    }

    /// Returns a reference to the page config.
    fn config(&self) -> &PageConfig;

    /// Executes the page. Main function that is able to collect and modify
    /// data as well as modules required for rendering.
    /// This should contain all required logic for the resulting page, eg. adding
    /// modules, collecting data etc.
    fn run(&mut self);

    /// *Optional:* A [register](crate::css::Register) holding all available CSS
    /// for [modules](Module) and [pages](Page).
    ///
    /// If implemented, [lewp](crate) will automatically add the required tags to the
    /// head of the page.
    ///
    /// *Defaults to None*
    fn css_register(&self) -> Option<Arc<CssRegister>> {
        None
    }

    /// Assembles the `<head>` tag of the page.
    fn assemble_head(&self) -> Node {
        let mut head_children = vec![
            charset(&self.charset()),
            title(self.title()),
            description(self.description()),
        ];
        if self.config().viewport_tag {
            head_children.push(viewport())
        }

        // collector vec for all inline css styles
        let mut inline_css = vec![];

        for module in self.modules() {
            let module = module.borrow();
            // add all head tags for module first
            let mut module_head_tags = module
                .head_tags()
                .iter()
                .map(|tag| tag.clone())
                .collect::<Vec<Node>>();
            head_children.append(&mut module_head_tags);

            // collect all CSS
            if let Some(r) = self.css_register() {
                if let Some(css) = r.query(
                    Arc::new(ComponentInformation {
                        id: module.id().to_string(),
                        level: Level::Module,
                        kind: ComponentType::Css,
                    }),
                    Entireness::Full,
                ) {
                    inline_css.push(css.clone());
                }
            }
        }
        let inline_css = &inline_css
            .into_iter()
            .fold(String::new(), |acc, e| format!("{}{}", acc, *e));

        // create a style tag for inline css
        if !inline_css.is_empty() {
            head_children.push(style(text(inline_css)));
        }

        head(head_children)
    }

    /// Assembles the `<body>` tag of the page.
    fn assemble_body(&self, modules: Vec<Nodes>) -> Node {
        //let body = NodeCreator::element("body", vec![]);
        let mut body_children = vec![];
        for module in modules {
            for node in module {
                body_children.push(node);
            }
        }
        body(body_children)
    }

    /// Assembles the full page and returns it as [Document].
    fn assemble_full(&self, modules: Vec<Nodes>) -> Document {
        let head = self.assemble_head();
        let body = self.assemble_body(modules);
        document(self.language(), head, body)
    }

    /// Renders the page. To a full valid HTML string.
    fn render(&self, modules: Vec<Nodes>) -> String {
        let mut bytes = vec![];
        let document: SerializableHandle =
            self.assemble_full(modules).document.into();
        serialize(&mut bytes, &document, SerializeOpts::default()).unwrap();
        String::from_utf8(bytes).unwrap()
    }

    /// Executes all implemented functions and renders the page afterwards.
    fn build(&mut self) -> String {
        self.run();
        let runtime_information = Arc::new(RuntimeInformation::new());
        let mut modules_rendered_dom = vec![];
        // all modules
        for module in self.modules() {
            let mut module = module.borrow_mut();
            // run
            if let Err(e) = module.run(runtime_information.clone()) {
                log::error!("{}", e);
            }
            // render
            modules_rendered_dom.push(module.render());

            // update runtime information
            runtime_information.increase_execution_count(module.id());
            runtime_information.set_previous_module_id(module.id());
        }
        self.render(modules_rendered_dom)
    }
}
