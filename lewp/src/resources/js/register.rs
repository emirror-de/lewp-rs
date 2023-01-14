use {
    super::Js,
    crate::{
        component::ComponentId,
        storage::{Level, ResourceType, Storage, StorageComponent},
    },
    std::{collections::HashMap, sync::Arc},
};

/// Options for the Register.
#[derive(Clone, Debug)]
pub struct JsRegisterOptions {
    uri_path_prefix: String,
    autoload: bool,
}

impl JsRegisterOptions {
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

impl Default for JsRegisterOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages the JavaScript of lewp components in the given file hierarchy.
///
/// This register can be used in multi threaded environments as a shared
/// variable. It loads all components available in the given file hierarchy and
/// keeps them in memory, as long as this instance is available.
pub struct JsRegister {
    options: JsRegisterOptions,
    components: HashMap<(ComponentId, Level), Arc<String>>,
}

impl JsRegister {
    /// Creates a new Register instance.
    pub fn new<T: Storage>(options: JsRegisterOptions) -> anyhow::Result<Self> {
        log::debug!("Creating new JS register with options: {options:?}");
        let mut register = Self {
            options,
            components: HashMap::new(),
        };
        if register.options.autoload() {
            register.load_process_components::<T>()?
        }
        Ok(register)
    }

    /// Returns a copy to the [RegisterOptions].
    pub fn options(&self) -> JsRegisterOptions {
        self.options.clone()
    }

    /// Queries the JS of the given component using the given options.
    pub fn query(&self, id: ComponentId, level: Level) -> Option<Arc<String>> {
        Some(Arc::clone(self.components.get(&(id, level))?))
    }

    /// Collects, processes and caches all available CSS in the file hierarchy.
    pub fn load_process_components<T: Storage>(
        &mut self,
    ) -> anyhow::Result<()> {
        self.load_process_modules::<T>()?;
        self.load_process_pages::<T>()
    }

    fn load_process_modules<T: Storage>(&mut self) -> anyhow::Result<()> {
        let module_ids = T::collect_component_ids(
            ResourceType::JavaScript,
            Level::Component,
        )?;
        for id in module_ids {
            let c = Js::new(id.clone(), Level::Component);
            let c = match c.content::<T>(()) {
                Ok(c) => c,
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Could not get minified JavaScript: {e}",
                    ))
                }
            };
            self.components
                .insert((id.clone(), Level::Component), Arc::new(c));
        }
        Ok(())
    }

    fn load_process_pages<T: Storage>(&mut self) -> anyhow::Result<()> {
        let page_ids =
            T::collect_component_ids(ResourceType::JavaScript, Level::Page)?;
        for id in page_ids {
            let c = Js::new(id.clone(), Level::Page);
            let c = match c.content::<T>(()) {
                Ok(c) => c,
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Could not get minified JavaScript: {e}",
                    ))
                }
            };
            self.components
                .insert((id.clone(), Level::Page), Arc::new(c));
        }
        Ok(())
    }
}
