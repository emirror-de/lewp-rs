/// Tag names that belong to interactive content, see [https://developer.mozilla.org/en-US/docs/Web/Guide/HTML/Content_categories#interactive_content]
pub fn interactive_content() -> Vec<&'static str> {
    vec![
        "a", "button", "details", "embed", "iframe", "keygen", "label",
        "select", "textarea",
    ]
}
