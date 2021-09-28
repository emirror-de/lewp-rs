use {
    lewp::{
        config::ModuleConfig,
        dom::{NodeCreator, Nodes},
        module::{Metadata, Module, Render, Runtime, RuntimeInformation},
        Error,
    },
    std::rc::Rc,
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
    fn run(&mut self, _runtime_info: Rc<RuntimeInformation>) -> Result<(), Error> {
        Ok(())
    }
}

impl Render for HelloWorld {
    fn view(&self) -> Nodes {
        let headline = NodeCreator::headline(1, &self.data, vec![]);
        vec![headline]
    }
}

#[test]
fn hello_world() {
    let mut module = HelloWorld::new();
    match module.run(Rc::new(RuntimeInformation::new())) {
        _ => (),
    }
}
