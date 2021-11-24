//! Traits and data structures for modules that have submodules.
use crate::{
    fh::{Component, ComponentType, Level},
    module::{Module, ModulePtr, Modules},
    LewpError,
    LewpErrorKind,
};

mod render;
mod runtime;

pub use {render::Render, runtime::Runtime};

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
                source_component: Component::new(
                    self.id(),
                    Level::Module,
                    ComponentType::Module,
                ),
            });
        }
        self.submodules_mut().push(module);
        Ok(())
    }
}
