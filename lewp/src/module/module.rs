use {
    super::{ModulePtr, RuntimeInformation},
    crate::{config::ModuleConfig, LewpError},
    lewp_html::{api::div, NodeExt, Nodes},
    std::{cell::RefCell, rc::Rc, sync::Arc},
};

/// Defines a web page module.
pub trait Module {
    /// Returns the unique module id.
    ///
    /// Allowed characters for id are `[a-z]`, `[0-9]` and `-`.
    /// **There is currently no check wether other characters are used. So please
    /// make sure that you do not use any other characters while creating modules.**
    fn id(&self) -> &str;

    /// The configuration of the module.
    fn config(&self) -> &ModuleConfig;

    /// Borrows the head tags that are required to run this module in a web page.
    fn head_tags(&self) -> &Nodes;

    /// Wraps `self` into a `Rc<RefCell<>>`
    fn into_module_ptr(self) -> ModulePtr
    where
        Self: Sized + 'static,
    {
        Rc::new(RefCell::new(self))
    }

    /// Constructs the view of the module.
    fn view(&self) -> Nodes;

    /// Renders as DOM nodes.
    fn render(&self) -> Nodes {
        if self.config().skip_wrapper {
            return self.view();
        }
        vec![div(self.view()).attrs(vec![
            ("class", self.id()),
            ("data-lewp-component", "module"),
        ])]
    }

    /// Executes the module. Main function that is able to collect and modify
    /// data required for rendering.
    fn run(
        &mut self,
        runtime_info: Arc<RuntimeInformation>,
    ) -> Result<(), LewpError>;
}
