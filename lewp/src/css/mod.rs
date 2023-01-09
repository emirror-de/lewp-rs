//! CSS modification functions and structs especially required by lewp.
//!
//! # CSS processing information
//!
//! The stylesheets are processed during the initialization of the [Component] struct.
//! This should usually take place at the startup of your program/server.
//!
//! # The component class name
//!
//! Every component HTML node receives a unique class name on rendering which is basically
//! the representation of [ComponentId](crate::component::ComponentId) defined in your
//! implementation of the [Component trait](crate::component::Component).
//!
//! For example if you defined the `id` method like this:
//! ```
//! # use lewp::component::ComponentId;
//! # struct YourModule;
//! # impl YourModule {
//! fn id(&self) -> ComponentId {
//!     "my-new-navigation-bar".into()
//! }
//! # }
//! ```
//! The resulting class name would be `my-new-navigation-bar`. The CSS that
//! you define for your component will receive this class name during processing.
//! So it is **NOT** required to add it in your CSS file.
//!
//! # Example, automated addition of the components class name
//!
//! Your CSS file for your component looks like this:
//! ```css
//! a {
//!     text-decoration: none;
//! }
//! ```
//! After processing, the CSS that is sent to the client will look like this:
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
//! ## The `#component` keyword
//! Lewp is not able to detect whether which style rule targets the main
//! node of a component by simply scanning your CSS files. To prevent the requirement
//! of storing this particular style rule to the page level, the `#component` keyword
//! has been introduced. This keyword is removed during processing, and the
//! components class name (see above) is appended to the rest of the rule name.
//!
//! For example if the id of your component is `my-new-navigation-bar` and the
//! node of the view is [nav](lewp_css::api::nav) the following code
//! ```css
//! #component nav {
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
//! # Things to consider
//! * The `#component` keyword only target is to identify your module's root node.
//! * The `#component` keyword needs to be followed by a space and a single identifier.
//!
//! If these conditions are not met, it can break your compiled stylesheet.

mod component;
mod entireness;
mod processed_component;
mod property_classification;
mod register;

#[cfg(test)]
mod test;

pub use {
    component::Component,
    entireness::Entireness,
    processed_component::ProcessedComponent,
    register::{Register, RegisterOptions},
};
