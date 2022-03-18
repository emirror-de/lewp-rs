//! Runtime traits and structs of a module.

use {
    crate::{fh::FileHierarchy, LewpError},
    std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc},
};

/// Contains runtime information of a module.
pub struct RuntimeInformation {
    /// Information about how often a module has been executed.
    module_execution_count: RefCell<HashMap<String, u32>>,
    /// The id of the previously executed module. Set to None if it is the first module that is
    /// being executed.
    previous_module_id: RefCell<Option<String>>,
    /// The file hierarchy where the module is working on.
    pub fh: Option<Arc<FileHierarchy>>,
}

impl RuntimeInformation {
    /// Creates a new instance with default values.
    pub fn new(fh: Option<Arc<FileHierarchy>>) -> Self {
        Self {
            module_execution_count: RefCell::new(HashMap::<String, u32>::new()),
            previous_module_id: RefCell::new(None),
            fh,
        }
    }

    /*
    /// Sets the file hierarchy.
    pub fn set_file_hierarchy(&mut self, fh: Arc<FileHierarchy>) {
        self.fh = Some(fh);
    }
    */

    /// Increases the execution count for the given module id by 1.
    pub fn increase_execution_count(&self, id: &str) {
        let execution_count = self.get_execution_count(id);
        self.module_execution_count
            .borrow_mut()
            .insert(id.to_string(), execution_count + 1);
    }

    /// Returns the count how often a module with the given id has been executed.
    pub fn get_execution_count(&self, id: &str) -> u32 {
        self.module_execution_count
            .borrow()
            .get(id)
            .unwrap_or(&0)
            .to_owned()
    }

    /// Returns the id of the module that run previously.
    pub fn previous_module_id(&self) -> Option<String> {
        self.previous_module_id.borrow().to_owned()
    }

    /// Sets the id of the module that run previously.
    pub fn set_previous_module_id(&self, module_id: &str) {
        *self.previous_module_id.borrow_mut() = Some(module_id.to_string());
    }
}

impl Default for RuntimeInformation {
    fn default() -> Self {
        Self::new(None)
    }
}
