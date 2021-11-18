use lewp::{
    config::PageConfig,
    module::{Module, Modules},
    page::{
        Assembler,
        Metadata as PageMetadata,
        Page,
        Render as PageRender,
        Runtime as PageRuntime,
    },
    Charset,
    LanguageTag,
};

mod modules {
    use {
        lewp::{
            config::ModuleConfig,
            dom::{NodeCreator, Nodes},
            module::{
                Metadata,
                Module,
                Modules,
                Render,
                Runtime,
                RuntimeInformation,
            },
            submodule::{
                Render as SubModuleRender,
                Runtime as SubModuleRuntime,
                SubModule,
            },
            LewpError,
        },
        std::rc::Rc,
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
            let headline = RandomHeadline::new();
            // Recommended way to add a module to have integrated loop prevention
            if instance.append_module(headline.into_module_ptr()).is_err() {
                log::error!("Could not append module!");
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
        fn run(
            &mut self,
            runtime_information: Rc<RuntimeInformation>,
        ) -> Result<(), LewpError> {
            // See Runtime trait in submodule for more run methods
            self.run_submodules(runtime_information)?;
            Ok(())
        }
    }

    impl Render for Header {
        fn view(&self) -> Nodes {
            let headline = NodeCreator::headline(1, &self.data, vec![]);
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

    pub struct RandomHeadline {
        config: ModuleConfig,
        head_tags: Nodes,
        data: String,
    }

    impl RandomHeadline {
        pub fn new() -> Self {
            Self {
                config: ModuleConfig::new(),
                head_tags: Nodes::new(),
                data: String::from("Wow this is dynamic!"),
            }
        }
    }

    impl Module for RandomHeadline {
        fn head_tags(&self) -> &Nodes {
            &self.head_tags
        }
    }

    impl Metadata for RandomHeadline {
        fn id(&self) -> &str {
            "random-headline"
        }

        fn config(&self) -> &ModuleConfig {
            &self.config
        }
    }

    impl Runtime for RandomHeadline {
        fn run(
            &mut self,
            _runtime_info: Rc<RuntimeInformation>,
        ) -> Result<(), LewpError> {
            self.data = String::from("Changed during run!");
            Ok(())
        }
    }

    impl Render for RandomHeadline {
        fn view(&self) -> Nodes {
            vec![NodeCreator::headline(2, &self.data, vec![])]
        }
    }
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

const SUBMODULE_RESULT: &str = "<!DOCTYPE html><html lang=\"de\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0, user-scalable=no\"><title>lewp sub-module demonstration!</title><meta name=\"description\" content=\"lewp can have sub-modules!\"></head><body><div class=\"lewp-module header\"><h1>hello-world</h1><div class=\"lewp-module random-headline\"><h2>Changed during run!</h2></div></div></body></html>";

#[test]
fn submodule() {
    let module = modules::Header::new();
    let mut page = HelloWorldPage {
        modules: vec![],
        config: PageConfig::new(),
    };
    page.add_module(module.into_module_ptr());
    let dom = page.execute();
    assert_eq!(SUBMODULE_RESULT, dom);
}
