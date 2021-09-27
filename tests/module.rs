use lewp::{
    config::ModuleConfig,
    dom::{NodeCreator, Nodes},
    module::{Metadata, Module, Modules, Render, Runtime, RuntimeInformation},
    Error,
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

#[test]
fn hello_world() {
    let mut module = HelloWorld::new();
    match module.run(&RuntimeInformation::new()) {
        _ => (),
    }
}
