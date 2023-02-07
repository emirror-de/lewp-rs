#![deny(missing_docs)]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

pub use {charsets::Charset, langtag::LanguageTag};

/// Re-export of the [lewp_html] crate.
pub mod html {
    pub use lewp_html::*;
}

pub mod archive;
pub mod component;
pub mod page;
pub mod resources;
//pub mod storage;
pub mod view;
