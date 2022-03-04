#[macro_use]
extern crate rocket;

pub(crate) mod modules;
mod pages;

use {
    lewp::{
        css::Component as CssComponent,
        fh::{
            Component,
            ComponentInformation,
            ComponentType,
            FileHierarchyBuilder,
            Level,
        },
        page::Page,
    },
    pages::Index,
    std::{path::PathBuf, rc::Rc},
};

#[get("/home")]
fn index() -> (rocket::http::ContentType, String) {
    let mut index = Index::new();
    (rocket::http::ContentType::HTML, index.build())
}

#[get("/css/<module_id>")]
fn css(module_id: String) -> (rocket::http::ContentType, String) {
    let fh = Rc::new(
        FileHierarchyBuilder::new()
            .mountpoint(PathBuf::from("."))
            .build(),
    );
    let ci = Rc::new(ComponentInformation {
        id: module_id,
        level: Level::Module,
        kind: ComponentType::Css,
    });
    let css = CssComponent::new(ci.clone(), fh.clone());
    let stylesheet = css.content(()).unwrap();
    (
        rocket::http::ContentType::CSS,
        stylesheet.to_css_string(false),
    )
}

#[launch]
fn server() -> _ {
    rocket::build().mount("/", routes![index, css])
}
