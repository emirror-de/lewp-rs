use {
    crate::{BrowsingContext, Charset, Document, Node, NodeExt, Nodes, Script},
    html5ever::{namespace_url, ns, tendril::Tendril, LocalName, QualName},
    langtag::LanguageTag,
    rcdom::NodeData,
    std::{cell::RefCell, path::Path},
};

/// Defines an element that only takes children as input variable.
macro_rules! api_children_only {
    (
        $(
        #[$link:meta]
        $(#[$outer:meta])*
        $name:ident, $tag_name:expr
        )*
    ) => {
        $(
            /// The
            #[$link]
            /// element.
            $(#[$outer])*
            pub fn $name(children: Nodes) -> Node {
                new_element($tag_name, children)
            }
        )*
    };
}

api_children_only! {
    /// [head](https://html.spec.whatwg.org/dev/semantics.html#the-head-element)
    head, "head"
    /// [body](https://html.spec.whatwg.org/dev/sections.html#the-body-element)
    body, "body"
    /// [article](https://html.spec.whatwg.org/dev/sections.html#the-article-element)
    article, "article"
    /// [section](https://html.spec.whatwg.org/dev/sections.html#the-section-element)
    section, "section"
    /// [nav](https://html.spec.whatwg.org/dev/sections.html#the-nav-element)
    nav, "nav"
    /// [aside](https://html.spec.whatwg.org/dev/sections.html#the-aside-element)
    aside, "aside"
    /// [h1](https://html.spec.whatwg.org/dev/sections.html#the-h1,-h2,-h3,-h4,-h5,-and-h6-elements)
    h1, "h1"
    /// [h2](https://html.spec.whatwg.org/dev/sections.html#the-h1,-h2,-h3,-h4,-h5,-and-h6-elements)
    h2, "h2"
    /// [h3](https://html.spec.whatwg.org/dev/sections.html#the-h1,-h2,-h3,-h4,-h5,-and-h6-elements)
    h3, "h3"
    /// [h4](https://html.spec.whatwg.org/dev/sections.html#the-h1,-h2,-h3,-h4,-h5,-and-h6-elements)
    h4, "h4"
    /// [h5](https://html.spec.whatwg.org/dev/sections.html#the-h1,-h2,-h3,-h4,-h5,-and-h6-elements)
    h5, "h5"
    /// [h6](https://html.spec.whatwg.org/dev/sections.html#the-h1,-h2,-h3,-h4,-h5,-and-h6-elements)
    h6, "h6"
    /// [hgroup](https://html.spec.whatwg.org/dev/sections.html#the-hgroup-element)
    hgroup, "hgroup"
    /// [header](https://html.spec.whatwg.org/dev/sections.html#the-header-element)
    header, "header"
    /// [footer](https://html.spec.whatwg.org/dev/sections.html#the-footer-element)
    footer, "footer"
    /// [address](https://html.spec.whatwg.org/dev/sections.html#the-address-element)
    address, "address"
    /// [p](https://html.spec.whatwg.org/dev/grouping-content.html#the-p-element)
    p, "p"
    /// [hr](https://html.spec.whatwg.org/dev/grouping-content.html#the-hr-element)
    hr, "hr"
    /// [pre](https://html.spec.whatwg.org/dev/grouping-content.html#the-pre-element)
    pre, "pre"
    /// [blockquote](https://html.spec.whatwg.org/dev/grouping-content.html#the-blockquote-element)
    blockquote, "blockquote"
    /// [ol](https://html.spec.whatwg.org/dev/grouping-content.html#the-ol-element)
    ol, "ol"
    /// [ul](https://html.spec.whatwg.org/dev/grouping-content.html#the-ul-element)
    ul, "ul"
    /// [menu](https://html.spec.whatwg.org/dev/grouping-content.html#the-menu-element)
    menu, "menu"
    /// [li](https://html.spec.whatwg.org/dev/grouping-content.html#the-li-element)
    li, "li"
    /// [dl](https://html.spec.whatwg.org/dev/grouping-content.html#the-dl-element)
    dl, "dl"
    /// [dt](https://html.spec.whatwg.org/dev/grouping-content.html#the-dt-element)
    dt, "dt"
    /// [dd](https://html.spec.whatwg.org/dev/grouping-content.html#the-dd-element)
    dd, "dd"
    /// [figure](https://html.spec.whatwg.org/dev/grouping-content.html#the-figure-element)
    figure, "figure"
    /// [figcaption](https://html.spec.whatwg.org/dev/grouping-content.html#the-figcaption-element)
    figcaption, "figcaption"
    /// [main](https://html.spec.whatwg.org/dev/grouping-content.html#the-main-element)
    main, "main"
    /// [div](https://html.spec.whatwg.org/dev/grouping-content.html#the-div-element)
    div, "div"
    /// [em](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-em-element)
    em, "em"
    /// [strong](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-strong-element)
    strong, "strong"
    /// [small](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-small-element)
    small, "small"
    /// [s](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-s-element)
    s, "s"
    /// [cite](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-cite-element)
    cite, "cite"
    /// [q](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-q-element)
    q, "q"
    /// [dfn](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-dfn-element)
    dfn, "dfn"
    /// [abbr](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-abbr-element)
    abbr, "abbr"
    /// [ruby](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-ruby-element)
    ruby, "ruby"
    /// [rt](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-rt-element)
    rt, "rt"
    /// [rp](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-rp-element)
    rp, "rp"
    /// [time](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-time-element)
    time, "time"
    /// [code](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-code-element)
    code, "code"
    /// [var](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-var-element)
    var, "var"
    /// [samp](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-samp-element)
    samp, "samp"
    /// [kbd](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-kbd-element)
    kbd, "kbd"
    /// [sub](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-sub-and-sup-elements)
    sub, "sub"
    /// [sup](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-sub-and-sup-elements)
    sup, "sup"
    /// [i](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-i-element)
    i, "i"
    /// [b](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-b-element)
    b, "b"
    /// [u](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-u-element)
    u, "u"
    /// [mark](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-mark-element)
    mark, "mark"
    /// [bdi](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-bdi-element)
    bdi, "bdi"
    /// [bdo](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-bdo-element)
    bdo, "bdo"
    /// [span](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-span-element)
    span, "span"
    /// [wbr](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-wbr-element)
    wbr, "wbr"
    /// [ins](https://html.spec.whatwg.org/dev/edits.html#the-ins-element)
    ins, "ins"
    /// [del](https://html.spec.whatwg.org/dev/edits.html#the-del-element)
    del, "del"
    /// [picture](https://html.spec.whatwg.org/dev/embedded-content.html#the-picture-element)
    picture, "picture"
    /// [source](https://html.spec.whatwg.org/dev/embedded-content.html#the-source-element)
    source, "source"
    /// [iframe](https://html.spec.whatwg.org/dev/iframe-embed-object.html#the-iframe-element)
    iframe, "iframe"
    /// [embed](https://html.spec.whatwg.org/dev/iframe-embed-object.html#the-embed-element)
    embed, "embed"
    /// [object](https://html.spec.whatwg.org/dev/iframe-embed-object.html#the-object-element)
    object, "object"
    /// [video](https://html.spec.whatwg.org/dev/media.html#the-video-element)
    video, "video"
    /// [audio](https://html.spec.whatwg.org/dev/media.html#the-audio-element)
    audio, "audio"
    /// [track](https://html.spec.whatwg.org/dev/media.html#the-track-element)
    track, "track"
    /// [area](https://html.spec.whatwg.org/dev/image-maps.html#the-area-element)
    area, "area"
    /// [math](https://html.spec.whatwg.org/dev/embedded-content-other.html#mathml)
    math, "math"
    /// [svg](https://html.spec.whatwg.org/dev/embedded-content-other.html#svg-0)
    svg, "svg"
    /// [table](https://html.spec.whatwg.org/dev/tables.html#the-table-element)
    table, "table"
    /// [caption](https://html.spec.whatwg.org/dev/tables.html#the-caption-element)
    caption, "caption"
    /// [colgroup](https://html.spec.whatwg.org/dev/tables.html#the-colgroup-element)
    colgroup, "colgroup"
    /// [col](https://html.spec.whatwg.org/dev/tables.html#the-col-element)
    col, "col"
    /// [tbody](https://html.spec.whatwg.org/dev/tables.html#the-tbody-element)
    tbody, "tbody"
    /// [thead](https://html.spec.whatwg.org/dev/tables.html#the-thead-element)
    thead, "thead"
    /// [tfoot](https://html.spec.whatwg.org/dev/tables.html#the-tfoot-element)
    tfoot, "tfoot"
    /// [tr](https://html.spec.whatwg.org/dev/tables.html#the-tr-element)
    tr, "tr"
    /// [td](https://html.spec.whatwg.org/dev/tables.html#the-td-element)
    td, "td"
    /// [th](https://html.spec.whatwg.org/dev/tables.html#the-th-element)
    th, "th"
    /// [form](https://html.spec.whatwg.org/dev/forms.html#the-form-element)
    form, "form"
    /// [label](https://html.spec.whatwg.org/dev/forms.html#the-label-element)
    label, "label"
    /// [input](https://html.spec.whatwg.org/dev/input.html#the-input-element)
    input, "input"
    /// [button](https://html.spec.whatwg.org/dev/form-elements.html#the-button-element)
    button, "button"
    /// [select](https://html.spec.whatwg.org/dev/form-elements.html#the-select-element)
    select, "select"
    /// [datalist](https://html.spec.whatwg.org/dev/form-elements.html#the-datalist-element)
    datalist, "datalist"
    /// [optgroup](https://html.spec.whatwg.org/dev/form-elements.html#the-optgroup-element)
    optgroup, "optgroup"
    /// [option](https://html.spec.whatwg.org/dev/form-elements.html#the-option-element)
    option, "option"
    /// [textarea](https://html.spec.whatwg.org/dev/form-elements.html#the-textarea-element)
    textarea, "textarea"
    /// [output](https://html.spec.whatwg.org/dev/form-elements.html#the-output-element)
    output, "output"
    /// [progress](https://html.spec.whatwg.org/dev/form-elements.html#the-progress-element)
    progress, "progress"
    /// [meter](https://html.spec.whatwg.org/dev/form-elements.html#the-meter-element)
    meter, "meter"
    /// [fieldset](https://html.spec.whatwg.org/dev/form-elements.html#the-fieldset-element)
    fieldset, "fieldset"
    /// [legend](https://html.spec.whatwg.org/dev/form-elements.html#the-legend-element)
    legend, "legend"
    /// [details](https://html.spec.whatwg.org/dev/interactive-elements.html#the-details-element)
    details, "details"
    /// [summary](https://html.spec.whatwg.org/dev/interactive-elements.html#the-summary-element)
    summary, "summary"
    /// [dialog](https://html.spec.whatwg.org/dev/interactive-elements.html#the-dialog-element)
    dialog, "dialog"
    /// [noscript](https://html.spec.whatwg.org/dev/scripting.html#the-noscript-element)
    noscript, "noscript"
    /// [template](https://html.spec.whatwg.org/dev/scripting.html#the-template-element)
    template, "template"
    /// [slot](https://html.spec.whatwg.org/dev/scripting.html#the-slot-element)
    slot, "slot"
    /// [canvas](https://html.spec.whatwg.org/dev/canvas.html#the-canvas-element)
    canvas, "canvas"
}

/// Helper function to create a tag node.
fn new_element(tag_name: &str, children: Nodes) -> Node {
    let node = rcdom::Node::new(rcdom::NodeData::Element {
        name: QualName::new(None, ns!(html), LocalName::from(tag_name)),
        attrs: RefCell::new(vec![]),
        template_contents: None,
        mathml_annotation_xml_integration_point: false,
    });
    node.append_children(children);
    node
}

/// Creates a new document with the given `<html>` node.
pub fn document(language: LanguageTag, head: Node, body: Node) -> Document {
    let dom = Document::default();
    let doctype = rcdom::Node::new(NodeData::Doctype {
        name: Tendril::from("html"),
        public_id: Tendril::from(""),
        system_id: Tendril::from(""),
    });
    dom.document.children.borrow_mut().push(doctype);
    dom.document
        .children
        .borrow_mut()
        .push(html(language, head, body));
    dom
}

/// Creates a [html](https://html.spec.whatwg.org/dev/semantics.html#the-html-element) element.
pub fn html(language: LanguageTag, head: Node, body: Node) -> Node {
    let language = match language.language() {
        Some(v) => v.primary().as_str().to_owned(),
        None => {
            crate::error!("Could not find language for creating <html> tag!");
            String::new()
        }
    };
    new_element("html", vec![head, body]).attr("lang", &language)
}

/// Creates a [title](https://html.spec.whatwg.org/dev/semantics.html#the-title-element) element.
pub fn title(title: &str) -> Node {
    new_element("title", vec![text(title)])
}

/// Creates a [base](https://html.spec.whatwg.org/dev/semantics.html#the-base-element) element.
pub fn base(href: &str, target: BrowsingContext) -> Node {
    let mut e = new_element("base", vec![]);
    if href != "" {
        e = e.attr("href", href);
    }
    match target {
        BrowsingContext::Empty => e,
        _ => e.attr("target", &format!("{}", target)),
    }
}

/// Creates a [link](https://html.spec.whatwg.org/dev/semantics.html#the-link-element) element.
pub fn link(type_: &str, href: &str) -> Node {
    let mut e = new_element("link", vec![]);
    if href != "" {
        e = e.attr("href", href);
    }
    if type_ != "" {
        e = e.attr("type", type_);
    }
    e
}

/// Creates a [meta](https://html.spec.whatwg.org/dev/semantics.html#the-meta-element) element
/// without attributes.
pub fn meta() -> Node {
    new_element("meta", vec![])
}

/// Creates a [br](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-br-element) element.
pub fn br() -> Node {
    new_element("br", vec![])
}

/// Creates a [style](https://html.spec.whatwg.org/dev/semantics.html#the-style-element) element.
pub fn style(text: Node) -> Node {
    new_element("style", vec![text])
}

/// Creates an [anchor](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-a-element) element.
pub fn a(href: &str, children: Nodes) -> Node {
    let mut e = new_element("a", children);
    if href != "" {
        e = e.attr("href", href);
    }
    e
}

/// Creates an [data](https://html.spec.whatwg.org/dev/text-level-semantics.html#the-data-element) element.
pub fn data(value: &str, children: Nodes) -> Node {
    new_element("a", children).attr("value", value)
}

/// Creates an [img](https://html.spec.whatwg.org/dev/embedded-content.html#the-img-element) element.
pub fn img(src: &Path, title: &str, alt: &str) -> Node {
    new_element("img", vec![]).attrs(vec![
        ("src", &format!("{}", src.display())),
        ("alt", alt),
        ("title", title),
    ])
}

/// Creates an [param](https://html.spec.whatwg.org/dev/iframe-embed-object.html#the-param-element) element.
pub fn param(name: &str, value: &str) -> Node {
    new_element("param", vec![]).attrs(vec![("name", name), ("value", value)])
}

/// Creates an [map](https://html.spec.whatwg.org/dev/image-maps.html#the-map-element) element.
pub fn map(name: &str, children: Nodes) -> Node {
    new_element("map", children).attr("name", name)
}
/// Creates an [script](https://html.spec.whatwg.org/dev/scripting.html#the-script-element) element.
pub fn script(content: Script) -> Node {
    match content {
        Script::Src(src) => new_element("script", vec![]).attr("src", src),
        Script::Inline(script_text) => {
            new_element("script", vec![text(script_text)])
        }
    }
}

/// Creates an element with the given custom name.
pub fn custom(name: &str, children: Nodes) -> Node {
    new_element(name, children)
}

/// Creates a text node.
pub fn text(text: &str) -> Node {
    rcdom::Node::new(NodeData::Text {
        contents: RefCell::new(Tendril::from(text)),
    })
}

/// Creates a `<meta>` charset tag node.
pub fn charset(charset: &Charset) -> Node {
    meta().attr("charset", &charset.to_string())
}

/// Creates a `<meta>` tag node with description.
pub fn description(description: &str) -> Node {
    meta().attrs(vec![("name", "description"), ("content", description)])
}

/// Creates a `<meta>` viewport tag node.
pub fn viewport() -> Node {
    meta().attrs(vec![
        ("name", "viewport"),
        (
            "content",
            "width=device-width, initial-scale=1.0, user-scalable=no",
        ),
    ])
}
