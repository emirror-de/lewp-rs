use {
    lewp::{
        config::{ModuleConfig, PageConfig},
        css::{
            Component, Entireness, ProcessedComponent, Register,
            RegisterOptions,
        },
        dom::{NodeCreator, NodeExt, Nodes},
        fh::ComponentInformation,
        module::{Module, Modules, RuntimeInformation},
        page::Page,
        LewpError,
    },
    std::rc::Rc,
};

fn main() {
    let fh = lewp::fh::FileHierarchyBuilder::new()
        .mountpoint(std::path::PathBuf::from("./testfiles"))
        .build();
    println!(
        "{:#?}",
        fh.collect_component_ids(
            lewp::fh::ComponentType::Css,
            lewp::fh::Level::Module,
        )
    );
    let component_information = Rc::new(ComponentInformation {
        id: "hello-world".to_string(),
        level: lewp::fh::Level::Module,
        kind: lewp::fh::ComponentType::Css,
    });
    let c = Component::new(component_information.clone(), Rc::new(fh));
    let parsed_component = ProcessedComponent::from(&c).unwrap();
    println!("{:#?}", parsed_component.render_critical());

    let fh = lewp::fh::FileHierarchyBuilder::new()
        .mountpoint(std::path::PathBuf::from("./testfiles"))
        .build();
    let mut r = Register::new(fh, RegisterOptions {});
    r.load_process_components();
    let css = r
        .query(component_information.clone(), Entireness::Full)
        .unwrap();
    println!("{:#?}", css);
}
