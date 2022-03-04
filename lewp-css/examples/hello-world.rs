use {
    cssparser::ToCss,
    lewp_css::{
        domain::{
            at_rules::{document::DocumentAtRule, media::MediaAtRule},
            CssRule, CssRules, StyleRule,
        },
        Stylesheet,
    },
};

fn recursive_modification(rules: &mut CssRules) {
    for rule in &mut rules.0 {
        match rule {
            CssRule::Style(StyleRule { selectors, .. }) => {
                let mut b = String::new();
                selectors.0[0].to_css(&mut b).unwrap();
                println!("{:#?}", b);
                let new_selector =
                    lewp_css::parse_css_selector(&format!(".hidden {}", b))
                        .unwrap();
                selectors.0 = vec![new_selector];
            }
            CssRule::Media(MediaAtRule { rules, .. })
            | CssRule::Document(DocumentAtRule { rules, .. }) => {
                recursive_modification(rules)
            }
            _ => (),
        }
        println!("{:#?}", rule);
    }
}

fn main() {
    let some_css =
        std::fs::read_to_string("testfiles/hello-world.css").unwrap();
    let mut stylesheet = Stylesheet::parse(&some_css).expect("CSS was invalid");

    // Alternatively, load from a file using Stylesheet::from_file_path("/path/to/stylesheet.css").unwrap();

    let mut destination = String::new();

    // Don't write source-map and source-url comments if any are present in the stylesheet
    let include_source_urls = false;

    // Example function that shows how to modify a parsed stylesheet
    recursive_modification(&mut stylesheet.rules);

    stylesheet
        .to_css(&mut destination, include_source_urls)
        .expect("Failed to write to destination");

    assert_eq!(
        &destination,
        ".hidden #something{display: flex;margin-top: 10px;font-face: Arial}"
    );

    // To serialize to a Vec<u8> of bytes instead
    let _bytes = stylesheet.to_bytes(include_source_urls);

    // To serialize to a file instead
    stylesheet
        .to_file_path("./stylesheet-hello-world.css", include_source_urls)
        .unwrap();
}
