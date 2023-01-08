//! View related models.

use {
    crate::{
        component::{
            Component,
            ComponentView,
            ComponentWrapper,
            DependencyList,
        },
        html::NodeList,
    },
    std::{cell::RefCell, rc::Rc},
};

/// A complete web page view. Contains all `HTML` nodes as well as parameters
/// required to render a valid `HTML` page.
pub struct PageView {
    /// The `<body>` tag content.
    body: Vec<Rc<RefCell<Option<ComponentView>>>>,
    /// The component dependency list of the page.
    dependency_list: DependencyList,
}

impl PageView {
    /// Appends the component to the page view.
    pub fn push<C: Component>(
        &mut self,
        component: &mut ComponentWrapper<C>,
    ) -> &mut Self {
        log::debug!("Added component \"{}\"", component.id());
        component.main();
        self.body.push(component.view());

        log::debug!("Processing dependencies for ID \"{}\"", component.id());
        let mut dependencies = component.dependency_list().clone();
        dependencies.push(component.model().id());
        log::debug!(
            "Adding dependencies for component \"{}\": \"{}\"",
            component.model().id(),
            dependencies
        );
        self.dependency_list.append(dependencies);
        self
    }

    /// Returns the children of the `<body>` tag of the current page view.
    pub fn body(self) -> NodeList {
        self.body
            .into_iter()
            .filter(|n| n.borrow().is_some())
            .map(|n| n.borrow().as_ref().unwrap().to_owned())
            .collect()
    }

    /// Returns a reference to the component dependency list.
    pub fn dependency_list(&self) -> &DependencyList {
        &self.dependency_list
    }
}

impl Default for PageView {
    fn default() -> Self {
        Self {
            body: vec![],
            dependency_list: DependencyList::default(),
        }
    }
}
