use {
    crate::{
        fh::{
            Component as FHComponent,
            ComponentInformation,
            ComponentType,
            FileHierarchy,
            Level,
        },
        file_hierarchy,
        resources::{
            Css,
            CssRegister,
            CssRegisterOptions,
            Entireness,
            ProcessedComponent,
        },
    },
    std::sync::Arc,
};

file_hierarchy!(TestHierarchy, "testfiles");

#[test]
fn css_components_and_register() {
    let mut test = TestHierarchy::collect_component_ids(
        ComponentType::Css,
        Level::Component,
    )
    .unwrap();
    test.sort();
    assert_eq!(test, vec!["footer", "hello-world", "navigation"],);
    let mut test =
        TestHierarchy::collect_component_ids(ComponentType::Css, Level::Page)
            .unwrap();
    test.sort();
    assert_eq!(test, vec!["sitemap"]);
    let component_information = Arc::new(ComponentInformation {
        id: "sitemap".to_string(),
        level: Level::Page,
        kind: ComponentType::Css,
    });
    let c = Css::new(component_information.clone());
    let parsed_component =
        ProcessedComponent::new::<TestHierarchy>(&c).unwrap();
    println!(
        "Parsed render critical: {:#?}",
        parsed_component.render_critical()
    );

    let r = CssRegister::new::<TestHierarchy>(CssRegisterOptions::default())
        .unwrap();
    let css = r.query(component_information, Entireness::Full).unwrap();
    println!("Queried from register: {css:#?}");
}

#[test]
fn isolate_css_module() {
    use crate::fh::Level;

    let css = Css::new(Arc::new(ComponentInformation {
        id: String::from("hello-world"),
        level: Level::Component,
        kind: ComponentType::Css,
    }));
    let stylesheet = match css.content::<TestHierarchy>(()) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    assert_eq!(
        stylesheet.to_css_string(true),
        String::from("header.hello-world{border: thin solid black}.hello-world h1{font-style: bold}.hello-world h2{font-style: italic}")
        );
}
