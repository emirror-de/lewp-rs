//! A component can be used to create self-contained areas of your web page, for example a navigation bar.
#![doc = include_str!("../docs/component.md")]

mod dependency_list;
mod details;

pub use {dependency_list::DependencyList, details::ComponentDetails};

use {
    crate::{
        html::{Node, NodeList},
        view::ComponentView,
    },
    std::{
        cell::{Ref, RefCell},
        rc::Rc,
    },
};

/// Implement this trait to create a component for your web page. This trait
/// also pre-defines some defaults like the dependency list. Make sure that
/// you implement these methods as well if you deviate from the defaults.
pub trait ComponentModel
where
    Self: Sized,
{
    /// The message that is can be used to update the component's state.
    type Message;
    /// Returns the [ComponentId] of the component.
    fn id(&self) -> ComponentId;
    /// Updates the component state by using the given message. This method
    /// is not executed automatically, it needs to be called by the user's
    /// implementation. Mostly, it is called from another component's
    /// [main](Component::main) method.
    fn update(&mut self, _message: Self::Message) {}
    /// The main method designing the behavior of the component. Does nothing by
    /// default.
    fn main(&mut self) {}
    /// Defines the view of the component.
    fn view(&self) -> Option<Node>;
    /// Use this when rendering a nested component. This ensures that all
    /// required attributes are set to make your `JavaScript` and `CSS` work on client side.
    fn nested_view(&self) -> Option<Node> {
        match self.view() {
            Some(v) => {
                v.to_component_view(self.id());
                Some(v)
            }
            None => None,
        }
    }
    /// Defines the additional head nodes this component requires.
    ///
    /// Defaults to an empty [NodeList].
    fn head(&self) -> NodeList {
        NodeList::new()
    }
    /// Returns the dependencies of the implementing component. If you are
    /// using other components within your component, you will need to add
    /// its ID to the dependency list by implementing this method.
    ///
    /// Defaults to an empty [DependencyList].
    fn dependency_list(&self) -> DependencyList {
        DependencyList::default()
    }
}

/// A unique component ID.
pub type ComponentId = String;

/// A component that is used to create web pages. This struct can be created from
/// a [ComponentModel].
pub struct Component<C>
where
    C: ComponentModel,
{
    ///// Contains head nodes required by the component.
    head: Rc<RefCell<NodeList>>,
    /// An instance of the model that is implemented by the user.
    model: Rc<RefCell<C>>,
    /// Contains the rendered view. This view gets initially created when the
    /// [main](Self::main) or [update](Self::update) method is called.
    view: Rc<RefCell<Option<Node>>>,
}

impl<C> Component<C>
where
    C: ComponentModel,
{
    /// Executes and renders the component by calling its
    /// [main](Component::main) method and updates its view.
    pub fn main(&mut self) {
        log::debug!("Running component \"{}\"", self.model.borrow().id());
        self.model.borrow_mut().main();
        self.update_content();
    }

    /// Updates the model by calling the models [update](Component::update)
    /// method using the given message.
    pub fn update(&mut self, message: <C as ComponentModel>::Message) {
        log::debug!(
            "Updating component state \"{}\"",
            self.model.borrow().id()
        );
        self.model.borrow_mut().update(message);
        self.update_content();
    }

    /// Updates the content of the component. This is called in [Self::main],
    /// as well as in [Self::update] (required for nested components).
    fn update_content(&mut self) {
        self.update_view();
        self.update_head();
    }

    /// Updates the view by calling model's [view](Component::view) method.
    /// This is especially required for nested components as the reference to
    /// the view gets stored in the [PageView] instance.
    fn update_view(&mut self) {
        log::debug!("Updating view for \"{}\"", self.model.borrow().id());
        let mut view = self.view.borrow_mut();
        *view = self.model.borrow().view();
    }

    /// Updates the head nodes by calling model's [head](Component::head) method.
    /// This is especially required for nested components as the reference to
    /// the head gets stored in the [PageView] instance.
    fn update_head(&mut self) {
        log::debug!("Updating head nodes for \"{}\"", self.model.borrow().id());
        let mut head = self.head.borrow_mut();
        *head = self.model.borrow().head();
    }

    /// Returns a clone of the given component view. This is for internal use
    /// only because it reveals a [RefCell] to the user.
    /// This method is called by the [HtmlPage] for further processing.
    pub(crate) fn view(&self) -> Rc<RefCell<Option<Node>>> {
        let view = self.view.borrow_mut();
        match *view {
            Some(ref v) => v.to_component_view(self.id()),
            None => (),
        };
        Rc::clone(&self.view)
    }

    /// Returns a clone of the given component head. This is for internal use
    /// only because it reveals a [RefCell] to the user.
    /// This method is called by the [HtmlPage] for further processing.
    pub(crate) fn head(&self) -> Rc<RefCell<NodeList>> {
        Rc::clone(&self.head)
    }

    /// Returns a borrowed reference of the model.
    pub fn model(&self) -> Ref<C> {
        self.model.borrow()
    }

    /// Returns the [ComponentId] filtered by allowed characters.
    /// The resulting ID is converted to lowercase. Disallowed characters get
    /// filtered.
    ///
    /// Allowed characters are: `[a-z]`, `[0-9]` and `-`.
    pub fn id(&self) -> ComponentId {
        self.model
            .borrow()
            .id()
            .to_lowercase()
            .chars()
            .filter(|x| x.is_alphanumeric() || x == &'-')
            .collect()
    }

    /// Returns the dependency list of the component.
    pub fn dependency_list(&self) -> DependencyList {
        let mut list = DependencyList::from(self.model().id());
        list.append(self.model().dependency_list());
        list
    }
}

impl<C: ComponentModel> From<C> for Component<C> {
    fn from(model: C) -> Self {
        log::debug!("Creating new component for model: \"{}\"", model.id());
        let model = Rc::new(RefCell::new(model));
        let view = Rc::new(RefCell::new(None));
        let head = Rc::new(RefCell::new(NodeList::new()));
        Self { model, view, head }
    }
}
