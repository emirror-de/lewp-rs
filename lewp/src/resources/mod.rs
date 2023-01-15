//! Contains access to resources in a given [storage](crate::storage).
//!
//! See [MemoryStorage](crate::storage::MemoryStorage) for optimized usage in
//! combination with [Css] and [Js].

pub(crate) mod css;
mod image;
mod js;
mod text;

pub use {css::Css, image::Image, js::Js, text::Text};
