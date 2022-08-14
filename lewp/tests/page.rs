use {
    lewp::{
        config::{ModuleConfig, PageConfig},
        html::{api::*, Node, NodeList},
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
    pub config: ModuleConfig,
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

impl From<ModuleConfig> for HelloWorld {
    fn from(config: ModuleConfig) -> Self {
        Self {
            config,
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

    fn run(&mut self) {}
}

const HELLO_WORLD_RESULT: &str = "<!DOCTYPE html><html lang=\"de\"><head><meta charset=\"utf-8\"><title>Hello World from lewp!</title><meta name=\"description\" content=\"My first page using lewp!\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0, user-scalable=no\"></head><body><h1 class=\"hello-world\" data-lewp-component=\"module\">hello-world</h1></body></html>";

#[test]
fn hello_world_page() {
    let module_config = ModuleConfig {};
    let module = HelloWorld::from(module_config);
    let mut page = HelloWorldPage {
        modules: vec![],
        config: PageConfig::default(),
    };
    page.add_module(module.into_module_ptr());
    let html_string = page.build();
    assert_eq!(HELLO_WORLD_RESULT, html_string);
}
