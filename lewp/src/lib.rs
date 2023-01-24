#![deny(missing_docs)]
//! ----------------
//!
//! ![Version](https://img.shields.io/crates/v/lewp?style=flat-square)
//! [![](https://img.shields.io/docsrs/lewp?style=flat-square)](https://docs.rs/lewp)
//! ![Downloads](https://img.shields.io/crates/d/lewp?style=flat-square)
//! ![MIT or Apache-2.0 License](https://img.shields.io/crates/l/lewp?style=flat-square)
//!
//! Lewp is a modular library that supports you in generating and rendering
//! your website with ease. Your components will be automatically isolated so
//! CSS and JavaScript definitions are not a pain anymore and do not interfere each other!
//! It also provides you with the possibility to
//! manage different types of resources like images required for your website and
//! embeds them on release build into the final binary.
//! Lewp also saves you from getting stuck and lost in the web template hell
//! by **NOT** mixing languages as other solutions do.
//!
//! Generate your HTML5 website technically optimized and always valid without
//! losing the algorithmic comfort and flexibility.
//!
//! ⚠ ***This crate is currently evolving. API breaking changes can happen anytime until v1.0.0.
//! Compiler warnings are currently used as development reminders and will be removed as soon as possible.***
//!
//! ## Features
//!
//! When using [lewp](crate), you get the following benefits during web development:
//!
//! * No more template hell in your code base
//! * No more whitespace bugs in your website
//! * Technically optimized, always valid, minified, HTML5 code
//! * Component based development, truly isolated
//! * Storage definition with pre-defined paths for easy resource management
//! * Uses [rust_embed] under the hood so all your assets are always available
//! * Build the DOM completely in Rust
//!
//! ## Hello world example
//!
//! For more examples with comments have a look at the repositories
//! [examples](https://github.com/emirror-de/lewp-rs/tree/main/lewp/examples).
//! ```
//! use lewp::{
//!     component::{Component, ComponentId},
//!     html::{
//!         api::{h1, text},
//!         Node,
//!     },
//!     page::{Page, PageId, PageModel},
//!     view::PageView,
//! };
//!
//! struct HelloWorld {
//!     data: String,
//! }
//! impl HelloWorld {
//!     pub fn new() -> Self {
//!         Self {
//!             data: String::from("Hello World!"),
//!         }
//!     }
//! }
//! impl Component for HelloWorld {
//!     type Message = ();
//!
//!     fn id(&self) -> ComponentId {
//!         "hello-world".into()
//!     }
//!
//!     fn main(&mut self) {}
//!
//!     fn view(&self) -> Option<Node> {
//!         Some(h1(vec![text(&self.data)]))
//!     }
//! }
//!
//! struct HelloWorldPage;
//! impl PageModel for HelloWorldPage {
//!     fn id(&self) -> PageId {
//!         "hello-world-page".into()
//!     }
//!
//!     fn main(&self, view: &mut PageView) {
//!         let mut comp = Component::new(HelloWorld::new());
//!         view.push(&mut comp);
//!     }
//! }
//!
//! fn main() {
//!     simple_logger::init().unwrap();
//!
//!     let page = Page::from(HelloWorldPage {});
//!     let executed_page = page.main();
//!
//!     println!("{}", executed_page.render());
//! }
//! ```

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
