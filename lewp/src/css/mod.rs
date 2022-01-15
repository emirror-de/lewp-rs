//! CSS modification functions especially required by lewp.

mod component;
mod entireness;
mod processed_component;
mod register;

#[cfg(test)]
mod test;

pub use {
    component::Component,
    entireness::Entireness,
    processed_component::ProcessedComponent,
    register::{Register, RegisterOptions},
};
