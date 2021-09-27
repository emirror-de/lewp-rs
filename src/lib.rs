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

/// Contains the error definitions that occur in [lewp](crate).
pub enum Error {
    /// Raised when a loop reference has been detected.
    LoopDetection(String),
}
