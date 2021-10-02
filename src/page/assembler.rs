//! Defines the behavior of assembling the web page.

use {
    super::Page,
    crate::dom::{Node, NodeCreator, Nodes, RcDom},
    std::rc::Rc,
};

/// Provides functions required to assemble a web page.
pub trait Assembler: Page {
    /// Assembles the `<head>` tag of the page.
    fn head(&self) -> Rc<Node> {
        let head = NodeCreator::element("head", vec![]);
        head.children
            .borrow_mut()
            .push(NodeCreator::charset(&self.charset()));
        if self.config().viewport_tag {
            head.children.borrow_mut().push(NodeCreator::viewport());
        }
        head.children
            .borrow_mut()
            .push(NodeCreator::title(self.title()));
        head.children
            .borrow_mut()
            .push(NodeCreator::description(self.description()));

        for module in self.modules() {
            let module = module.borrow();
            for head_tag in module.head_tags() {
                head.children.borrow_mut().push(head_tag.clone());
            }
        }
        head
    }

    /// Assembles the `<body>` tag of the page.
    fn body(&self, modules: Vec<Nodes>) -> Rc<Node> {
        let body = NodeCreator::element("body", vec![]);
        for module in modules {
            for node in module {
                body.children.borrow_mut().push(node);
            }
        }
        body
    }

    /// Assembles the full page and returns it as [RcDom].
    fn full(&self, modules: Vec<Nodes>) -> RcDom {
        let dom = RcDom::default();
        let doctype = NodeCreator::doctype_html();

        let html = NodeCreator::html(self.language());

        let head = self.head();
        let body = self.body(modules);

        html.children.borrow_mut().push(head);
        html.children.borrow_mut().push(body);

        dom.document.children.borrow_mut().push(doctype);
        dom.document.children.borrow_mut().push(html);
        dom
    }
}
