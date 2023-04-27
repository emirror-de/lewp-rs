# How do pages work?

A page represents surprisingly, a HTML5 page. Every page consists of one or
more [components](crate::component). By adding a component to your page,
`lewp` knows which dependencies are required and automatically adds the required
tags to the final document (only if the files are available on the file system).
However, `lewp` is designed to be very flexible
about the integration of your CSS or JavaScript dependencies.
To enable this, an [Archive](crate::archive::Archive)
or [ArchiveCache](crate::archive::ArchiveCache) (depending on your use case)
is required to define the required routes for your dependencies.

This setup makes it very easy to rapidly develop SEO friendly, high quality web pages.
In addition to that, metadata of your web page can be added very easily. All
you need to do is to implement the responsible method in the [PageModel] trait.
But we will get to this lateron.

# The homepage and Hello World! example

Lets have a closer look on how a [PageModel] is defined.

```rust
# use lewp::{
#     component::{Component, ComponentId, ComponentModel},
#     html::{
#         api::{h1, text},
#         Node,
#     },
#     page::{PageModel, PageId, Page},
#     view::PageView,
# };
#
# #[derive(Default)]
# struct HelloWorld;
#
# impl ComponentModel for HelloWorld {
#     type Message = ();
#     fn id(&self) -> ComponentId {
#         "hello-world".into()
#     }
#     fn main(&mut self) {}
#     fn view(&self) -> Option<Node> {
#         Some(h1(vec![text("Hello World!")]))
#     }
# }
#
// Define your page model. From this struct you can generate a page as you will
// see below.
#[derive(Default)]
struct HomePage;

impl PageModel for HomePage {
    // Throughout your site, the page id should be unique for the same reason as
    // the component id. Use lower kebab-case here as convention.
    fn id(&self) -> PageId {
        "home-page".into()
    }
    // The main method of the page. In here you can add your components to the
    // page and do whatever processing is required for your page to be rendered.
    // [This method may become async in the future]
    fn main(&self, view: &mut PageView) {
        // Create your component that produces a h1 tag with "Hello World!" in it.
        let mut comp = Component::from(HelloWorld::default());
        // The component is only borrowed, to enable the possibility of adding
        // it twice to your page. You can use the state of your component to
        // define the behavior when adding it multiple times.
        // However, the required head nodes for example CSS and JS is being added
        // only once, so you can be sure that there is no overhead when adding
        // the component multiple times.
        view.push(&mut comp);
    }
}
```

As you can see, to get a first start with your page only [PageModel::id] and [PageModel::main]
methods are required. However, these are not the only ones available to it.
`lewp` already provides default implementations for all the other methods, so
you only need to implement them if you really want to use other values for it.
It is recommended to always change [PageModel::title] and [PageModel::description]
for your page.

Default implementations may increase during growth of `lewp`, so have a look
every now and then to the trait to stay up to date what you are able to implement.

You are only one step away to get your simple Hello World! page as HTML5 string:
```rust
# use lewp::{
#     component::{Component, ComponentId, ComponentModel},
#     html::{
#         api::{h1, text},
#         Node,
#     },
#     page::{PageModel, PageId, Page},
#     view::PageView,
# };
#
# #[derive(Default)]
# struct HelloWorld;
#
# impl ComponentModel for HelloWorld {
#     type Message = ();
#     fn id(&self) -> ComponentId {
#         "hello-world".into()
#     }
#     fn main(&mut self) {}
#     fn view(&self) -> Option<Node> {
#         Some(h1(vec![text("Hello World!")]))
#     }
# }
# #[derive(Default)]
# struct HomePage;
# impl PageModel for HomePage {
#     fn id(&self) -> PageId {
#         "home-page".into()
#     }
#     fn main(&self, view: &mut PageView) {
#         let mut comp = Component::from(HelloWorld::default());
#         view.push(&mut comp);
#     }
# }
// In your main method, which is usually the route handler of your web framework,
// crate the instance of your page, run and render it!
fn main() {
    // Create an instance of your page from your model.
    let prepared_page = Page::from(HomePage::default());
    // You have full control when you want to run and render your page.
    // Because the internal state of the page changes when running the main
    // method, you need to get the result in order to be able to render the
    // resulting page.
    // Please note, that the following main method is Page::main and NOT PageModel::main!
    let executed_page = prepared_page.main();
    println!("{}", executed_page.render());
}
```