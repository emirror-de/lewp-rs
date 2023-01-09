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
//! Lewp also saves you from getting stuck and lost in the web template hell
//! by **NOT** mixing languages.
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
//! * Component based development, truly isolated with minimum overhead
//! * File hierarchy for easy resource management
//! * Build the DOM completely in Rust
//!
//! ## Hello world example
//! ```
//! use lewp::{
//!     component::{Component, ComponentId},
//!     html::{
//!         api::{h1, text},
//!         Node,
//!     },
//!     page::{Page, PageId},
//!     view::PageView,
//! };
//!
//! // Your hello world component.
//! struct HelloWorld {
//!     data: String,
//! }
//!
//! impl HelloWorld {
//!     pub fn new() -> Self {
//!         Self {
//!             data: String::from("Hello World!"),
//!         }
//!     }
//! }
//!
//! // Implement the [Component] trait to define the behavior and view.
//! impl Component for HelloWorld {
//!     // No message required for a simple component.
//!     type Message = ();
//!
//!     // The unique ID of your component is used to identify and process further
//!     // resources, as well as isolation in the world of JavaScript on client side.
//!     // It is best practice to use the lowercase kebab-case of your structs name
//!     // to have a clear identification of the components resources in the file
//!     // hierarchy and your code base.
//!     fn id(&self) -> ComponentId {
//!         "hello-world".into()
//!     }
//!
//!     // There is no reason for your page to fail. Errors during processing should
//!     // result in a different view that you define below.
//!     fn main(&mut self) {}
//!
//!     // This is the view of your component.
//!     fn view(&self) -> Option<Node> {
//!         Some(h1(vec![text(&self.data)]))
//!     }
//! }
//!
//! // Define your page. This simple example page only contains one component that
//! // only specifies a h1 node.
//! struct HelloWorldPage;
//!
//! impl Page for HelloWorldPage {
//!     // Throughout your site, the page id should be unique for the same reason as
//!     // the component id. Use lowercase kebab-case here as well as convention.
//!     fn id(&self) -> PageId {
//!         "hello-world-page".into()
//!     }
//!
//!     // The main method of the page. In here you can add your components to the
//!     // page and do whatever processing is required for your page to be rendered.
//!     fn main(&self, view: &mut PageView) {
//!         let mut comp = Component::new(HelloWorld::new());
//!         // The component is only borrowed, to enable the possibility of adding
//!         // it twice to your page. You can use the state of your component to
//!         // define the behavior when adding it multiple times.
//!         // However, the required head nodes for example CSS and JS is being added
//!         // only once, so you can be sure that there is no overhead when adding
//!         // the component multiple times.
//!         view.push(&mut comp);
//!     }
//! }
//!
//! fn main() {
//!     simple_logger::init().unwrap();
//!
//!     // Create an instance of your page
//!     let page = Page::new(HelloWorldPage {});
//!
//!     // You have full control when you want to run and render your page.
//!     // Because the internal state of the page changes when running the main
//!     // method, you need to get the result in order to be able to render the
//!     // resulting page.
//!     let executed_page = page.main();
//!
//!     println!("{}", executed_page.render());
//! }
//! ```

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
