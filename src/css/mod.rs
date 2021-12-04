//! CSS modification functions especially required by lewp.

mod component;
mod entireness;
mod register;

pub use {
    component::Component,
    entireness::Entireness,
    register::{Register, RegisterOptions},
};
