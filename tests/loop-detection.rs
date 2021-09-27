use lewp::{
    config::PageConfig,
    module::Modules,
    page::{
        Assembler, Metadata as PageMetadata, Page, Render as PageRender, Runtime as PageRuntime,
    },
    Charset, LanguageTag,
};

mod modules {
    use lewp::{
        config::ModuleConfig,
        dom::{NodeCreator, Nodes},
        module::{Metadata, Module, Modules, Render, Runtime, RuntimeInformation},
        submodule::{Render as SubModuleRender, Runtime as SubModuleRuntime, SubModule},
        Error,
    };

    pub struct Header {
        config: ModuleConfig,
        head_tags: Nodes,
        children: Modules,
        data: String,
    }

    impl Header {
        pub fn new() -> Self {
            let mut instance = Self {
                config: ModuleConfig::new(),
                head_tags: Nodes::new(),
                children: Modules::new(),
                data: String::from("hello-world"),
            };
            // Recommended way to add a module to have integrated loop prevention
            if instance
                .append_module(Box::new(Self {
                    config: ModuleConfig::new(),
                    head_tags: Nodes::new(),
                    children: Modules::new(),
                    data: String::from("hello-world"),
                }))
                .is_err()
            {
                assert_eq!(true, true);
            }
            instance
        }
    }

    impl Module for Header {
        fn head_tags(&self) -> &Nodes {
            &self.head_tags
        }
    }

    impl Metadata for Header {
        fn id(&self) -> &str {
            "header"
        }

        fn config(&self) -> &ModuleConfig {
            &self.config
        }
    }

    impl Runtime for Header {
        fn run(&mut self, runtime_information: &mut Box<RuntimeInformation>) -> Result<(), Error> {
            // See Runtime trait in submodule for more run methods
            self.run_submodules(runtime_information)?;
            Ok(())
        }
    }

    impl Render for Header {
        fn view(&self) -> Nodes {
            let headline = NodeCreator::headline(1, &self.data);
            let mut view = vec![headline];
            // see Render trait in submodule for more rendering methods
            self.render_submodules(&mut view);
            view
        }
    }

    impl SubModule for Header {
        fn submodules(&self) -> &Modules {
            &self.children
        }

        fn submodules_mut(&mut self) -> &mut Modules {
            &mut self.children
        }
    }

    impl SubModuleRuntime for Header {}
    impl SubModuleRender for Header {}
}

struct HelloWorldPage {
    modules: Modules,
    config: PageConfig,
}

impl Page for HelloWorldPage {
    fn modules(&self) -> &Modules {
        &self.modules
    }
    fn modules_mut(&mut self) -> &mut Modules {
        &mut self.modules
    }
}

impl PageMetadata for HelloWorldPage {
    fn title(&self) -> &str {
        "lewp sub-module demonstration!"
    }

    fn description(&self) -> &str {
        "lewp can have sub-modules!"
    }

    fn language(&self) -> LanguageTag {
        LanguageTag::parse("de-DE").unwrap()
    }

    fn charset(&self) -> Charset {
        Charset::Utf8
    }

    fn config(&self) -> &PageConfig {
        &self.config
    }
}

impl PageRuntime for HelloWorldPage {
    fn run(&mut self) {}
}

impl PageRender for HelloWorldPage {}

impl Assembler for HelloWorldPage {}

#[test]
fn loop_detection() {
    let module = Box::new(modules::Header::new());
    let mut page = HelloWorldPage {
        modules: vec![],
        config: PageConfig::new(),
    };
    page.add_module(module);
    page.execute();
}
