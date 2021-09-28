//! Traits and data structures to create, run, assemble and render a web page.

use {
    crate::module::{ModulePtr, Modules, RuntimeInformation},
    std::rc::Rc,
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
    fn add_module(&mut self, module: ModulePtr) {
        self.modules_mut().push(module);
    }

    /// Executes all implemented functions and renders the page afterwards.
    fn execute(&mut self) -> String
    where
        Self: Assembler,
    {
        let runtime_information = Rc::new(RuntimeInformation::new());
        let mut modules_rendered_dom = vec![];
        // all modules
        for module in self.modules() {
            let mut module = module.borrow_mut();
            // run
            if let Err(e) = module.run(runtime_information.clone()) {
                log::error!(
                    "Module with id \"{}\" returned an error: {:#?}",
                    module.id(),
                    e
                );
            }
            // render
            modules_rendered_dom.push(module.render());

            // update runtime information
            runtime_information.increase_execution_count(module.id());
            runtime_information.set_previous_module_id(module.id());
        }
        self.render(modules_rendered_dom)
    }
}
