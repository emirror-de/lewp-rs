use {
    lewp::{
        config::{ModuleConfig, PageConfig},
        dom::{NodeCreator, Nodes},
        module::{Metadata, Module, Modules, Render, Runtime, RuntimeInformation},
        page::{
            Assembler, Metadata as PageMetadata, Page, Render as PageRender, Runtime as PageRuntime,
        },
        Charset, Error, LanguageTag,
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
}

impl Metadata for HelloWorld {
    fn id(&self) -> &str {
        "hello-world"
    }

    fn config(&self) -> &ModuleConfig {
        &self.config
    }
}

impl Runtime for HelloWorld {
    fn run(&mut self, _runtime_info: &RuntimeInformation) -> Result<(), Error> {
        Ok(())
    }
}

impl Render for HelloWorld {
    fn view(&self) -> Nodes {
        let headline = NodeCreator::headline(1, &self.data);
        vec![headline]
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
}

impl PageRuntime for HelloWorldPage {
    fn run(&mut self) {}
}

impl PageRender for HelloWorldPage {}

impl Assembler for HelloWorldPage {}

const HELLO_WORLD_RESULT: &'static str = "<!DOCTYPE html><html lang=\"de\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0, user-scalable=no\"><title>Hello World from lewp!</title><meta name=\"description\" content=\"My first page using lewp!\"></head><body><div class=\"lewp-module hello-world\"><h1>hello-world</h1></div></body></html>";
const HELLO_WORLD_RESULT_SKIPPED_WRAPPER: &'static str = "<!DOCTYPE html><html lang=\"de\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0, user-scalable=no\"><title>Hello World from lewp!</title><meta name=\"description\" content=\"My first page using lewp!\"></head><body><h1>hello-world</h1></body></html>";

#[test]
fn hello_world_with_module_wrapper() {
    let module = Rc::new(HelloWorld::new());
    let mut page = HelloWorldPage {
        modules: vec![],
        config: PageConfig::new(),
    };
    page.add_module(module);
    let html_string = page.execute();
    assert_eq!(HELLO_WORLD_RESULT, html_string);
}

#[test]
fn hello_world_skipped_wrapper() {
    let module_config = ModuleConfig { skip_wrapper: true };
    let module = Rc::new(HelloWorld::from(module_config));
    let mut page = HelloWorldPage {
        modules: vec![],
        config: PageConfig::new(),
    };
    page.add_module(module);
    let html_string = page.execute();
    assert_eq!(HELLO_WORLD_RESULT_SKIPPED_WRAPPER, html_string);
}