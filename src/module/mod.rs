//! Traits and types necessary for creating a web module.

mod metadata;
mod render;
mod reset;
mod runtime;

use {crate::dom::Nodes, std::rc::Rc};

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
}
