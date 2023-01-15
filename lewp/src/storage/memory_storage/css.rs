use {
    super::{
        super::{ResourceType, Storage},
        Level,
        MemoryStorage,
        StorageComponent,
        StorageRegister,
    },
    crate::{
        component::ComponentId,
        resources::{css::Entireness, Css},
    },
    std::{collections::HashMap, sync::Arc},
};

/// Options when querying a [`MemoryStorage<Css>`].
#[derive(Default)]
pub struct CssQueryOptions {
    entity: Entireness,
}

impl StorageRegister for MemoryStorage<Css> {
    type Options = ();
    type QueryOptions = CssQueryOptions;
    type Content = String;
    fn initialize<S: Storage>(_options: Self::Options) -> anyhow::Result<Self> {
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
        options: Self::QueryOptions,
    ) -> Option<Arc<Self::Content>> {
        let ref_css = self.register.get(&(id, level))?;
        let css = match options.entity {
            Entireness::Full => ref_css.full(),
            Entireness::RenderCritical => ref_css.render_critical(),
            Entireness::NonRenderCritical => ref_css.non_render_critical(),
        };
        Some(css)
    }
    fn options(&self) -> &Self::Options {
        &()
    }
}

impl MemoryStorage<Css> {
    /// Collects, processes and caches all available CSS in the file hierarchy.
    pub fn load_process_components<S: Storage>(
        &mut self,
    ) -> anyhow::Result<()> {
        self.load_process_modules::<S>()?;
        self.load_process_pages::<S>()
    }

    fn load_process_modules<S: Storage>(&mut self) -> anyhow::Result<()> {
        let module_ids =
            S::collect_component_ids(ResourceType::Css, Level::Component)?;
        for id in module_ids {
            let c = Css::new(id.clone(), Level::Component);
            self.register
                .insert((id.clone(), Level::Component), c.content::<S>(())?);
        }
        Ok(())
    }

    fn load_process_pages<S: Storage>(&mut self) -> anyhow::Result<()> {
        let page_ids =
            S::collect_component_ids(ResourceType::Css, Level::Page)?;
        for id in page_ids {
            let c = Css::new(id.clone(), Level::Page);
            self.register.insert((id, Level::Page), c.content::<S>(())?);
        }
        Ok(())
    }
}
