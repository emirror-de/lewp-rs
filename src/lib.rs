#![deny(missing_docs)]
//! ![](https://raw.githubusercontent.com/emirror-de/lewp-rs/main/logo/lewp-transparent-background.inkscape.png)
//!
//! ----------------
//!
//! ![Version](https://img.shields.io/crates/v/lewp?style=flat-square)
//! [![](https://img.shields.io/docsrs/lewp?style=flat-square)](https://docs.rs/lewp)
//! ![Downloads](https://img.shields.io/crates/d/lewp?style=flat-square)
//! ![MIT or Apache-2.0 License](https://img.shields.io/crates/l/lewp?style=flat-square)
//! [![](https://img.shields.io/discord/855726181142495242?color=154683&label=discord&style=flat-square)](https://discord.gg/nx7YtsjEbT)
//!
//! Say goodbye to the web template hell. Generate your HTML5 website technically optimized and always valid. In your Rust source.
//!
//! ⚠ ***This crate is currently evolving. API changes can happen anytime until v1.0.0***
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
//! ## Examples
//!
//! Please find examples in the `examples` folder in the repository.

use crate::fh::Level;

pub use {charsets::Charset, langtag::LanguageTag};

pub mod config;
pub mod css;
pub mod dom;
pub mod fh;
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
    /// Indicates an error that occured during CSS processing.
    ///
    /// **Returns**
    ///
    /// `([Level](crate::fh::Level), emitting id, message)`
    Css(Level, String, String),
}
