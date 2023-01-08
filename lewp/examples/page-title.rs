use lewp::{
    page::{Page, PageId},
    view::PageView,
};

// This page will only contain a title node in <head>.
struct DescriptivePage;

impl Page for DescriptivePage {
    // Throughout your site, the page id should be unique for the same reason as
    // the component id.
    fn id(&self) -> PageId {
        "descriptive-page".into()
    }

    // The view does not change.
    fn main(&self, _view: &mut PageView) {}

    fn title(&self) -> String {
        "A new, custom titled web page.".into()
    }

    fn description(&self) -> String {
        "Describe the page you are creating here.".into()
    }
}

fn main() {
    simple_logger::init().unwrap();
    let descriptive_page = DescriptivePage {};
    let page = Page::new(descriptive_page);
    println!("{}", page.main().render());
}
