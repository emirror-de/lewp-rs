use {
    super::{ModulePtr, RuntimeInformation},
    crate::{
        config::ModuleConfig,
        dom::{NodeCreator, Nodes},
        LewpError,
    },
    std::{cell::RefCell, rc::Rc},
};

/// Defines a web page module.
pub trait Module {
    /// Returns the unique module id.
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
        let module_dom = self.view();
        if !self.config().skip_wrapper {
            let wrapper = NodeCreator::element(
                "div",
                vec![
                    NodeCreator::attribute("class", self.id()),
                    NodeCreator::attribute("data-lewp-component", "module"),
                ],
            );
            for node in module_dom {
                wrapper.children.borrow_mut().push(node);
            }
            return vec![wrapper];
        }
        module_dom
    }

    /// Executes the module. Main function that is able to collect and modify
    /// data required for rendering.
    fn run(
        &mut self,
        runtime_info: Rc<RuntimeInformation>,
    ) -> Result<(), LewpError>;
}
