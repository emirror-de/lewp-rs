use {
    super::{Component, Entireness, ProcessedComponent},
    crate::{
        fh::{ComponentInformation, ComponentType, FileHierarchy, Level},
        LewpError,
    },
    std::{collections::HashMap, rc::Rc, sync::Arc},
};

/// Options for the Register.
pub struct RegisterOptions {}

impl RegisterOptions {
    /// Creates a new RegisterOptions instance.
    pub fn new() -> Self {
        Self {}
    }
}

/// Manages the CSS of lewp components in the given file hierarchy.
///
/// This register can be used in multi threaded environments as a shared
/// variable. It loads all components available in the given file hierarchy and
/// keeps them in memory, as long as this instance is available.
pub struct Register {
    fh: Rc<FileHierarchy>,
    options: RegisterOptions,
    components: HashMap<Rc<ComponentInformation>, ProcessedComponent>,
}

impl Register {
    /// Creates a new Register instance.
    pub fn new(fh: FileHierarchy, options: RegisterOptions) -> Self {
        Self {
            fh: Rc::new(fh),
            options,
            components: HashMap::new(),
        }
    }

    /// Queries the CSS of the given component using the given options.
    pub fn query(
        &self,
        component_information: Rc<ComponentInformation>,
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

    fn load_process_modules(&mut self) -> Result<(), LewpError> {
        let module_ids = self
            .fh
            .collect_component_ids(ComponentType::Css, Level::Module)?;
        for id in module_ids {
            let component_information = Rc::new(ComponentInformation {
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
            let component_information = Rc::new(ComponentInformation {
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
    fn default() -> Self {
        Self::new(FileHierarchy::new(), RegisterOptions::new())
    }
}
