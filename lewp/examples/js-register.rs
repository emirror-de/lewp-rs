use {
    lewp::{
        component::{Component, ComponentId},
        fh::FileHierarchyBuilder,
        html::{
            api::{h1, text},
            Node,
        },
        js::RegisterOptions as JsRegisterOptions,
        page::{Page, PageId},
        view::PageView,
    },
    std::{path::PathBuf, sync::Arc},
};

// Your hello world component.
struct HelloWorld {
    data: String,
}

impl HelloWorld {
    pub fn new() -> Self {
        Self {
            data: String::from("Hello World!"),
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
    fn main(&mut self) {}

    // This is the view of your component.
    fn view(&self) -> Option<Node> {
        Some(h1(vec![text(&self.data)]))
    }
}

// Define your page. This simple example page only contains one component that
// only specifies a h1 node.
struct HelloWorldPage;

impl Page for HelloWorldPage {
    // Throughout your site, the page id should be unique for the same reason as
    // the component id.
    fn id(&self) -> PageId {
        "sitemap".into()
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
    let fh = Arc::new(
        FileHierarchyBuilder::new()
            .mountpoint(PathBuf::from("./lewp/testfiles"))
            .build(),
    );
    let hello_world = HelloWorldPage {};
    let page = Page::new(hello_world)
        .with_file_hierarchy(Arc::clone(&fh))
        .with_js_register(JsRegisterOptions::default())
        .unwrap();
    println!("{}", page.main().render());
}
