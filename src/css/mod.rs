//! CSS modification functions especially required by lewp.

mod component;
mod entireness;
mod register;

use crate::fh::{Component as FHComponent, FileHierarchy};

pub use {
    component::{Component, ComponentBuilder},
    entireness::Entireness,
    register::{Register, RegisterOptions},
};
