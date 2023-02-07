use {
    lewp::{
        component::{Component, ComponentId, ComponentModel},
        page::{Page, PageId, PageModel},
        view::PageView,
    },
    lewp_html::{api::script, Node, NodeList},
};

// This component does not add data to the HTML body, only specifies a head node.
struct HeadComponent;

impl HeadComponent {
    pub fn new() -> Self {
        Self {}
    }
}

// Implement the [ComponentModel] trait to define the behavior and view.
impl ComponentModel for HeadComponent {
    // No message required for a simple component.
    type Message = ();

    // The unique ID of your component is used to identify and process further
    // resources, as well as isolation in the world of JavaScript on client side.
    fn id(&self) -> ComponentId {
        "head-component".into()
    }

    // There is no reason your page should fail. It should always render
    // at least something. Errors during processing should already be
    // handled before you call your page to be rendered.
    fn main(&mut self) {}

    // This is the view of your component.
    fn view(&self) -> Option<Node> {
        None
    }

    fn head(&self) -> NodeList {
        let mut head = NodeList::new();
        head.push(script(lewp_html::Script::Inline(
            "console.log(\"This component only adds a head node to the page.\")",
        )));
        head
    }
}

// This page will only contain a head node.
struct HeadOnlyPage;

impl PageModel for HeadOnlyPage {
    // Throughout your site, the page id should be unique for the same reason as
    // the component id.
    fn id(&self) -> PageId {
        "head-only-page".into()
    }

    // The main method of the page. In here you can add your components to the
    // page and do whatever processing is required for your page to be rendered.
    fn main(&self, view: &mut PageView) {
        let mut comp = Component::from(HeadComponent::new());
        // the component is only borrowed, to enable the possibility of adding
        // it twice to your page. You can use the state of your component to
        // define the behavior when adding it multiple times.
        view.push(&mut comp);
        // head nodes are only added !ONCE! for every component intentionally
        view.push(&mut comp);
    }
}

fn main() {
    simple_logger::init().unwrap();
    let hello_world = HeadOnlyPage {};
    let page = Page::from(hello_world);
    println!("{}", page.main().render());
}
