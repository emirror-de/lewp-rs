use lewp::{config::PageConfig, Charset, LanguageTag, Module, Modules, Page};

mod modules {
    use {
        lewp::{
            config::ModuleConfig,
            LewpError,
            Module,
            Modules,
            RuntimeInformation,
        },
        lewp_html::{api::*, Node, NodeExt, Nodes},
        std::sync::Arc,
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
            let headline = RandomHeadline::new().into_module_ptr();
            // Recommended way to add a module to have integrated loop prevention
            if instance.append_module(headline.clone()).is_err() {
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
            runtime_information: Arc<RuntimeInformation>,
        ) -> Result<(), LewpError> {
            // See Runtime trait in submodule for more run methods
            self.run_submodules(runtime_information)?;
            Ok(())
        }

        fn view(&self) -> Node {
            let mut view = div(vec![h1(vec![text(&self.data)])]);
            let mut submodules = vec![];
            // see Render trait in submodule for more rendering methods
            self.render_submodules(&mut submodules);
            view.append_children(submodules);
            view
        }

        fn submodules(&self) -> Option<&Modules> {
            Some(&self.children)
        }

        fn submodules_mut(&mut self) -> Option<&mut Modules> {
            Some(&mut self.children)
        }
    }

    pub struct RandomHeadline {
        config: ModuleConfig,
        head_tags: Nodes,
        current_headline: Option<usize>,
        execution_count: u32,
        data: Vec<String>,
    }

    impl RandomHeadline {
        pub fn new() -> Self {
            Self {
                config: ModuleConfig::new(),
                head_tags: Nodes::new(),
                current_headline: None,
                execution_count: 0,
                data: vec![
                    String::from("My first generated headline"),
                    String::from("Wow this is dynamic!"),
                ],
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
            runtime_information: Arc<RuntimeInformation>,
        ) -> Result<(), LewpError> {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            self.current_headline = Some(rng.gen_range(0..self.data.len()));
            self.execution_count =
                runtime_information.get_execution_count(self.id());
            Ok(())
        }

        fn view(&self) -> Node {
            let headline = match self.current_headline {
                Some(v) => h2(vec![text(&self.data[v])]),
                None => h2(vec![text("This module did not run yet!")]),
            };
            let p = p(vec![text(&format!(
                "Has been executed {} times before!",
                self.execution_count
            ))]);
            div(vec![headline, p])
        }
    }
}

struct HelloWorldPage {
    modules: Modules,
    config: PageConfig,
}

impl Page for HelloWorldPage {
    fn id(&self) -> &str {
        "helloworldpage"
    }

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

fn main() {
    let module = modules::Header::new();
    let mut page = HelloWorldPage {
        modules: vec![],
        config: PageConfig::new(None),
    };
    page.add_module(module.into_module_ptr());
    let dom = page.build();
    println!("{}", dom);
}
