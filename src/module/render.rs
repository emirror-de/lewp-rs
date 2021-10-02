//! Traits and functions required for rendering a module or a page to DOM nodes.

use crate::{
    dom::{NodeCreator, Nodes},
    module::Metadata,
};

/// Functions to render `self` as DOM nodes.
pub trait Render: Metadata {
    /// Constructs the view of the module.
    fn view(&self) -> Nodes;
    /// Renders as DOM nodes.
    fn render(&self) -> Nodes {
        let module_dom = self.view();
        if !self.config().skip_wrapper {
            let wrapper = NodeCreator::element(
                "div",
                vec![NodeCreator::attribute(
                    "class",
                    &format!("lewp-module {}", self.id()),
                )],
            );
            for node in module_dom {
                wrapper.children.borrow_mut().push(node);
            }
            return vec![wrapper];
        }
        module_dom
    }
}
