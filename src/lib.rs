#![deny(missing_docs)]
//! Generate your HTML5 website technically optimized and always valid.
//!
//! This crate addresses typical problems in web development like
//! * generate **always valid** HTML5 code,
//! * **always serve minified** HTML5 code,
//! * development of **modules, truly isolated** during website runtime.
//!

pub use {charsets::Charset, langtag::LanguageTag};

pub mod config;
pub mod dom;
pub mod module;
pub mod page;
#[cfg(feature = "submodules")]
pub mod submodule;

/// Contains the error definitions that occur in [lewp](crate).
#[derive(Debug)]
pub enum Error {
    /// Raised when a loop reference has been detected.
    LoopDetection(String),
    /// Occurs when the [run](crate::module::Runtime) function fails.
    ///
    /// **Returns**
    ///
    /// `(module id, message)`
    RuntimeError((String, String)),
    /// Indicates that a module has not been found.
    ///
    /// **Returns**
    ///
    /// `(emitting_module_id, message)`
    ModuleNotFound((String, String)),
    /// Occurs when a mutable reference could not be obtained.
    ///
    /// **Returns**
    ///
    /// `(emitting_module_id, message)`
    MutableReference((String, String)),
}
