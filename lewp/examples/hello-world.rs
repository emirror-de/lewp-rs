use {
    lewp::{
        config::{ModuleConfig, PageConfig},
        html::{
            api::{h1, text},
            Node,
            NodeList,
        },
        Charset,
        LanguageTag,
        LewpError,
        Module,
        Modules,
        Page,
        RuntimeInformation,
    },
    std::rc::Rc,
};

struct HelloWorld {
    config: ModuleConfig,
    head_tags: NodeList,
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
    fn head_tags(&self) -> &NodeList {
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

    fn view(&self) -> Node {
        h1(vec![text(&self.data)])
    }
}

struct HelloWorldPage {
    modules: Modules,
    config: PageConfig,
}

impl HelloWorldPage {
    /// Creates a new page.
    pub fn new(config: PageConfig) -> Self {
        Self {
            modules: vec![],
            config,
        }
    }
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
        self.add_module(HelloWorld::new().into_module_ptr());
    }
}

fn main() {
    let mut page = HelloWorldPage::new(PageConfig::default());
    println!("{}", page.build());
}
