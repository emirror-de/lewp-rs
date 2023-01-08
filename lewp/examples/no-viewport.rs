use lewp::{
    html::Node,
    page::{Page, PageId},
    view::PageView,
};

// This page will not contain a <meta name="viewport" ...> node.
struct NoViewportPage;

impl Page for NoViewportPage {
    // Throughout your site, the page id should be unique for the same reason as
    // the component id.
    fn id(&self) -> PageId {
        "no-viewport-page".into()
    }

    // The view does not change.
    fn main(&self, _view: &mut PageView) {}

    fn viewport(&self) -> Option<Node> {
        None
    }
}

fn main() {
    simple_logger::init().unwrap();
    let no_viewport = NoViewportPage {};
    let page = Page::new(no_viewport);
    println!("{}", page.main().render());
}
