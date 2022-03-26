use lewp_css::Stylesheet;

fn main() {
    let some_css = std::fs::read_to_string("testfiles/pico.min.css").unwrap();
    let stylesheet = Stylesheet::parse(&some_css).expect("CSS was invalid");

    // Alternatively, load from a file using Stylesheet::from_file_path("/path/to/stylesheet.css").unwrap();

    let mut destination = String::new();

    // Don't write source-map and source-url comments if any are present in the stylesheet
    let include_source_urls = false;

    stylesheet
        .to_css(&mut destination, include_source_urls)
        .expect("Failed to write to destination");

    // To serialize to a file instead
    stylesheet
        .to_file_path("./stylesheet-hello-world.css", include_source_urls)
        .unwrap();
}
