use {
    super::SubModule,
    crate::{
        fh::{Component, ComponentType, Level},
        module::RuntimeInformation,
        LewpError,
        LewpErrorKind,
    },
    std::rc::Rc,
};

/// Required to run submodules.
pub trait Runtime: SubModule {
    /// Runs all submodules in order as they are returned in [super::SubModule::submodules].
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
    /// **idx**: The index of the module in [super::SubModule::submodules].
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
                    source_component: Component {
                        id: self.id().to_string(),
                        kind: ComponentType::Module,
                        level: Level::Module,
                    },
                });
            }
        };
        module.run(Rc::new(RuntimeInformation::new()))?;
        Ok(())
    }

    /// Runs the first submodule with the given [id](crate::module::Metadata::id).
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
            source_component: Component {
                id: self.id().to_string(),
                kind: ComponentType::Module,
                level: Level::Module,
            },
        })
    }

    /// Runs all submodules with the given [id](crate::module::Metadata::id).
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
}
