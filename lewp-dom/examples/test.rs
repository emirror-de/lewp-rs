use lewp_dom::api::{a, text};
fn main() {
    let _ = simplelog::SimpleLogger::init(
        simplelog::LevelFilter::Trace,
        simplelog::Config::default(),
    );
    let anchor = a(vec![a(vec![text("Hi there")])]);
    println!("{:#?}", anchor);
}
//fn main() {
//    let _ = simplelog::SimpleLogger::init(
//        simplelog::LevelFilter::Trace,
//        simplelog::Config::default(),
//    );
//    use lewp_dom::{
//        a,
//        audio,
//        document,
//        text,
//        DocumentExt,
//        LanguageTag,
//        NodeExt,
//    };
//    let d = document(
//        LanguageTag::parse("en-US").unwrap(),
//        vec![a(vec![a(vec![]), text("A simple text test.")])],
//    );
//    println!("{}", d.into_html());
//}

//
// html("de-DE", vec![
//    head(vec![
//        script("/something")
//    ]),
//    body(
//        vec![
//        h1("Hello World"),
//        p("This is a paragraph!")
//            .with_attributes(vec![Attribute::new("class", "italic"])
//        ]
//    )
// ])
//
//
//
//
//
//
//
