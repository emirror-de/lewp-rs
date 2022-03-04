use {
    lewp::{
        config::{ModuleConfig, PageConfig},
        html::{api::*, Nodes},
        module::{Module, Modules, RuntimeInformation},
        page::Page,
        Charset,
        LanguageTag,
        LewpError,
    },
    std::rc::Rc,
};

struct HelloWorld {
    pub config: ModuleConfig,
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
        vec![h1(vec![text(&self.data)])]
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

const HELLO_WORLD_RESULT: &str = "<!DOCTYPE html><html lang=\"de\"><head><meta charset=\"utf-8\"><title>Hello World from lewp!</title><meta name=\"description\" content=\"My first page using lewp!\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0, user-scalable=no\"></head><body><div class=\"hello-world\" data-lewp-component=\"module\"><h1>hello-world</h1></div></body></html>";
const HELLO_WORLD_RESULT_SKIPPED_WRAPPER: &str = "<!DOCTYPE html><html lang=\"de\"><head><meta charset=\"utf-8\"><title>Hello World from lewp!</title><meta name=\"description\" content=\"My first page using lewp!\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0, user-scalable=no\"></head><body><h1>hello-world</h1></body></html>";

#[test]
fn hello_world_with_module_wrapper() {
    let module = HelloWorld::new();
    let mut page = HelloWorldPage {
        modules: vec![],
        config: PageConfig::new(),
    };
    page.add_module(module.into_module_ptr());
    let html_string = page.build();
    assert_eq!(HELLO_WORLD_RESULT, html_string);
}

#[test]
fn hello_world_skipped_wrapper() {
    let module_config = ModuleConfig { skip_wrapper: true };
    let module = HelloWorld::from(module_config);
    let mut page = HelloWorldPage {
        modules: vec![],
        config: PageConfig::new(),
    };
    page.add_module(module.into_module_ptr());
    let html_string = page.build();
    assert_eq!(HELLO_WORLD_RESULT_SKIPPED_WRAPPER, html_string);
}
