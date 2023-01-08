#![deny(missing_docs)]
//! ----------------
//!
//! ![Version](https://img.shields.io/crates/v/lewp?style=flat-square)
//! [![](https://img.shields.io/docsrs/lewp?style=flat-square)](https://docs.rs/lewp)
//! ![Downloads](https://img.shields.io/crates/d/lewp?style=flat-square)
//! ![MIT or Apache-2.0 License](https://img.shields.io/crates/l/lewp?style=flat-square)
//!
//! Lewp is a modular library that supports you in generating and rendering
//! your website with ease. It also provides you with the possibility to
//! manage different types of resources like images required for your website.
//! Say goodbye to the web template hell. Generate your HTML5 website technically optimized and always valid. In your Rust source.
//!
//! ⚠ ***This crate is currently evolving. API breaking changes can happen anytime until v1.0.0.
//! Compiler warnings are currently used as reminder and will be removed as development continues.***
//!
//! ## Provided solutions
//!
//! When using [lewp](crate), you get the following benefits during web development:
//!
//! * No more template hell in your code base
//! * No more whitespace bugs in your website
//! * Technically optimized, always valid, minified, HTML5 code
//! * Component based development, truly isolated
//! * Build the DOM completely in Rust

pub use {
    charsets::Charset,
    error::{LewpError, LewpErrorKind},
    langtag::LanguageTag,
};

/// Re-export of the [lewp_html] crate.
pub mod html {
    pub use lewp_html::*;
}

pub mod component;
pub mod css;
mod error;
pub mod fh;
pub mod page;
pub mod resources;
pub mod view;
