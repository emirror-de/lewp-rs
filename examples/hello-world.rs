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
    config: ModuleConfig,
    head_tags: Nodes,
    children: Modules,
    data: String,
}

impl HelloWorld {
    pub fn new() -> Self {
        Self {
            config: ModuleConfig::new(),
            head_tags: vec![],
            children: vec![],
            data: String::from("hello-world"),
        }
    }
}

impl Module for HelloWorld {
    fn head_tags(&self) -> &Nodes {
        &self.head_tags
    }

    fn children(&self) -> &Modules {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Modules {
        &mut self.children
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

fn main() {
    let module = Rc::new(HelloWorld::new());
    let mut page = HelloWorldPage {
        modules: vec![],
        config: PageConfig::new(),
    };
    page.add_module(module);
    let dom = page.execute();
    println!("{}", dom);
}
