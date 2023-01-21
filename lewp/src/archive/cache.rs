use {
    super::{Archive, ArchiveComponent},
    crate::{
        component::ComponentDetails,
        resources::{
            Css,
            CssOptions,
            Js,
            JsOptions,
            Resource,
            ResourceLevel,
            ResourceType,
        },
    },
    std::{any::Any, collections::HashMap, sync::Arc},
};

/// Can hold multiple components identified by [ComponentDetails] in memory.
#[derive(Default)]
pub struct ArchiveCache {
    cache: HashMap<ComponentDetails, Arc<dyn Any + Send + Sync>>,
}

impl ArchiveCache {
    /// Inserts the given component into the cache.
    pub fn insert<C: ArchiveComponent + Send + Sync + 'static>(
        &mut self,
        component: Arc<Resource<C>>,
    ) {
        self.cache.insert(component.details().clone(), component);
    }

    /// Inserts the given list of components into the cache.
    pub fn insert_all<C: ArchiveComponent + Send + Sync + 'static>(
        &mut self,
        components: Vec<Arc<Resource<C>>>,
    ) {
        for c in components {
            self.cache.insert(c.details().clone(), c);
        }
    }

    /// Queries the component with the given details.
    pub fn query<C: ArchiveComponent + Send + Sync + 'static>(
        &self,
        details: &ComponentDetails,
    ) -> Option<Arc<&Resource<C>>> {
        let c = match self.cache.get(&details) {
            Some(c) => c.downcast_ref::<Resource<C>>().unwrap(),
            None => return None,
        };
        Some(Arc::new(c))
    }

    /// Prepares the instance for further use in your program. After calling this
    /// method, the instance is not adjustable anymore.
    pub fn seal(self) -> Arc<Self> {
        Arc::new(self)
    }

    /// Loads all [Css] components from the archive and inserts them into the
    /// cache.
    pub fn load_css<A: Archive>(mut self) -> anyhow::Result<Self> {
        self.load_css_modules::<A>()?;
        self.load_css_pages::<A>()?;
        Ok(self)
    }

    fn load_css_modules<A: Archive>(&mut self) -> anyhow::Result<()> {
        let module_ids = A::collect_component_ids(
            ResourceType::Css,
            ResourceLevel::Component,
        )?;
        for id in module_ids {
            let options = CssOptions {
                id,
                level: ResourceLevel::Component,
            };
            let css = Resource::<Css>::load::<A>(options)?;
            self.insert(Arc::new(css));
        }
        Ok(())
    }

    fn load_css_pages<A: Archive>(&mut self) -> anyhow::Result<()> {
        let page_ids =
            A::collect_component_ids(ResourceType::Css, ResourceLevel::Page)?;
        for id in page_ids {
            let options = CssOptions {
                id,
                level: ResourceLevel::Page,
            };
            let css = Resource::<Css>::load::<A>(options)?;
            self.insert(Arc::new(css));
        }
        Ok(())
    }

    /// Loads all [Js] components from the archive and inserts them into the
    /// cache.
    pub fn load_javascript<A: Archive>(mut self) -> anyhow::Result<Self> {
        self.load_js_modules::<A>()?;
        Ok(self)
    }

    fn load_js_modules<A: Archive>(&mut self) -> anyhow::Result<()> {
        let module_ids = A::collect_component_ids(
            ResourceType::JavaScript,
            ResourceLevel::Component,
        )?;
        for id in module_ids {
            let options = JsOptions {
                id,
                level: ResourceLevel::Component,
            };
            let js = Resource::<Js>::load::<A>(options)?;
            self.insert(Arc::new(js));
        }
        Ok(())
    }
}
