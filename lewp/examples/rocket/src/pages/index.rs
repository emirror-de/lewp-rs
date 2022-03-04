use {
    crate::modules::header::Header,
    lewp::{
        config::PageConfig,
        module::{Module, Modules},
        page::Page,
        LanguageTag,
    },
};

pub struct Index {
    modules: Modules,
    config: PageConfig,
}

impl Index {
    pub fn new() -> Self {
        Self {
            modules: Modules::new(),
            config: PageConfig::new(),
        }
    }
}

impl Page for Index {
    fn modules(&self) -> &Modules {
        &self.modules
    }
    fn modules_mut(&mut self) -> &mut Modules {
        &mut self.modules
    }

    fn title(&self) -> &str {
        "Welcome to the lewp pure HTML5 dashboard."
    }

    fn description(&self) -> &str {
        "This page demonstrates the beauty and power of lewp websites!"
    }

    fn language(&self) -> LanguageTag {
        LanguageTag::parse("de-DE").unwrap()
    }

    fn config(&self) -> &PageConfig {
        &self.config
    }

    fn run(&mut self) {
        let header = Header::new();
        self.add_module(header.into_module_ptr());
    }
}
