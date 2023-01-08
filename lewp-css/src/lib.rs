// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//! # lewp-css
//!
//! *Forked version and continued version of [css](https://github.com/lemonrock/css)*
//!
//! A Rust library crate for parsing, manipulating and serializing CSS stylesheets.
//! Makes use of existing CSS parser logic from Servo.
//! Includes forks of code from Servo because these are unpublished on <https://crates.io>.
//! One use of this library is to minify CSS, as the serialized form it produces is minimal.
//! Another use is provide a crate that others can use for auto-prefixing CSS and to eliminate unused CSS.
//! The values of property declarations are currently stored as a string. Parsing property declarations is a monster job (in effect there are bespoke rules for every property). If you feel like helping...
//!
//!
//! ## Usages
//!
//!
//! ### Loading and Saving Stylesheets
//!
//! ```
//! use ::lewp_css::Stylesheet;
//!
//! let some_css = "input { margin-left: 10pt; top: 20px; }".to_owned();
//! let stylesheet = Stylesheet::parse(&some_css).expect("CSS was invalid");
//!
//! // Alternatively, load from a file using Stylesheet::from_file_path("/path/to/stylesheet.css").unwrap();
//!
//! let mut destination = String::new();
//!
//! // Don't write source-map and source-url comments if any are present in the stylesheet
//! let include_source_urls = false;
//!
//! stylesheet.to_css(&mut destination, include_source_urls).expect("Failed to write to destination");
//!
//! assert_eq!(&destination, "input{margin-left: 10pt;top: 20px}");
//!
//! // To serialize to a Vec<u8> of bytes instead
//! let mut bytes = stylesheet.to_bytes(false);
//!
//! // To serialize to a file instead
//! //stylesheet.to_file_path("/path/to/to/stylesheet.css", false).unwrap();
//! ```
//!
//!
//! ### To parse a single CSS selector
//!
//! ```
//! use ::lewp_css::parse_css_selector;
//!
//! let selector = parse_css_selector("P.myclass").unwrap();
//! ```
//!
//!
//! ### To match CSS selectors to HTML
//!
//! Use the `html5ever_ext` crate. (The function `domain::selectors::matches()` can do matching but needs a lot of HTML logic to do so).
//!

#[macro_use]
extern crate bitflags;
#[macro_use]
pub extern crate cssparser;
extern crate either;
extern crate phf;
extern crate precomputed_hash;
#[macro_use]
extern crate quick_error;
pub extern crate servo_arc;
pub extern crate smallvec;

/// Contains definitions of objects used in Stylesheet.
pub mod domain;
pub(crate) mod parsers;
pub(crate) mod serializers;

mod blocking_io_only_std_fmt_write_to_std_io_write_adaptor;
mod custom_parse_error;
mod parse_css_selector;
mod stylesheet;
mod stylesheet_error;

pub use {
    custom_parse_error::CustomParseError,
    parse_css_selector::*,
    stylesheet::Stylesheet,
    stylesheet_error::*,
};
