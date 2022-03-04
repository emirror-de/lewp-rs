use lewp_html::{api::*, DocumentExt, LanguageTag, NodeExt, Script};

fn main() {
    let valid_html = document(
        LanguageTag::parse("en-US").unwrap(),
        head(vec![
            link("text/css", "/static/css/main.css"),
        ]),
        body(vec![
            h1(vec![text("Hello World!")]),
            p(vec![text("This is the first paragraph created with lewp-html!")])
                .attrs(vec![("class", "prelude"), ("id", "first-paragraph")]),
            h2(vec![text("The elegant way to create HTML!")]),
            p(vec![text("Creating HTML has never been easier. This paragraph is highlighted!")])
                .attr("class", "highlighted"),
        ]),
    )
    .into_html();
    println!("{}", valid_html);
}
