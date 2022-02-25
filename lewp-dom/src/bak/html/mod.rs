//! Typed HTML5 element nodes.

/// Content categories defined in the HTML5 spec.
mod content_category;

/// Content model posibilities defined in the HTML5 spec.
mod content_model;

/// The HTML5 node definitions.
pub mod nodes;

use crate::NodeExt;

pub use {content_category::ContentCategory, content_model::ContentModel};

/// Defines critical methods for a node to be typed.
pub trait TypedNode
where
    Self: NodeExt,
{
    /// Returns the node type.
    fn categories(&self) -> Vec<ContentCategory>;
    /// Returns true if the given node matches its content model, false otherwise.
    ///
    /// This method is used for validation of nodes that are appended and is only
    /// called in ***DEBUG BUILDS***.
    fn content_model(&self, node: &Self) -> Vec<ContentModel>;
    /// Appends the given  [Node] as children.
    fn append_child(&mut self, node: &Self) {
        #[cfg(debug_assertions)]
        if !self.validate(node) {
            return;
        }
        self.children().borrow_mut().push(node.clone());
    }
    /// Appends the given [Vec]tor of [Node]s as children.
    fn append_children(&mut self, nodes: &mut Vec<Self>) {
        for node in nodes {
            self.append_child(&node);
        }
    }
    /// Validates that the given node is in line with the HTML5 specification.
    fn validate(&self, node: &Self) -> bool {
        use content_model::ContentModel::*;
        // if the content model is transparent, all children of the given node
        // must be checked
        //for model in self.content_model(&node) {
        //    match model {
        //        Specific(b) => {
        //            if !b {
        //                return false;
        //            }
        //        }
        //        Category(c) => {
        //            if c == ContentCategory::Transparent {
        //                for child in node.children.borrow().iter() {
        //                    if !self.validate(child) {
        //                        return false;
        //                    }
        //                }
        //            }
        //        }
        //    }
        //}
        true
    }
}
