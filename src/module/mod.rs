//! Traits and types necessary for creating a web module.

mod metadata;
mod render;
mod reset;
mod runtime;

use {
    crate::dom::Nodes,
    std::{cell::RefCell, rc::Rc},
};

pub use {
    metadata::Metadata,
    render::Render,
    reset::Reset,
    runtime::{Runtime, RuntimeInformation},
};

/// A collection of modules.
pub type Modules = Vec<ModulePtr>;

/// Wrapper type for a Module instance.
pub type ModulePtr = Rc<RefCell<dyn Module>>;

/// Defines a web page module.
pub trait Module: Metadata + Runtime + Render {
    /// Borrows the head tags that are required to run this module in a web page.
    fn head_tags(&self) -> &Nodes;

    /// Wraps `self` into a `Rc<RefCell<>>`
    fn into_module_ptr(self) -> ModulePtr
    where
        Self: Sized + 'static,
    {
        Rc::new(RefCell::new(self))
    }
}
