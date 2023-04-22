use {
    crate::{
        archive::{Archive, ArchiveCache},
        component::ComponentDetails,
        lewp_archive,
        resources::{
            Css,
            CssOptions,
            Resource,
            ResourceLevel,
            ResourceType,
            WebInterface,
        },
    },
    std::sync::Arc,
};

lewp_archive!(TestArchive, "testfiles");
impl WebInterface for TestArchive {}

#[test]
fn css_components_and_archive() {
    let mut test = TestArchive::collect_component_ids(
        ResourceType::Css,
        ResourceLevel::Component,
    )
    .unwrap();
    test.sort();
    assert_eq!(test, vec!["footer", "hello-world", "navigation"],);
    let mut test = TestArchive::collect_component_ids(
        ResourceType::Css,
        ResourceLevel::Page,
    )
    .unwrap();
    test.sort();
    assert_eq!(test, vec!["sitemap"]);
    let options = CssOptions {
        id: "sitemap".into(),
        level: ResourceLevel::Page,
    };
    let c = Arc::new(Resource::<Css>::load::<TestArchive>(options).unwrap());
    println!("Parsed render critical: {:#?}", c.content.render_critical);

    let mut a = ArchiveCache::default();
    a.insert(c);
    let details = ComponentDetails::new(
        "sitemap".into(),
        ResourceType::Css,
        ResourceLevel::Page,
    );
    let css = a.query::<Css>(&details).unwrap();
    println!("Queried from register: {css:#?}");
}

#[test]
fn isolate_css_module() {
    use crate::resources::ResourceLevel;

    let options = CssOptions {
        id: "hello-world".into(),
        level: ResourceLevel::Component,
    };
    let css = Resource::<Css>::load::<TestArchive>(options).unwrap();
    assert_eq!(
        *css.content.full,
        String::from("header.hello-world{border: thin solid black}.hello-world h1{font-style: bold}.hello-world h2{font-style: italic}")
        );
}
