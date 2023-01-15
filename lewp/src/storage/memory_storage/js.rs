use {
    super::{
        super::{ResourceType, Storage},
        Level,
        MemoryStorage,
        StorageComponent,
        StorageRegister,
    },
    crate::{component::ComponentId, resources::Js},
    std::{collections::HashMap, sync::Arc},
};

/// Query options for JavaScript in a [`MemoryStorage<Js>`].
#[derive(Default)]
pub struct JsQueryOptions;

impl StorageRegister for MemoryStorage<Js> {
    type Options = ();
    type QueryOptions = JsQueryOptions;
    type Content = String;
    fn initialize<S: Storage>(options: Self::Options) -> anyhow::Result<Self> {
        log::debug!("Creating new JS memory storage with options: {options:?}");
        let mut register = Self {
            register: HashMap::new(),
        };
        register.load_process_components::<S>()?;
        Ok(register)
    }
    fn query(
        &self,
        id: ComponentId,
        level: Level,
        _options: Self::QueryOptions,
    ) -> Option<Arc<Self::Content>> {
        Some(Arc::clone(self.register.get(&(id, level))?))
    }
    fn options(&self) -> &Self::Options {
        &()
    }
}

impl MemoryStorage<Js> {
    /// Collects, processes and caches all available JS in the file hierarchy.
    pub fn load_process_components<S: Storage>(
        &mut self,
    ) -> anyhow::Result<()> {
        self.load_process_modules::<S>()?;
        self.load_process_pages::<S>()
    }

    fn load_process_modules<S: Storage>(&mut self) -> anyhow::Result<()> {
        let module_ids = S::collect_component_ids(
            ResourceType::JavaScript,
            Level::Component,
        )?;
        for id in module_ids {
            let c = Js::new(id.clone(), Level::Component);
            let c = match c.content::<S>(()) {
                Ok(c) => c,
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Could not get minified JavaScript: {e}",
                    ))
                }
            };
            self.register.insert((id.clone(), Level::Component), c);
        }
        Ok(())
    }

    fn load_process_pages<S: Storage>(&mut self) -> anyhow::Result<()> {
        let page_ids =
            S::collect_component_ids(ResourceType::JavaScript, Level::Page)?;
        for id in page_ids {
            let c = Js::new(id.clone(), Level::Page);
            let c = match c.content::<S>(()) {
                Ok(c) => c,
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Could not get minified JavaScript: {e}",
                    ))
                }
            };
            self.register.insert((id.clone(), Level::Page), c);
        }
        Ok(())
    }
}
