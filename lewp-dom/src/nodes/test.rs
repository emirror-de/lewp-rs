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
