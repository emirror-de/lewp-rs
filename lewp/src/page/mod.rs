//! Traits and data structures to create, run, assemble and render a web page.

use {
    crate::{
        config::PageConfig,
        css::{Entireness, Register as CssRegister},
        fh::{ComponentInformation, ComponentType, FileHierarchy, Level},
        html::{api::*, Document, Node, NodeList},
        module::{ModulePtr, Modules, RuntimeInformation},
        Charset,
        LanguageTag,
    },
    html5ever::serialize,
    markup5ever_rcdom::SerializableHandle,
    std::{rc::Rc, sync::Arc},
};

/// Defines the unique page ID. This ID is used to identify eg. the page resources
/// on the file system.
pub type PageId = str;

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

    /// Returns a reference to the page ID.
    fn id(&self) -> &PageId;

    /// Borrows the head tags that are specific for this page.
    fn head_tags(&self) -> Option<&NodeList> {
        None
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

    /// Runs through the given modules and returns a list containing all module
    /// ids including all submodule ids.
    ///
    /// The list items are unique, so the ids do only occur once.
    fn collect_required_module_ids(&self, modules: &Modules) -> Vec<String> {
        let mut required_ids = vec![];
        for module in modules {
            let module = module.borrow();
            required_ids.push(module.id().to_string());
            if let Some(submodules) = module.submodules() {
                let mut sub_ids = self.collect_required_module_ids(&submodules);
                required_ids.append(&mut sub_ids);
            }
        }
        required_ids.dedup();
        required_ids
    }

    /// Runs recursively through the given modules and collects a [NodeList]
    /// that are required in the `<head>` tag to run all those modules.
    fn collect_head_tags(
        &self,
        modules: &Modules,
        required_module_ids: &mut Vec<String>,
    ) -> NodeList {
        let mut head_tags = NodeList::new();
        let mut inline_css = vec![];

        for module in modules {
            let module = module.borrow();

            if !required_module_ids.contains(&module.id().to_string()) {
                // skip css if already processed
                continue;
            }
            log::debug!("Removing id: {}", module.id());
            // remove the id from the processed array
            required_module_ids.retain(|e| e != module.id());

            // add all head tags for module first
            head_tags.append(&mut module.head_tags().clone());

            // collect all CSS
            if let Some(r) = &self.config().css_register {
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
            // add submodule head tags if available
            if let Some(submodules) = module.submodules() {
                let mut subs =
                    self.collect_head_tags(&submodules, required_module_ids);
                head_tags.append(&mut subs);
            }
        }
        let inline_css = &inline_css
            .into_iter()
            .fold(String::new(), |acc, e| format!("{}{}", acc, *e));

        // create a style tag for inline css
        if !inline_css.is_empty() {
            head_tags.push(style(text(inline_css)));
        }
        head_tags
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

        // add all page specific head tags
        match self.head_tags() {
            Some(h) => head_children.append(&mut h.clone()),
            None => (),
        };

        // collector vec for all inline css styles
        //let mut inline_css = vec![];

        // add page css
        if let Some(r) = &self.config().css_register {
            if let Some(css) = r.query(
                Arc::new(ComponentInformation {
                    id: self.id().to_string(),
                    level: Level::Page,
                    kind: ComponentType::Css,
                }),
                Entireness::Full,
            ) {
                let css = format!("{}", css);
                // create a style tag for inline css
                if !css.is_empty() {
                    head_children.push(style(text(&css)));
                }
            }
        }

        // contains all ids of the modules that have already been processed
        // to prevent duplicate tags/inserts
        let mut module_ids_to_process =
            self.collect_required_module_ids(&self.modules());
        let mut head_tags =
            self.collect_head_tags(&self.modules(), &mut module_ids_to_process);
        head_children.append(&mut head_tags);

        //for module in self.modules() {
        //    let module = module.borrow();
        //    if !module_ids_to_process.contains(&module.id().to_string()) {
        //        continue;
        //    }
        //    processed_module_ids.push(module.id().to_string());

        //    // add all head tags for module first
        //    head_children.append(&mut module.head_tags().clone());

        //    // collect all CSS
        //    if let Some(r) = self.css_register() {
        //        if let Some(css) = r.query(
        //            Arc::new(ComponentInformation {
        //                id: module.id().to_string(),
        //                level: Level::Module,
        //                kind: ComponentType::Css,
        //            }),
        //            Entireness::Full,
        //        ) {
        //            inline_css.push(css.clone());
        //        }
        //    }
        //}
        //let inline_css = &inline_css
        //    .into_iter()
        //    .fold(String::new(), |acc, e| format!("{}{}", acc, *e));

        //// create a style tag for inline css
        //if !inline_css.is_empty() {
        //    head_children.push(style(text(inline_css)));
        //}

        head(head_children)
    }

    /// Assembles the `<body>` tag of the page.
    fn assemble_body(&self, modules: NodeList) -> Node {
        //let body = NodeCreator::element("body", vec![]);
        let mut body_children = vec![];
        for module in modules {
            body_children.push(module);
        }
        body(body_children)
    }

    /// Assembles the full page and returns it as [Document].
    fn assemble_full(&self, modules: NodeList) -> Document {
        let head = self.assemble_head();
        let body = self.assemble_body(modules);
        document(self.language(), head, body)
    }

    /// Renders the page. To a full valid HTML string.
    fn render(&self, modules: NodeList) -> String {
        let mut bytes = vec![];
        let document: SerializableHandle =
            self.assemble_full(modules).document.into();
        serialize(&mut bytes, &document, Default::default()).unwrap();
        String::from_utf8(bytes).unwrap()
    }

    /// Executes all implemented functions and renders the page afterwards.
    fn build(&mut self) -> String {
        self.run();
        let runtime_information =
            Rc::new(RuntimeInformation::new(self.config()));
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
