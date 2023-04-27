# How do components work?

Every component represents isolated content on your web page. This starts by
the generation of HTML code on the server and ends mainly with CSS and JavaScript
on the client side. By implementing [ComponentModel] you define the behavior
as well as all dependencies of your component.

**Components are executed once**. When the state of your component should be updated
later in the generation of your page, you can use the [ComponenModel::Message] type
in combination with the [ComponentModel::update] method. See below for more information.

# The navigation bar example

A component can represent anything you can imagine for your web page. Lets have
a closer look to a navigation bar example. Lets assume we want to create a simple
navigation bar with a list of links with the possibility to highlight the currently
selected navigation link.

For other basic examples have a look in the
[examples folder](https://github.com/emirror-de/lewp-rs/tree/main/lewp/examples)
in the repository.

## Define your component state

The state of your component is usually represented by a struct. We keep it simple for
our case:
```rust
struct NavigationBar {
    pub selected_index: usize,
}
```
Our navigation bar state only contains the index of the selected link.

## Define the behavior, implement `ComponentModel`

To be able to use our `NavigationBar` struct with `lewp-rs` it is required to
implement the [ComponentModel] trait. For the sake of simplicity, this example
does only implement mandatory methods.
```rust
# use lewp::{
#     component::{Component, ComponentId, ComponentModel},
#     html::{
#         api::{nav, ul, li, a, text},
#         Node,
#         NodeExt,
#     },
#     page::{Page, PageId},
#     view::PageView,
# };
# struct NavigationBar {
# 	pub selected_index: usize,
# }
impl ComponentModel for NavigationBar {
    type Message = ();
    fn id(&self) -> ComponentId {
        "navigation-bar".into() // used for isolation and identification in an archive
    }
    fn main(&mut self) {
        // We do not need to process anything in here for our case.
        // This function may be transformed async in future release to be able
        // to grab data from a database for example.
    }
    fn view(&self) -> Option<Node> {
        let mut list_items = vec![
            li(vec![a("/", vec![text("Home")])]),
            li(vec![a("/admin", vec![text("Administrator")])]),
            li(vec![a("/faq", vec![text("FAQ")])]),
        ];
        // Accessing indices directly without previous checks is of course bad practice,
        // but simple enough for demonstration in our case.
        list_items[self.selected_index].borrow_attr("class", "selected");
				
        Some(nav(vec![ul(list_items)]))
    }
}
```

Thats basically it. Your component now renders a list of three links with the
selected one having a `class` attribute with the value `selected`.

### Why `Option` as return type of `view`?

In `lewp` it is also possible to create "head-only" components, meaning they
do not get rendered inside the `<body>` tag of your page, but exist inside `<head>`.
At first glance this seems to be a bit more of boilerplate, but enables more flexible
use cases for `lewp`. By using "head-only" components it is possible to include
scripts that do not have visible DOM nodes attached to your page.

To create a "head-only" component, simply return `None` in the `view` method.
An example can be found in the
[examples](https://github.com/emirror-de/lewp-rs/tree/main/lewp/examples)
folder in the repository.

### The `ComponentModel::Message` type

The associated `Message` type of a component is used to pass messages to the
implementing component. With this type it is possible to update components that
have been previously executed. To make use of this feature, you need to implement
the [update](ComponentModel::update) method.

# Nested components

It is also possible to nest multiple components. Please have a look at the
[nested component](https://github.com/emirror-de/lewp-rs/blob/main/lewp/examples/nested-component.rs)
example in the repository, as this is a bit more noisy. Please pay special attention
to the [ComponentModel::nested_view] method when nesting components.
In addition to that, make sure that you implement [ComponentModel::dependency_list]
if you do have nested components.

# How do I add `CSS` or `JavaScript` to my component?

`lewp-rs` by design does not support the definition of `CSS` or `JavaScript` within
your Rust component. To use `CSS` or `JavaScript` on the client side, an
[Archive](super::archive::Archive) is required. Please have a look at the
[Css ArchiveComponent](crate::resources::Css) and
[Js ArchiveComponent](crate::resources::Js) documentation as well.

# How do I add a component to the page?

Please have a closer look to the [page](super::page) documentation.
