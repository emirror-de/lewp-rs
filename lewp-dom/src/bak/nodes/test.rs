#[test]
fn a() {
    use crate::{a, document, text, DocumentExt, LanguageTag};
    let d = document(
        LanguageTag::parse("en-US").unwrap(),
        vec![a(vec![text("A simple text test.")])],
    );
    assert_eq!(
        "<!DOCTYPE html><html lang=\"en\"><a href=\"#\">A simple text test.</a></html>",
        d.into_html()
    );
}

#[test]
fn abbr() {
    use crate::{abbr, document, DocumentExt, LanguageTag};
    let d = document(
        LanguageTag::parse("en-US").unwrap(),
        vec![abbr("CSS", "Cascading Stylesheets")],
    );
    assert_eq!(
        "<!DOCTYPE html><html lang=\"en\"><abbr title=\"Cascading Stylesheets\">CSS</abbr></html>",
        d.into_html()
    );
}

#[test]
fn address() {
    use crate::{address, document, text, DocumentExt, LanguageTag};
    let d = document(
        LanguageTag::parse("en-US").unwrap(),
        vec![address(vec![text("My address.")])],
    );
    assert_eq!(
        "<!DOCTYPE html><html lang=\"en\"><address>My address.</address></html>",
        d.into_html()
    );
}

#[test]
fn area() {
    use crate::{area, document, text, DocumentExt, LanguageTag};
    let d = document(
        LanguageTag::parse("en-US").unwrap(),
        vec![area("rect", "250,0,250,0", "alt text", "/map")],
    );
    assert_eq!(
        "<!DOCTYPE html><html lang=\"en\"><area shape=\"rect\" coords=\"250,0,250,0\" alt=\"alt text\" href=\"/map\"></html>",
        d.into_html()
    );
}

#[test]
fn document() {
    use crate::{document, DocumentExt, LanguageTag};
    let d = document(LanguageTag::parse("en-US").unwrap(), vec![]);
    assert_eq!("<!DOCTYPE html><html lang=\"en\"></html>", d.into_html());
}

#[test]
fn div() {
    use crate::{div, document, DocumentExt, LanguageTag};
    let d = document(LanguageTag::parse("en-US").unwrap(), vec![div(vec![])]);
    assert_eq!(
        "<!DOCTYPE html><html lang=\"en\"><div></div></html>",
        d.into_html()
    );
}

#[test]
fn audio() {
    use crate::{audio, document, DocumentExt, LanguageTag};
    let d = document(LanguageTag::parse("en-US").unwrap(), vec![audio(vec![])]);
    assert_eq!(
        "<!DOCTYPE html><html lang=\"en\"><audio></audio></html>",
        d.into_html()
    );
}

#[test]
fn text() {
    use crate::{document, text, DocumentExt, LanguageTag};
    let d = document(
        LanguageTag::parse("en-US").unwrap(),
        vec![text("A simple text test.")],
    );
    assert_eq!(
        "<!DOCTYPE html><html lang=\"en\">A simple text test.</html>",
        d.into_html()
    );
}
