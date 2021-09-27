//! Runtime traits and structs of a page.

/// Defines the behavior during runtime.
pub trait Runtime {
    /// Executes the page. Main function that is able to collect and modify
    /// data as well as modules required for rendering.
    fn run(&mut self);
}
