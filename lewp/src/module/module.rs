use {
    super::{ModuleId, ModulePtr, RuntimeInformation},
    crate::{
        config::ModuleConfig,
        fh::{ComponentInformation, ComponentType, Level},
        LewpError,
        LewpErrorKind,
        Modules,
    },
    lewp_html::{api::div, Node, NodeExt, NodeList},
    std::{cell::RefCell, rc::Rc, sync::Arc},
};

/// Defines a web page module.
pub trait Module {
    /// Returns the unique module id.
    ///
    /// Allowed characters for id are `[a-z]`, `[0-9]` and `-`.
    /// **There is currently no check wether other characters are used. So please
    /// make sure that you do not use any other characters while creating modules.**
    fn id(&self) -> &ModuleId;

    /// The configuration of the module.
    fn config(&self) -> &ModuleConfig;

    /// Borrows the head tags that are required to run this module in a web page.
    fn head_tags(&self) -> &NodeList;

    /// Wraps `self` into a `Rc<RefCell<>>`
    fn into_module_ptr(self) -> ModulePtr
    where
        Self: Sized + 'static,
    {
        Rc::new(RefCell::new(self))
    }

    /// Constructs the view of the module.
    fn view(&self) -> Node;

    /// Renders as DOM Node.
    fn render(&self) -> Node {
        let view = self.view();
        view.borrow_attrs(vec![
            ("class", self.id()),
            ("data-lewp-component", "module"),
        ]);
        view
    }

    /// Executes the module. Main function that is able to collect and modify
    /// data required for rendering.
    fn run(
        &mut self,
        runtime_info: Rc<RuntimeInformation>,
    ) -> Result<(), LewpError>;

    /*
     * SUBMODULES
     */

    /// Returns a reference to the submodules.
    fn submodules(&self) -> Option<&Modules> {
        None
    }

    /// Returns a mutable reference to the submodules.
    fn submodules_mut(&mut self) -> Option<&mut Modules> {
        None
    }

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
        match self.submodules_mut() {
            None => (),
            Some(modules) => modules.push(module),
        };
        Ok(())
    }

    /// Renders all submodules to the parent view.
    fn render_submodules(&self, parent_module_view: &mut NodeList) {
        let submodules = match self.submodules() {
            None => return,
            Some(modules) => modules,
        };
        for module in submodules {
            parent_module_view.push(module.borrow().render());
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
        parent_module_view: &mut NodeList,
    ) -> Result<(), LewpError> {
        let submodules = match self.submodules() {
            None => return Ok(()),
            Some(modules) => modules,
        };
        let module = match submodules.get(idx) {
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
        parent_module_view.push(module.render());
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
        parent_module_view: &mut NodeList,
    ) -> Result<(), LewpError> {
        let submodules = match self.submodules() {
            None => return Ok(()),
            Some(modules) => modules,
        };
        for module in submodules {
            let module = module.borrow();
            if module.id() != id {
                continue;
            }
            parent_module_view.push(module.render());
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
        parent_module_view: &mut NodeList,
    ) -> Result<(), LewpError> {
        let submodules = match self.submodules() {
            None => return Ok(()),
            Some(modules) => modules,
        };
        for module in submodules {
            let module = module.borrow();
            if module.id() != id {
                continue;
            }
            parent_module_view.push(module.render());
        }
        Ok(())
    }

    /// Runs all submodules in order as they are returned in [Self::submodules].
    fn run_submodules(
        &mut self,
        runtime_information: Rc<RuntimeInformation>,
    ) -> Result<(), LewpError> {
        let submodules = match self.submodules() {
            None => return Ok(()),
            Some(modules) => modules,
        };
        for module in submodules {
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
    fn run_submodule(
        &mut self,
        idx: usize,
        runtime_information: Rc<RuntimeInformation>,
    ) -> Result<(), LewpError> {
        let submodules = match self.submodules() {
            None => return Ok(()),
            Some(modules) => modules,
        };
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
        module.run(runtime_information.clone())?;
        Ok(())
    }

    /// Runs the first submodule with the given [id](crate::module::Module::id).
    ///
    /// Parameters:
    ///
    /// **id**: The unique identifier of the module.
    fn run_submodule_id(
        &mut self,
        id: &str,
        runtime_information: Rc<RuntimeInformation>,
    ) -> Result<(), LewpError> {
        let modules = match self.submodules() {
            None => return Ok(()),
            Some(modules) => modules,
        };
        for module in modules {
            let mut module = module.borrow_mut();
            if module.id() != id {
                continue;
            }
            module.run(runtime_information.clone())?;
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
    fn run_submodule_id_all(
        &mut self,
        id: &str,
        runtime_information: Rc<RuntimeInformation>,
    ) -> Result<(), LewpError> {
        let modules = match self.submodules() {
            None => return Ok(()),
            Some(modules) => modules,
        };
        for module in modules {
            let mut module = module.borrow_mut();
            if module.id() != id {
                continue;
            }
            module.run(runtime_information.clone())?;
        }
        Ok(())
    }

    /// Returns the meta information for this module.
    fn component_information(&self) -> Arc<ComponentInformation> {
        Arc::new(ComponentInformation {
            id: self.id().to_string(),
            level: Level::Module,
            kind: ComponentType::Module,
        })
    }
}
