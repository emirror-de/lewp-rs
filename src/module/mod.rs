//! Traits and types necessary for creating a web module.

mod metadata;
mod render;
mod reset;
mod runtime;

use {
    crate::{dom::Nodes, Error},
    std::rc::Rc,
};

pub use {
    metadata::Metadata,
    render::Render,
    reset::Reset,
    runtime::{Runtime, RuntimeInformation},
};

/// A collection of modules.
pub type Modules = Vec<Rc<dyn Module>>;

/// Defines a web page module.
pub trait Module: Metadata + Runtime + Render {
    /// Borrows the head tags that are required to run this module in a web page.
    fn head_tags(&self) -> &Nodes;

    /// Returns a reference to the children modules.
    fn children(&self) -> &Modules;

    /// Returns a mutable reference to the children modules.
    fn children_mut(&mut self) -> &mut Modules;

    /// Appends the given module as a child.
    fn append_module(&mut self, module: Rc<dyn Module>) -> Result<(), Error> {
        if self.id() == module.id() {
            return Err(Error::LoopDetection(format!(
                "append_module, {}",
                self.id()
            )));
        }
        self.children_mut().push(module);
        Ok(())
    }
}
