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
    fn add_module(&mut self, module: Rc<dyn Module>) {
        self.modules_mut().push(module);
    }

    /// Executes all implemented functions and renders the page afterwards.
    fn execute(&mut self) -> String
    where
        Self: Assembler,
    {
        let mut runtime_options = RuntimeInformation::new();
        let mut module_register = HashMap::<String, u32>::new();
        let mut modules_rendered_dom = vec![];
        // all modules
        for module in self.modules_mut() {
            // run
            let module = match Rc::get_mut(module) {
                None => {
                    log::error!(
                        "Could not get mutable reference to module \"{}\". Skipping...",
                        module.id()
                    );
                    continue;
                }
                Some(m) => m,
            };
            if let Err(e) = module.run(&runtime_options) {
                log::error!(
                    "Module with id \"{}\" returned an error: {:#?}",
                    module.id(),
                    e
                );
            }
            // render
            modules_rendered_dom.push(module.render());
            let id = module.id().to_owned();
            module_register.insert(id, module_register.get(module.id()).unwrap_or(&0) + 1);
            runtime_options.previously_executed_count =
                *module_register.get(module.id()).unwrap_or(&0);
            runtime_options.previous_module_id = Some(module.id().to_string());
        }
        self.render(modules_rendered_dom)
    }
}