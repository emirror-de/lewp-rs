use {
    super::Component as JsComponent,
    crate::{
        fh::{
            Component,
            ComponentInformation,
            ComponentType,
            FileHierarchy,
            Level,
        },
        LewpError,
        LewpErrorKind,
    },
    std::{collections::HashMap, sync::Arc},
};

/// Options for the Register.
pub struct RegisterOptions {
    uri_path_prefix: String,
    autoload: bool,
}

impl RegisterOptions {
    /// Creates a [RegisterOptions] instance with `uri_path_prefix` set to `/resources/css` and
    /// `autoload` to `true`.
    pub fn new() -> Self {
        Self {
            uri_path_prefix: "/resources/css".to_string(),
            autoload: true,
        }
    }

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
        Self::new()
    }
}

/// Manages the CSS of lewp components in the given file hierarchy.
///
/// This register can be used in multi threaded environments as a shared
/// variable. It loads all components available in the given file hierarchy and
/// keeps them in memory, as long as this instance is available.
pub struct Register {
    options: RegisterOptions,
    components: HashMap<Arc<ComponentInformation>, Arc<String>>,
}

impl Register {
    /// Creates a new Register instance.
    pub fn new<T: FileHierarchy>(
        options: RegisterOptions,
    ) -> anyhow::Result<Self> {
        let mut register = Self {
            options,
            components: HashMap::new(),
        };
        if register.options.autoload() {
            register.load_process_components::<T>()?
        }
        Ok(register)
    }

    /// Queries the CSS of the given component using the given options.
    pub fn query(
        &self,
        component_information: Arc<ComponentInformation>,
    ) -> Option<Arc<String>> {
        Some(Arc::clone(self.components.get(&component_information)?))
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
        let module_ids = T::collect_component_ids(
            ComponentType::JavaScript,
            Level::Component,
        )?;
        for id in module_ids {
            let component_information = Arc::new(ComponentInformation {
                id: id.clone(),
                level: Level::Component,
                kind: ComponentType::JavaScript,
            });
            let c = JsComponent::new(component_information.clone());
            let c = match c.content::<T>(()) {
                Ok(c) => c,
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "{}",
                        LewpError {
                            kind: LewpErrorKind::JavaScript,
                            message: format!(
                                "Could not get minified JavaScript: {e}",
                            ),
                            source_component: component_information.clone(),
                        }
                    ))
                }
            };
            self.components
                .insert(component_information.clone(), Arc::new(c));
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
                kind: ComponentType::JavaScript,
            });
            let c = JsComponent::new(component_information.clone());
            let c = match c.content::<T>(()) {
                Ok(c) => c,
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "{}",
                        LewpError {
                            kind: LewpErrorKind::JavaScript,
                            message: format!(
                                "Could not get minified JavaScript: {e}",
                            ),
                            source_component: component_information.clone(),
                        }
                    ))
                }
            };
            self.components
                .insert(component_information.clone(), Arc::new(c));
        }
        Ok(())
    }
}
