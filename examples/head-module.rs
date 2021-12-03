use {
    lewp::{
        config::{ModuleConfig, PageConfig},
        dom::{NodeCreator, NodeExt, Nodes},
        module::{Module, Modules, RuntimeInformation},
        page::Page,
        LewpError,
    },
    std::rc::Rc,
};

struct HeadOnly {
    config: ModuleConfig,
    head_tags: Nodes,
}

impl HeadOnly {
    pub fn new() -> Self {
        let mut config = ModuleConfig::new();
        config.skip_wrapper = true;
        Self {
            config,
            head_tags: Self::create_head_tags(),
        }
    }

    fn create_head_tags() -> Nodes {
        let content = String::from(
            "
            document.addEventListener('DOMContentLoaded', () => {
            document.querySelector('#head-only').innerHTML = 
            \"I have been added using JavaScript.\"
            });",
        );
        let script = NodeCreator::element(
            "script",
            vec![NodeCreator::attribute("defer", "defer")],
        );
        script.add_text(&content);
        vec![script]
    }
}

impl Module for HeadOnly {
    fn head_tags(&self) -> &Nodes {
        &self.head_tags
    }

    fn id(&self) -> &str {
        "head-module"
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
        vec![NodeCreator::element(
            "div",
            vec![NodeCreator::attribute("id", "head-only")],
        )]
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
        "Head-only module example!"
    }

    fn description(&self) -> &str {
        "This page shows how to create a head-only module!"
    }

    fn config(&self) -> &PageConfig {
        &self.config
    }

    fn run(&mut self) {
        let module = HeadOnly::new();
        self.add_module(module.into_module_ptr());
    }
}

fn main() {
    let mut page = HelloWorldPage {
        modules: vec![],
        config: PageConfig::new(),
    };
    let dom = page.build();
    println!("{}", dom);
}
