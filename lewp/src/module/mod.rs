//! Traits and types necessary for creating a web module.

mod module;
mod runtime;

use std::{cell::RefCell, rc::Rc};

pub use {module::Module, runtime::RuntimeInformation};

/// A collection of modules.
pub type Modules = Vec<ModulePtr>;

/// Wrapper type for a Module instance.
pub type ModulePtr = Rc<RefCell<dyn Module>>;

/// The unique module ID. Similar to the page ID, it is used to identify resources
/// belonging to the module on the file system.
pub type ModuleId = str;
