//! View related models.
//!
//! These models are shown here for reference and are not required to be created
//! by the user.

use {
    crate::{
        component::{Component, ComponentId, ComponentModel, DependencyList},
        html::{Node, NodeExt, NodeList},
    },
    std::{cell::RefCell, rc::Rc},
};

/// Defines required additions for a [Node] to be a view of a component.
pub trait ComponentView {
    /// Prepares the [Node] to be considered a view of a component if required.
    fn to_component_view(&self, id: ComponentId);
}

impl ComponentView for Node {
    fn to_component_view(&self, id: ComponentId) {
        self.add_class(&id);
        self.borrow_attr("data-lewp-type", "component");
    }
}

/// A complete web page view. Contains all `HTML` nodes as well as parameters
/// required to render a valid `HTML` page.
#[derive(Default)]
pub struct PageView {
    /// The `<head>` tag content.
    ///
    /// Every entry of the [Vec] corresponds to a component. Because the component
    /// itself cannot be stored, a [Rc] is passed to the view.
    head: Vec<Rc<RefCell<NodeList>>>,
    /// The `<body>` tag content.
    body: Vec<Rc<RefCell<Option<Node>>>>,
    /// The component dependency list of the page.
    dependency_list: DependencyList,
}

impl PageView {
    /// Appends the component to the page view.
    pub fn push<C: ComponentModel>(
        &mut self,
        component: &mut Component<C>,
    ) -> &mut Self {
        log::debug!("Added component \"{}\"", component.id());
        component.main();
        self.body.push(component.view());

        if !&self.dependency_list.contains(component.id()) {
            log::debug!(
                "Storing head tags reference for ID \"{}\"",
                component.id()
            );
            self.head.push(component.head());
        }

        log::debug!("Processing dependencies for ID \"{}\"", component.id());
        let mut dependencies = component.dependency_list();
        dependencies.push(component.model().id());
        log::debug!(
            "Adding dependencies for component \"{}\": \"{}\"",
            component.model().id(),
            dependencies
        );
        self.dependency_list.append(dependencies);

        self
    }

    /// Collects the children of the `<body>` tag of the current page view.
    pub fn body(self) -> NodeList {
        self.body
            .into_iter()
            .filter(|n| n.borrow().is_some())
            .map(|n| n.borrow().as_ref().unwrap().to_owned())
            .collect()
    }

    /// Collects the children of the `<head>` tag of the current page view.
    pub fn head(&self) -> NodeList {
        self.head.iter().fold(NodeList::new(), |mut acc, h| {
            acc.append(&mut h.borrow_mut());
            acc
        })
    }

    /// Returns a reference to the component dependency list.
    pub fn dependency_list(&self) -> &DependencyList {
        &self.dependency_list
    }
}
