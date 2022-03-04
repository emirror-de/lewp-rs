use lewp::{config::PageConfig, Charset, LanguageTag, Module, Modules, Page};

mod modules {
    use {
        lewp::{
            config::ModuleConfig,
            html::{api::*, Nodes},
            LewpError,
            Module,
            Modules,
            RuntimeInformation,
            SubModule,
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
            // See Runtime trait in submodule for more run methods
            self.run_submodules(runtime_information)?;
            Ok(())
        }

        fn view(&self) -> Nodes {
            let mut view = vec![h1(vec![text(&self.data)])];
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

        fn id(&self) -> &str {
            "random-headline"
        }

        fn config(&self) -> &ModuleConfig {
            &self.config
        }

        fn run(
            &mut self,
            _runtime_info: Rc<RuntimeInformation>,
        ) -> Result<(), LewpError> {
            self.data = String::from("Changed during run!");
            Ok(())
        }

        fn view(&self) -> Nodes {
            vec![h2(vec![text(&self.data)])]
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

    fn run(&mut self) {}
}

const SUBMODULE_RESULT: &str = "<!DOCTYPE html><html lang=\"de\"><head><meta charset=\"utf-8\"><title>lewp sub-module demonstration!</title><meta name=\"description\" content=\"lewp can have sub-modules!\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0, user-scalable=no\"></head><body><div class=\"header\" data-lewp-component=\"module\"><h1>hello-world</h1><div class=\"random-headline\" data-lewp-component=\"module\"><h2>Changed during run!</h2></div></div></body></html>";

#[test]
fn submodule() {
    let module = modules::Header::new();
    let mut page = HelloWorldPage {
        modules: vec![],
        config: PageConfig::new(),
    };
    page.add_module(module.into_module_ptr());
    let dom = page.build();
    assert_eq!(SUBMODULE_RESULT, dom);
}
