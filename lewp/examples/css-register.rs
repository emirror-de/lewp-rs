use {
    lewp::{
        config::{ModuleConfig, PageConfig},
        css::{Register as CssRegister, RegisterOptions},
        dom::{NodeCreator, Nodes},
        module::{Module, Modules, RuntimeInformation},
        page::Page,
        Charset, LanguageTag, LewpError,
    },
    std::{rc::Rc, sync::Arc},
};

struct HelloWorld {
    config: ModuleConfig,
    head_tags: Nodes,
    data: String,
}

impl HelloWorld {
    pub fn new() -> Self {
        Self {
            config: ModuleConfig::new(),
            head_tags: vec![],
            data: String::from("hello-world"),
        }
    }
}

impl Module for HelloWorld {
    fn head_tags(&self) -> &Nodes {
        &self.head_tags
    }

    fn id(&self) -> &str {
        "hello-world"
    }

    fn config(&self) -> &ModuleConfig {
        &self.config
    }

    fn run(
        &mut self,
        _runtime_info: Rc<RuntimeInformation>,
    ) -> Result<(), LewpError> {
        Ok(())
    }

    fn view(&self) -> Nodes {
        let headline = NodeCreator::headline(1, &self.data, vec![]);
        vec![headline]
    }
}

struct HelloWorldPage {
    modules: Modules,
    config: PageConfig,
    css_register: Arc<CssRegister>,
}

impl Page for HelloWorldPage {
    fn modules(&self) -> &Modules {
        &self.modules
    }
    fn modules_mut(&mut self) -> &mut Modules {
        &mut self.modules
    }

    fn title(&self) -> &str {
        "Hello World from lewp!"
    }

    fn description(&self) -> &str {
        "My first page using lewp!"
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

    fn run(&mut self) {
        let module = HelloWorld::new();
        self.add_module(module.into_module_ptr());
    }

    fn css_register(&self) -> Option<Arc<CssRegister>> {
        Some(self.css_register.clone())
    }
}

fn main() {
    let fh = lewp::fh::FileHierarchyBuilder::new()
        .mountpoint(std::path::PathBuf::from("./lewp/testfiles"))
        .build();

    let mut css_register = CssRegister::new(fh, RegisterOptions::new());
    css_register.load_process_components().unwrap();
    let css_register = Arc::new(css_register);

    let mut page = HelloWorldPage {
        modules: vec![],
        config: PageConfig::new(),
        css_register,
    };
    let dom = page.build();
    println!("{}", dom);
}
