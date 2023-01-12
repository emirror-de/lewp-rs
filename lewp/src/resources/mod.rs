//! Contains access to resources in the file hierarchy.

mod css;
mod image;
mod js;
mod text;

pub use {
    css::{
        Css,
        CssRegister,
        CssRegisterOptions,
        Entireness,
        ProcessedComponent,
        PropertyClassification,
    },
    image::Image,
    js::{Js, JsRegister, JsRegisterOptions},
    text::Text,
};
