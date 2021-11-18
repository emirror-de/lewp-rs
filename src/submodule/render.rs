use crate::{dom::Nodes, submodule::SubModule, LewpError};

/// Renders the given submodule to the calling module.
pub trait Render: SubModule {
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
    /// **idx**: The index of the module in [super::SubModule::submodules].
    fn render_submodule(
        &self,
        idx: usize,
        parent_module_view: &mut Nodes,
    ) -> Result<(), LewpError> {
        let module = match self.submodules().get(idx) {
            Some(m) => m.borrow(),
            None => {
                return Err(LewpError::ModuleNotFound((
                    self.id().to_string(),
                    format!("Submodule with index {} not found!", idx),
                )))
            }
        };
        parent_module_view.append(&mut module.render());
        Ok(())
    }

    /// Renders the first submodule with the given [id](crate::module::Metadata::id).
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
        Err(LewpError::ModuleNotFound((
            self.id().to_string(),
            format!(
                "Module with id \"{}\" could not be found in the submodules during rendering.",
                id.to_string()
            ),
        )))
    }

    /// Renders all submodules with the given [id](crate::module::Metadata::id).
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
}
