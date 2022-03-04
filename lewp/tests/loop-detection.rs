use lewp::{
    config::PageConfig,
    module::{Module, Modules},
    page::Page,
    Charset,
    LanguageTag,
};

mod modules {
    use {
        lewp::{
            config::ModuleConfig,
            html::{api::*, Nodes},
            module::{Module, Modules, RuntimeInformation},
            submodule::SubModule,
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
            // Recommended way to add a module to have integrated loop prevention
            if instance
                .append_module(
                    Self {
                        config: ModuleConfig::new(),
                        head_tags: Nodes::new(),
                        children: Modules::new(),
                        data: String::from("hello-world"),
                    }
                    .into_module_ptr(),
                )
                .is_err()
            {
                assert_eq!(true, true);
            } else {
                assert_eq!(true, false);
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

#[test]
fn loop_detection() {
    let module = modules::Header::new();
    let mut page = HelloWorldPage {
        modules: vec![],
        config: PageConfig::new(),
    };
    page.add_module(module.into_module_ptr());
    page.build();
}
