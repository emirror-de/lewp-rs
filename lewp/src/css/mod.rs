//! CSS modification functions and structs especially required by lewp.
//!
//! # CSS processing information
//!
//! The stylesheets are processed during the initialization of the [Lewp](crate::Lewp) struct.
//! This should usually take place at the startup of your program/server.
//!
//! # The module class name
//!
//! Every module receive a unique class name on rendering which is basically
//! the representation of [ModuleId](crate::ModuleId) defined in your
//! implementation of the [module trait](crate::Module).
//!
//! For example if you defined the `id` method like this:
//! ```
//! fn id(&self) -> ModuleId {
//!     "my-new-navigation-bar"
//! }
//! ```
//! The resulting class name would be `my-new-navigation-bar`. To query all
//! anchors that are contained in this module you can use:
//! ```css
//! .my-new-navigation-bar a {
//!     text-decoration: none;
//! }
//! ```
//!
//! # Special keywords
//!
//! To be able to detect parts of your CSS source code that targets specific
//! nodes, it is required to introduce special keywords.
//!
//! ## The `#module` keyword
//! Lewp is not able to detect whether which style rule targets the main
//! node of a module by simply scanning your CSS files. To prevent the requirement
//! of storing this particular style rule to the page level, the `#module` keyword
//! has been introduced. This keyword is removed during processing, and the
//! module's class name (see above) is appended to the rest of the rule name.
//!
//! For example if the id of your module is `my-new-navigation-bar` the following code
//! ```css
//! #module nav {
//!     background: #fff;
//! }
//! ```
//! will be transformed into:
//! ```css
//! nav.my-new-navigation-bar {
//!     background: #fff;
//! }
//! ```
//!
//! **Things to consider**
//! * The `#module` keyword only target is to identify your module's root node.
//! * The `#module` keyword needs to be followed by a space and a single identifier.
//! If these conditions are not met, it can break your compiled stylesheet.

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
