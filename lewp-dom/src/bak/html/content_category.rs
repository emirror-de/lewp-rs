/// Representation of the categories defined in content models section of HTML5 spec.
///
/// See <https://html.spec.whatwg.org/multipage/dom.html#content-models>
#[derive(PartialEq)]
pub enum ContentCategory {
    /// Sets up presentation or behavior of content.
    Metadata,
    /// Most elements that are used in the body of documents and applications are categorized as flow content.
    Flow,
    /// Sectioning content is content that defines the scope of headings and footers.
    Sectioning,
    /// Heading content defines the header of a section
    /// (whether explicitly marked up using sectioning content elements,
    /// or implied by the heading content itself).
    Heading,
    /// Phrasing content is the text of the document, as well as elements that
    /// mark up that text at the intra-paragraph level. Runs of phrasing content form paragraphs.
    Phrasing,
    /// Embedded content is content that imports another resource into the
    /// document, or content from another vocabulary that is inserted into the document.
    Embedded,
    /// Interactive content is content that is specifically intended for user interaction.
    Interactive,
    /// As a general rule, elements whose content model allows any flow content
    /// or phrasing content should have at least one node in its contents that
    /// is palpable content and that does not have the hidden attribute specified.
    Palpable,
    /// Script-supporting elements are those that do not represent anything
    /// themselves (i.e. they are not rendered), but are used to support scripts,
    /// e.g. to provide functionality for the user.
    ScriptSupporting,
    /// Some elements are described as transparent; they have "transparent"
    /// in the description of their content model. The content model of a
    /// transparent element is derived from the content model of its parent
    /// element: the elements required in the part of the content model that
    /// is "transparent" are the same elements as required in the part of the
    /// content model of the parent of the transparent element in which the transparent element finds itself.
    Transparent,
    /// When an element's content model is nothing, the element must contain no
    /// Text nodes (other than inter-element whitespace) and no element nodes.
    Nothing,
}
