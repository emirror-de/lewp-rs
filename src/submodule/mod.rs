//! Traits and data structures for modules that have submodules.
use {
    crate::{
        dom::Nodes,
        fh::{Component, ComponentInformation, Level},
        module::{Module, ModulePtr, Modules, RuntimeInformation},
        LewpError,
        LewpErrorKind,
    },
    std::rc::Rc,
};

//mod render;
//mod runtime;

//pub use {render::Render, runtime::Runtime};

/// Enables management of submodules.
pub trait SubModule: Module {
    /// Returns a reference to the submodules.
    fn submodules(&self) -> &Modules;

    /// Returns a mutable reference to the submodules.
    fn submodules_mut(&mut self) -> &mut Modules;

    /// Appends the given module as a submodule.
    fn append_module(&mut self, module: ModulePtr) -> Result<(), LewpError> {
        if self.id() == module.borrow().id() {
            return Err(LewpError {
                kind: LewpErrorKind::LoopDetection,
                message: format!(
                    "Circular reference found in module with id '{}'.",
                    self.id()
                ),
                source_component: self.component_information(),
            });
        }
        self.submodules_mut().push(module);
        Ok(())
    }

    /// Renders all submodules to the parent view.
    fn render_submodules(&self, parent_module_view: &mut Nodes) {
        for module in self.submodules() {
            parent_module_view.append(&mut module.borrow().render());
        }
    }

    /// Renders the given submodule to the parent view.
    ///
    /// Parameters:
    ///
    /// **idx**: The index of the module in [Self::submodules].
    fn render_submodule(
        &self,
        idx: usize,
        parent_module_view: &mut Nodes,
    ) -> Result<(), LewpError> {
        let module = match self.submodules().get(idx) {
            Some(m) => m.borrow(),
            None => {
                return Err(LewpError {
                    kind: LewpErrorKind::Render,
                    message: format!(
                        "Could not find module with index {}.",
                        idx
                    ),
                    source_component: self.component_information(),
                });
            }
        };
        parent_module_view.append(&mut module.render());
        Ok(())
    }

    /// Renders the first submodule with the given [id](crate::module::Module::id).
    ///
    /// Parameters:
    ///
    /// **id**: The unique identifier of the module.
    fn render_submodule_id(
        &self,
        id: &str,
        parent_module_view: &mut Nodes,
    ) -> Result<(), LewpError> {
        for module in self.submodules() {
            let module = module.borrow();
            if module.id() != id {
                continue;
            }
            parent_module_view.append(&mut module.render());
            return Ok(());
        }
        Err(LewpError {
            kind: LewpErrorKind::Render,
            message:
                "Module could not be found in the submodules during rendering."
                    .to_string(),
            source_component: self.component_information(),
        })
    }

    /// Renders all submodules with the given [id](crate::module::Module::id).
    ///
    /// Parameters:
    ///
    /// **id**: The unique identifier of the modules to be rendered.
    fn render_submodule_id_all(
        &self,
        id: &str,
        parent_module_view: &mut Nodes,
    ) -> Result<(), LewpError> {
        for module in self.submodules() {
            let module = module.borrow();
            if module.id() != id {
                continue;
            }
            parent_module_view.append(&mut module.render());
        }
        Ok(())
    }

    /// Runs all submodules in order as they are returned in [Self::submodules].
    fn run_submodules(
        &mut self,
        runtime_information: Rc<RuntimeInformation>,
    ) -> Result<(), LewpError> {
        for module in self.submodules() {
            let mut module = module.borrow_mut();
            module.run(runtime_information.clone())?;
            runtime_information.increase_execution_count(module.id());
        }
        Ok(())
    }

    /// Runs the given submodule.
    ///
    /// Parameters:
    ///
    /// **idx**: The index of the module in [Self::submodules].
    fn run_submodule(&mut self, idx: usize) -> Result<(), LewpError> {
        let submodules = self.submodules();
        let mut module = match submodules.get(idx) {
            Some(m) => m.borrow_mut(),
            None => {
                return Err(LewpError {
                    kind: LewpErrorKind::Runtime,
                    message: format!(
                        "Could not run submodule with index {}",
                        idx
                    ),
                    source_component: self.component_information(),
                });
            }
        };
        module.run(Rc::new(RuntimeInformation::new()))?;
        Ok(())
    }

    /// Runs the first submodule with the given [id](crate::module::Module::id).
    ///
    /// Parameters:
    ///
    /// **id**: The unique identifier of the module.
    fn run_submodule_id(&mut self, id: &str) -> Result<(), LewpError> {
        for module in self.submodules() {
            let mut module = module.borrow_mut();
            if module.id() != id {
                continue;
            }
            module.run(Rc::new(RuntimeInformation::new()))?;
            return Ok(());
        }
        Err(LewpError {
            kind: LewpErrorKind::ModuleNotFound,
            message: "Could not find module in submodules register."
                .to_string(),
            source_component: self.component_information(),
        })
    }

    /// Runs all submodules with the given [id](crate::module::Module::id).
    ///
    /// Parameters:
    ///
    /// **id**: The unique identifier of the modules to be run.
    fn run_submodule_id_all(&mut self, id: &str) -> Result<(), LewpError> {
        for module in self.submodules() {
            let mut module = module.borrow_mut();
            if module.id() != id {
                continue;
            }
            module.run(Rc::new(RuntimeInformation::new()))?;
        }
        Ok(())
    }

    /// Returns the meta information for this submodule.
    fn component_information(&self) -> Rc<ComponentInformation> {
        Rc::new(ComponentInformation {
            id: self.id().to_string(),
            level: Level::Module,
            kind: String::from("SubModule"),
        })
    }
}
