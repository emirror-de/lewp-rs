#![deny(missing_docs)]
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
//! ⚠ ***This crate is currently evolving. API breaking changes can happen anytime until v1.0.0.
//! Compiler warnings will be removed in 1.0.0***
//!
//! ## Provided solutions
//!
//! When using [lewp](crate), you get the following benefits during web development:
//!
//! * No more template hell in your code base
//! * No more whitespace bugs in your website
//! * Technically optimized, always valid, minified, HTML5 code
//! * Module based development, truly isolated
//! * Build the DOM completely in Rust
//!
//! ## Hello world! example
//!
//! ```
//! use {
//!     lewp::{
//!         config::{ModuleConfig, PageConfig},
//!         html::{
//!             api::{h1, text},
//!             NodeList,
//!             Node,
//!         },
//!         Module, Modules, RuntimeInformation,
//!         Page,
//!         Charset,
//!         LanguageTag,
//!         LewpError,
//!     },
//!     std::rc::Rc,
//! };
//!
//! // This is one of your modules the webpage is build with.
//! struct HelloWorld {
//!     config: ModuleConfig,
//!     head_tags: NodeList,
//!     data: String,
//! }
//!
//! impl HelloWorld {
//!     pub fn new() -> Self {
//!         Self {
//!             config: ModuleConfig::new(),
//!             head_tags: vec![],
//!             data: String::from("hello-world"),
//!         }
//!     }
//! }
//!
//! // The [Module] trait is required for [lewp] to know it is a module. :-)
//! impl Module for HelloWorld {
//!     fn head_tags(&self) -> &NodeList {
//!         &self.head_tags
//!     }
//!
//!     fn id(&self) -> &str {
//!         "hello-world"
//!     }
//!
//!     fn config(&self) -> &ModuleConfig {
//!         &self.config
//!     }
//!
//!     fn run(
//!         &mut self,
//!         _runtime_info: Rc<RuntimeInformation>,
//!     ) -> Result<(), LewpError> {
//!         Ok(())
//!     }
//!
//!     fn view(&self) -> Node {
//!         h1(vec![text(&self.data)])
//!     }
//! }
//!
//! // This struct defines the actual page. The containing members are
//! // required because they define the base on which [lewp] is working on.
//! struct HelloWorldPage {
//!     modules: Modules,
//!     config: PageConfig,
//! }
//!
//! impl HelloWorldPage {
//!     /// Creates a new page.
//!     pub fn new(config: PageConfig) -> Self {
//!         Self {
//!             modules: vec![],
//!             config,
//!         }
//!     }
//! }
//!
//! impl Page for HelloWorldPage {
//!     fn id(&self) -> &str {
//!         "helloworldpage"
//!     }
//!
//!     fn modules(&self) -> &Modules {
//!         &self.modules
//!     }
//!     fn modules_mut(&mut self) -> &mut Modules {
//!         &mut self.modules
//!     }
//!
//!     fn title(&self) -> &str {
//!         "Hello World from lewp!"
//!     }
//!
//!     fn description(&self) -> &str {
//!         "My first page using lewp!"
//!     }
//!
//!     fn language(&self) -> LanguageTag {
//!         LanguageTag::parse("de-DE").unwrap()
//!     }
//!
//!     fn charset(&self) -> Charset {
//!         Charset::Utf8
//!     }
//!
//!     fn config(&self) -> &PageConfig {
//!         &self.config
//!     }
//!
//!     fn run(&mut self) {
//!         self.add_module(HelloWorld::new().into_module_ptr());
//!     }
//! }
//!
//! fn main() {
//!     let mut page = HelloWorldPage::new(PageConfig::default());
//!     println!("{}", page.build());
//! }
//! ```

pub use {
    crate::lewp::Lewp,
    charsets::Charset,
    error::{LewpError, LewpErrorKind},
    langtag::LanguageTag,
    module::*,
    page::Page,
};

/// Re-export of the [lewp_html] crate.
pub mod html {
    pub use lewp_html::*;
}

pub mod config;
pub mod css;
mod error;
pub mod fh;
mod lewp;
pub(crate) mod module;
mod page;
pub mod resources;
//mod web_adapter;
