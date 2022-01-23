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
