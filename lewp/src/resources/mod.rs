//! Contains access to resources in a given [storage](crate::storage).

mod css;
mod image;
mod js;
mod text;

pub use {
    css::{Css, Entireness, ProcessedComponent, PropertyClassification},
    image::Image,
    js::{Js, JsRegister, JsRegisterOptions},
    text::Text,
};
