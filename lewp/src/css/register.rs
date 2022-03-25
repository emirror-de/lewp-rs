use {
    super::{Component, Entireness, ProcessedComponent},
    crate::{
        fh::{ComponentInformation, ComponentType, FileHierarchy, Level},
        LewpError,
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
    fh: Arc<FileHierarchy>,
    options: RegisterOptions,
    components: HashMap<Arc<ComponentInformation>, ProcessedComponent>,
}

impl Register {
    /// Creates a new Register instance.
    pub fn new(
        fh: Arc<FileHierarchy>,
        options: RegisterOptions,
    ) -> Result<Self, LewpError> {
        let mut register = Self {
            fh,
            options,
            components: HashMap::new(),
        };
        if register.options.autoload() {
            register.load_process_components()?
        }
        Ok(register)
    }

    /// Queries the CSS of the given component using the given options.
    pub fn query(
        &self,
        component_information: Arc<ComponentInformation>,
        entity: Entireness,
    ) -> Option<Arc<String>> {
        let ref_css = self.components.get(&component_information)?;
        Some(ref_css.render_critical())
    }

    /// Collects, processes and caches all available CSS in the file hierarchy.
    pub fn load_process_components(&mut self) -> Result<(), LewpError> {
        self.load_process_modules()?;
        self.load_process_pages()
    }

    /// Returns the path prefix where the CSS is mounted on the webserver.
    pub fn css_path(&self, level: Level, id: String) -> &str {
        &self.options.uri_path_prefix()
    }

    fn load_process_modules(&mut self) -> Result<(), LewpError> {
        let module_ids = self
            .fh
            .collect_component_ids(ComponentType::Css, Level::Module)?;
        for id in module_ids {
            let component_information = Arc::new(ComponentInformation {
                id: id.clone(),
                level: Level::Module,
                kind: ComponentType::Css,
            });
            let c =
                Component::new(component_information.clone(), self.fh.clone());
            let c = ProcessedComponent::from(&c)?;
            self.components.insert(component_information.clone(), c);
        }
        Ok(())
    }

    fn load_process_pages(&mut self) -> Result<(), LewpError> {
        let page_ids = self
            .fh
            .collect_component_ids(ComponentType::Css, Level::Page)?;
        for id in page_ids {
            let component_information = Arc::new(ComponentInformation {
                id: id.clone(),
                level: Level::Page,
                kind: ComponentType::Css,
            });
            let c =
                Component::new(component_information.clone(), self.fh.clone());
            let c = ProcessedComponent::from(&c)?;
            self.components.insert(component_information.clone(), c);
        }
        Ok(())
    }
}

impl Default for Register {
    /// Creates a CssRegister with a default [FileHierarchy] and [RegisterOptions].
    fn default() -> Self {
        Self::new(Arc::new(FileHierarchy::default()), RegisterOptions::default())
            .expect("Default CSS register instantiation should always work! If not, check your FileHierarchy setup first!")
    }
}
