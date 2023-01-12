use {
    super::{Css, Entireness, ProcessedComponent},
    crate::fh::{ComponentInformation, ComponentType, FileHierarchy, Level},
    std::{collections::HashMap, sync::Arc},
};

/// Options for the Register.
#[derive(Clone, Debug)]
pub struct RegisterOptions {
    uri_path_prefix: String,
    autoload: bool,
}

impl RegisterOptions {
    /// Returns the autoload value.
    pub fn autoload(&self) -> bool {
        self.autoload
    }

    /// Returns the uri_path_prefix value.
    pub fn uri_path_prefix(&self) -> &str {
        &self.uri_path_prefix
    }

    /// If set to true, the components will be loaded automatically on instantiation of the
    /// register.
    pub fn set_autoload(mut self, autoload: bool) -> Self {
        self.autoload = autoload;
        self
    }

    /// Sets the uri_path_prefix, usually the mountpoint of all CSS on the webserver.
    ///
    /// See [Self::new]
    pub fn set_uri_path_prefix(mut self, prefix: &str) -> Self {
        self.uri_path_prefix = prefix.to_string();
        self
    }
}

impl Default for RegisterOptions {
    fn default() -> Self {
        Self {
            uri_path_prefix: "/resources/css".to_string(),
            autoload: true,
        }
    }
}

/// Manages the CSS of lewp components in the given file hierarchy.
///
/// This register can be used in multi threaded environments as a shared
/// variable. It loads all components available in the given file hierarchy and
/// keeps them in memory, as long as this instance is available.
pub struct Register {
    options: RegisterOptions,
    components: HashMap<Arc<ComponentInformation>, ProcessedComponent>,
}

impl Register {
    /// Creates a new Register instance.
    pub fn new<T: FileHierarchy>(
        options: RegisterOptions,
    ) -> anyhow::Result<Self> {
        log::debug!("Creating new CSS register with options: {options:?}");
        let mut register = Self {
            options,
            components: HashMap::new(),
        };
        if register.options.autoload() {
            register.load_process_components::<T>()?;
        }
        Ok(register)
    }

    /// Returns a copy to the [RegisterOptions].
    pub fn options(&self) -> RegisterOptions {
        self.options.clone()
    }

    /// Queries the CSS of the given component using the given options.
    pub fn query(
        &self,
        component_information: Arc<ComponentInformation>,
        entity: Entireness,
    ) -> Option<Arc<String>> {
        let ref_css = self.components.get(&component_information)?;
        let css = match entity {
            Entireness::Full => ref_css.full(),
            Entireness::RenderCritical => ref_css.render_critical(),
            Entireness::NonRenderCritical => ref_css.non_render_critical(),
        };
        Some(css)
    }

    /// Collects, processes and caches all available CSS in the file hierarchy.
    pub fn load_process_components<T: FileHierarchy>(
        &mut self,
    ) -> anyhow::Result<()> {
        self.load_process_modules::<T>()?;
        self.load_process_pages::<T>()
    }

    /// Returns the path prefix where the CSS is mounted on the webserver.
    pub fn css_path(&self, level: Level, id: String) -> &str {
        &self.options.uri_path_prefix()
    }

    fn load_process_modules<T: FileHierarchy>(&mut self) -> anyhow::Result<()> {
        let module_ids =
            T::collect_component_ids(ComponentType::Css, Level::Component)?;
        for id in module_ids {
            let component_information = Arc::new(ComponentInformation {
                id: id.clone(),
                level: Level::Component,
                kind: ComponentType::Css,
            });
            let c = Css::new(component_information.clone());
            let c = ProcessedComponent::new::<T>(&c)?;
            self.components.insert(component_information.clone(), c);
        }
        Ok(())
    }

    fn load_process_pages<T: FileHierarchy>(&mut self) -> anyhow::Result<()> {
        let page_ids =
            T::collect_component_ids(ComponentType::Css, Level::Page)?;
        for id in page_ids {
            let component_information = Arc::new(ComponentInformation {
                id: id.clone(),
                level: Level::Page,
                kind: ComponentType::Css,
            });
            let c = Css::new(component_information.clone());
            let c = ProcessedComponent::new::<T>(&c)?;
            self.components.insert(component_information.clone(), c);
        }
        Ok(())
    }
}
