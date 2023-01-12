//! Integration of JavaScript for Lewp.

mod component;
mod register;

pub use {
    component::Js,
    register::{JsRegister, JsRegisterOptions},
};
