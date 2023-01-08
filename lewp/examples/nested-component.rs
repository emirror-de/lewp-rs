use lewp::{
    component::{Component, ComponentId, DependencyList},
    html::{
        api::{div, h1, h2, text},
        Node,
    },
    page::{Page, PageId},
    view::PageView,
};

// Your hello world component that has a nested component.
struct HelloWorld {
    data: String,
    nested: NestedComponent,
}

impl HelloWorld {
    pub fn new() -> Self {
        Self {
            data: String::from("Hello World!"),
            nested: NestedComponent {},
        }
    }
}

// Implement the [Component] trait to define the behavior and view.
impl Component for HelloWorld {
    // No message required for a simple component.
    type Message = ();

    // The unique ID of your component is used to identify and process further
    // resources, as well as isolation in the world of JavaScript on client side.
    fn id(&self) -> ComponentId {
        "hello-world".into()
    }

    // There is no reason your page should fail. It should always render
    // at least something. Errors during processing should already be
    // handled before you call your page to be rendered.
    fn main(&mut self) {
        // you can decide when to execute the nested component
        self.nested.main();
    }

    // This is the view of your component.
    fn view(&self) -> Option<Node> {
        // Make sure that you use `nested_view()` instead of the standard `view()`
        // method. If not, your JavaScript on the client side will not be working.
        let nested_view = self.nested.nested_view().unwrap();

        // You have full control where you insert the nested component's view.
        Some(div(vec![h1(vec![text(&self.data)]), nested_view]))
    }

    fn dependency_list(&self) -> DependencyList {
        let mut d = DependencyList::default();
        d.push(self.nested.id());
        d
    }
}

// Define your component that is nested in [HelloWorld].
struct NestedComponent;

// Implement the [Component] trait to define the behavior and view.
impl Component for NestedComponent {
    // No message required for a simple component.
    type Message = ();

    // The unique ID of your component is used to identify and process further
    // resources, as well as isolation in the world of JavaScript on client side.
    fn id(&self) -> ComponentId {
        "nested-component-id".into()
    }

    // There is no reason your page should fail. It should always render
    // at least something. Errors during processing should already be
    // handled before you call your page to be rendered.
    fn main(&mut self) {}

    // This is the view of your component.
    fn view(&self) -> Option<Node> {
        Some(h2(vec![text(
            "This is a text rendered as a nested component! :-)",
        )]))
    }
}

// Define your page. This simple example page only contains one component that
// only specifies a h1 node.
struct HelloWorldPage;

impl Page for HelloWorldPage {
    // Throughout your site, the page id should be unique for the same reason as
    // the component id.
    fn id(&self) -> PageId {
        "helloworldpage".into()
    }

    // The main method of the page. In here you can add your components to the
    // page and do whatever processing is required for your page to be rendered.
    fn main(&self, view: &mut PageView) {
        let mut comp = Component::new(HelloWorld::new());
        // the component is only borrowed, to enable the possibility of adding
        // it twice to your page. You can use the state of your component to
        // define the behavior when adding it multiple times.
        view.push(&mut comp);
    }
}

fn main() {
    simple_logger::init().unwrap();
    let hello_world = HelloWorldPage {};
    let page = Page::new(hello_world);
    println!("{}", page.main().render());
}
