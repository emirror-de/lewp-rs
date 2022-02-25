//! Implementation of a generic HTML5 node.

use std::{cell::RefCell, rc::Rc};

/// A HTML5 Node.
pub type Node = Rc<rcdom::Node>;

impl crate::NodeExt for rcdom::Node {
    fn children(&self) -> &RefCell<Vec<Node>> {
        &self.children
    }
    fn data(&self) -> &rcdom::NodeData {
        &self.data
    }
    fn categories(&self) -> Vec<rcdom::ContentCategory> {
        (self.content_categories)(self)
    }
    fn append_children(&self, children: Vec<Node>) {
        for child in children {
            self.append_child(child);
        }
    }
    #[cfg(not(debug_assertions))]
    fn append_child(&self, child: Node) {
        self.children.borrow_mut().push(child);
    }
    #[cfg(debug_assertions)]
    fn append_child(&self, child: Node) {
        let content_model = (self.content_model)(self, child.clone());
        for content in content_model {
            use rcdom::{ContentCategory, ContentModel::*};
            match content {
                Specific(b) => {
                    if !b {
                        error!(
                            "Cannot append child because it fails a specific content model check:\nParent:\n{:#?}\nChild:\n{:#?}",
                            self,
                            child
                            );
                        return;
                    }
                }
                Category(category) => {
                    if &category == &ContentCategory::Transparent {
                        // check all children if they are allowed
                    }
                    if !self.categories().contains(&category) {
                        // do not add if category not allowed
                        error!(
                            "Cannot append child because the category {:?} is not allowed!\nAllowed categories:\n{:?}",
                            category,
                            self.categories()
                            );
                        return;
                    }
                }
            }
        }
        self.children.borrow_mut().push(child);
    }
}
