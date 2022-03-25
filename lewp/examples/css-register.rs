use {
    lewp::{
        config::{ModuleConfig, PageConfig},
        css::{Register as CssRegister, RegisterOptions},
        html::{api::*, Node, Nodes},
        Charset,
        LanguageTag,
        Lewp,
        LewpError,
        Module,
        Modules,
        Page,
        RuntimeInformation,
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

    fn run(&mut self) {
        let module = HelloWorld::new();
        self.add_module(module.into_module_ptr());
    }
}

fn main() {
    let fh = lewp::fh::FileHierarchyBuilder::new()
        .mountpoint(std::path::PathBuf::from("./lewp/testfiles"))
        .build();

    let lewp = Lewp::new()
        .with_file_hierarchy(fh)
        .with_css_register(RegisterOptions::default());

    let mut page = HelloWorldPage {
        modules: vec![],
        config: PageConfig::from(&lewp),
    };
    let dom = page.build();
    println!("{}", dom);
}
