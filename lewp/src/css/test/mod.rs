use {
    crate::{
        css::{
            Component, Entireness, ProcessedComponent, Register,
            RegisterOptions,
        },
        fh::{
            self, Component as FHComponent, ComponentInformation,
            ComponentType, Level,
        },
    },
    std::rc::Rc,
};

#[test]
fn css_components_and_register() {
    let fh = crate::fh::FileHierarchyBuilder::new()
        .mountpoint(std::path::PathBuf::from("./testfiles"))
        .build();
    assert_eq!(
        fh.collect_component_ids(ComponentType::Css, Level::Module,)
            .unwrap()
            .sort(),
        vec!["footer", "hello-world", "navigation"].sort(),
    );
    assert_eq!(
        fh.collect_component_ids(ComponentType::Css, Level::Page,)
            .unwrap()
            .sort(),
        vec!["sitemap"].sort()
    );
    let component_information = Rc::new(ComponentInformation {
        id: "sitemap".to_string(),
        level: Level::Page,
        kind: ComponentType::Css,
    });
    let c = Component::new(component_information.clone(), Rc::new(fh));
    let parsed_component = ProcessedComponent::from(&c).unwrap();
    println!(
        "Parsed render critical: {:#?}",
        parsed_component.render_critical()
    );

    let fh = fh::FileHierarchyBuilder::new()
        .mountpoint(std::path::PathBuf::from("./testfiles"))
        .build();
    let mut r = Register::new(fh, RegisterOptions {});
    r.load_process_components().unwrap();
    let css = r
        .query(component_information.clone(), Entireness::Full)
        .unwrap();
    println!("Queried from register: {:#?}", css);
}

#[test]
fn isolate_css_module() {
    use crate::fh::Level;

    // get temporary directory
    let dir = tempfile::tempdir().unwrap();
    // base the file hierarchy to this directory
    let fh = crate::fh::FileHierarchyBuilder::new()
        .mountpoint(dir.path().to_path_buf())
        .build();

    // create path where the testfiles should be copied
    let testfiles_destination = dir.path().join("modules");
    let testfiles_source = "testfiles/modules";
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.copy_inside = true;
    match fs_extra::dir::copy(
        testfiles_source,
        testfiles_destination,
        &copy_options,
    ) {
        Err(msg) => panic!("{}", msg.to_string()),
        Ok(_) => (),
    };

    let css = Component::new(
        Rc::new(ComponentInformation {
            id: String::from("hello-world"),
            level: Level::Module,
            kind: ComponentType::Css,
        }),
        Rc::new(fh),
    );
    let stylesheet = match css.content(()) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    assert_eq!(
        stylesheet.to_css_string(true),
        String::from(".hello-world h1{font-style: bold}.hello-world h2{font-style: italic}")
        );

    match dir.close() {
        Err(msg) => panic!("{}", msg.to_string()),
        Ok(_) => (),
    }
}
