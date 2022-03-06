use {
    lewp::{
        config::ModuleConfig,
        html::{api::*, Nodes},
        LewpError,
        Module,
        RuntimeInformation,
    },
    std::sync::Arc,
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
        _runtime_info: Arc<RuntimeInformation>,
    ) -> Result<(), LewpError> {
        Ok(())
    }

    fn view(&self) -> Nodes {
        vec![h1(vec![text(&self.data)])]
    }
}

#[test]
fn hello_world() {
    let mut module = HelloWorld::new();
    match module.run(Arc::new(RuntimeInformation::new())) {
        _ => (),
    }
}
