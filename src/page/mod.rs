//! Traits and data structures to create, run, assemble and render a web page.

use {
    crate::module::{Module, Modules, RuntimeInformation},
    std::{collections::HashMap, rc::Rc},
};

mod assembler;
mod metadata;
mod render;
mod runtime;

pub use {assembler::Assembler, metadata::Metadata, render::Render, runtime::Runtime};

/// Main trait of a page.
pub trait Page: Metadata + Runtime + Render {
    /// Returns a reference to the modules added to the page.
    fn modules(&self) -> &Modules;

    /// Returns a mutable reference to the modules added to the page.
    fn modules_mut(&mut self) -> &mut Modules;

    /// Adds the module to the page. The page is rendered FIFO.
    fn add_module(&mut self, module: Box<dyn Module>) {
        self.modules_mut().push(module);
    }

    /// Executes all implemented functions and renders the page afterwards.
    fn execute(&mut self) -> String
    where
        Self: Assembler,
    {
        let mut runtime_information = Box::new(RuntimeInformation::new());
        let mut modules_rendered_dom = vec![];
        // all modules
        for module in self.modules_mut() {
            // run
            if let Err(e) = module.run(&mut runtime_information) {
                log::error!(
                    "Module with id \"{}\" returned an error: {:#?}",
                    module.id(),
                    e
                );
            }
            // render
            modules_rendered_dom.push(module.render());

            // update runtime information
            let id = module.id().to_owned();
            runtime_information.increase_execution_count(module.id());
            runtime_information.previous_module_id = Some(module.id().to_string());
        }
        self.render(modules_rendered_dom)
    }
}
