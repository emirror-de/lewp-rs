//! Runtime traits and structs of a module.

use crate::Error;

/// Defines the behavior during runtime.
pub trait Runtime {
    /// Executes the module. Main function that is able to collect and modify
    /// data required for rendering.
    fn run(&mut self, runtime_info: &RuntimeInformation) -> Result<(), Error>;
}

/// Contains runtime information of a module.
pub struct RuntimeInformation {
    /// The number of times the module has been executed previously.
    pub previously_executed_count: u32,
    /// The id of the previously executed module. Set to None if it is the first module that is
    /// being executed.
    pub previous_module_id: Option<String>,
}

impl RuntimeInformation {
    /// Creates a new instance with default values.
    pub fn new() -> Self {
        Self {
            previously_executed_count: 0,
            previous_module_id: None,
        }
    }
}

impl Default for RuntimeInformation {
    fn default() -> Self {
        Self::new()
    }
}
