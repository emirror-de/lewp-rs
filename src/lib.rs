#![deny(missing_docs)]
//! Say goodbye to the web template hell. Generate your HTML5 website technically optimized and always valid. In your Rust source.
//!
//! ## Provided solutions
//!
//! When using [lewp](crate), you get the following benefits during web development:
//!
//! * No template hell in your code base any longer
//! * No whitespace bugs in your website
//! * Technically optimized, always valid, minified, HTML5 code
//! * Module based development, truly isolated
//! * Build the DOM fully in Rust
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
    /// Indicates that a module has not been found.
    ///
    /// **Returns**
    ///
    /// `(emitting module id, message)`
    ModuleNotFound((String, String)),
}
