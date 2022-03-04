//! Traits and data structures to create, run, assemble and render a web page.

use {
    crate::{
        config::PageConfig,
        css::{Entireness, Register as CssRegister},
        dom::{Node, NodeCreator, Nodes, RcDom},
        fh::{ComponentInformation, ComponentType, Level},
        module::{ModulePtr, Modules, RuntimeInformation},
        Charset, LanguageTag,
    },
    html5ever::{serialize, serialize::SerializeOpts},
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
    fn assemble_head(&self) -> Rc<Node> {
        let head = NodeCreator::element("head", vec![]);
        head.children
            .borrow_mut()
            .push(NodeCreator::charset(&self.charset()));
        if self.config().viewport_tag {
            head.children.borrow_mut().push(NodeCreator::viewport());
        }
        head.children
            .borrow_mut()
            .push(NodeCreator::title(self.title()));
        head.children
            .borrow_mut()
            .push(NodeCreator::description(self.description()));

        // collector vec for all inline css styles
        let mut inline_css = vec![];

        for module in self.modules() {
            let module = module.borrow();
            for head_tag in module.head_tags() {
                head.children.borrow_mut().push(head_tag.clone());
            }
            // collect all CSS
            if let Some(r) = self.css_register() {
                if let Some(css) = r.query(
                    Rc::new(ComponentInformation {
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

        // create a style tag for inline css
        let node = NodeCreator::element("style", vec![]);
        let css_text = NodeCreator::text(
            &inline_css
                .into_iter()
                .fold(String::new(), |acc, e| format!("{}{}", acc, *e)),
        );
        node.children.borrow_mut().push(css_text);
        head.children.borrow_mut().push(node);

        head
    }

    /// Assembles the `<body>` tag of the page.
    fn assemble_body(&self, modules: Vec<Nodes>) -> Rc<Node> {
        let body = NodeCreator::element("body", vec![]);
        for module in modules {
            for node in module {
                body.children.borrow_mut().push(node);
            }
        }
        body
    }

    /// Assembles the full page and returns it as [RcDom].
    fn assemble_full(&self, modules: Vec<Nodes>) -> RcDom {
        let dom = RcDom::default();
        let doctype = NodeCreator::doctype_html();

        let html = NodeCreator::html(self.language());

        let head = self.assemble_head();
        let body = self.assemble_body(modules);

        html.children.borrow_mut().push(head);
        html.children.borrow_mut().push(body);

        dom.document.children.borrow_mut().push(doctype);
        dom.document.children.borrow_mut().push(html);
        dom
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
        let runtime_information = Rc::new(RuntimeInformation::new());
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
