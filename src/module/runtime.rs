//! Runtime traits and structs of a module.

use {crate::Error, std::collections::HashMap};

/// Defines the behavior during runtime.
pub trait Runtime {
    /// Executes the module. Main function that is able to collect and modify
    /// data required for rendering.
    fn run(&mut self, runtime_info: &mut Box<RuntimeInformation>) -> Result<(), Error>;
}

/// Contains runtime information of a module.
pub struct RuntimeInformation {
    /// Information about how often a module has been executed.
    module_execution_count: HashMap<String, u32>,
    /// The id of the previously executed module. Set to None if it is the first module that is
    /// being executed.
    pub previous_module_id: Option<String>,
}

impl RuntimeInformation {
    /// Creates a new instance with default values.
    pub fn new() -> Self {
        Self {
            module_execution_count: HashMap::<String, u32>::new(),
            previous_module_id: None,
        }
    }

    /// Increases the execution count for the given module id by 1.
    pub fn increase_execution_count(&mut self, id: &str) {
        self.module_execution_count
            .insert(id.to_string(), self.get_execution_count(id) + 1);
    }

    /// Returns the count how often a module with the given id has been executed.
    pub fn get_execution_count(&self, id: &str) -> u32 {
        self.module_execution_count.get(id).unwrap_or(&0).to_owned()
    }
}

impl Default for RuntimeInformation {
    fn default() -> Self {
        Self::new()
    }
}
