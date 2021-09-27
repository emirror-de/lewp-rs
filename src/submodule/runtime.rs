use {
    super::SubModule,
    crate::{module::RuntimeInformation, Error},
    std::rc::Rc,
};

/// Required to run submodules.
pub trait Runtime: SubModule {
    /// Runs all submodules in order as they are returned in [super::SubModule::submodules].
    fn run_submodules(
        &mut self,
        runtime_information: &mut Box<RuntimeInformation>,
    ) -> Result<(), Error> {
        for child in self.submodules_mut() {
            let child_id = child.id().to_owned();
            child.run(runtime_information)?;
            runtime_information.increase_execution_count(child.id());
        }
        Ok(())
    }

    /// Runs the given submodule.
    ///
    /// Parameters:
    ///
    /// **idx**: The index of the module in [super::SubModule::submodules].
    fn run_submodule(&mut self, idx: usize) -> Result<(), Error> {
        let submodules = self.submodules_mut();
        let module = match submodules.get_mut(idx) {
            Some(m) => m,
            None => {
                return Err(Error::ModuleNotFound((
                    self.id().to_string(),
                    format!("Could not run submodule with index {}", idx),
                )))
            }
        };
        let child_id = module.id().to_owned();
        module.run(&mut Box::new(RuntimeInformation::new()))?;
        Ok(())
    }

    /// Runs the first submodule with the given [id](crate::module::Metadata::id).
    ///
    /// Parameters:
    ///
    /// **id**: The unique identifier of the module.
    fn run_submodule_id(&mut self, id: &str) -> Result<(), Error> {
        for module in self.submodules_mut() {
            if module.id() != id {
                continue;
            }
            module.run(&mut Box::new(RuntimeInformation::new()))?;
            return Ok(());
        }
        Err(Error::ModuleNotFound((
            self.id().to_string(),
            format!(
                "Module with id \"{}\" could not be found in the submodules.",
                id.to_string()
            ),
        )))
    }

    /// Runs all submodules with the given [id](crate::module::Metadata::id).
    ///
    /// Parameters:
    ///
    /// **id**: The unique identifier of the modules to be run.
    fn run_submodule_id_all(&mut self, id: &str) -> Result<(), Error> {
        for module in self.submodules_mut() {
            if module.id() != id {
                continue;
            }
            module.run(&mut Box::new(RuntimeInformation::new()))?;
        }
        Ok(())
    }
}
