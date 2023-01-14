use crate::{
    lewp_storage,
    resources::{
        Css,
        CssRegister,
        CssRegisterOptions,
        Entireness,
        ProcessedComponent,
    },
    storage::{Level, ResourceType, Storage, StorageComponent},
};

lewp_storage!(TestStorage, "testfiles");

#[test]
fn css_components_and_register() {
    let mut test =
        TestStorage::collect_component_ids(ResourceType::Css, Level::Component)
            .unwrap();
    test.sort();
    assert_eq!(test, vec!["footer", "hello-world", "navigation"],);
    let mut test =
        TestStorage::collect_component_ids(ResourceType::Css, Level::Page)
            .unwrap();
    test.sort();
    assert_eq!(test, vec!["sitemap"]);
    let c = Css::new("sitemap".into(), Level::Page);
    let parsed_component = ProcessedComponent::new::<TestStorage>(&c).unwrap();
    println!(
        "Parsed render critical: {:#?}",
        parsed_component.render_critical()
    );

    let r =
        CssRegister::new::<TestStorage>(CssRegisterOptions::default()).unwrap();
    let css = r
        .query("sitemap".into(), Level::Page, Entireness::Full)
        .unwrap();
    println!("Queried from register: {css:#?}");
}

#[test]
fn isolate_css_module() {
    use crate::storage::Level;

    let css = Css::new("hello-world".into(), Level::Component);
    let stylesheet = match css.content::<TestStorage>(()) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    assert_eq!(
        stylesheet.to_css_string(true),
        String::from("header.hello-world{border: thin solid black}.hello-world h1{font-style: bold}.hello-world h2{font-style: italic}")
        );
}
