use {
    crate::{DomError, Node, NodeExt},
    html5ever::{namespace_url, ns, LocalName, QualName},
    rcdom::NodeData,
};

mod lists;

/// Checks recursively if the child is valid inside parent.
pub fn validate(parent: &Node, child: &Node) -> Result<(), DomError> {
    let tag_name = match extract_tag_name(&parent)? {
        Some(t) => t,
        None => return Ok(()),
    };
    match &tag_name[..] {
        "a" => a(&child),
        _ => {
            return Err(DomError::Unsupported(
                format!("Unsupported parent node given!"),
                parent.clone(),
            ))
        }
    };
    Ok(())
}

/// Extracts the given tag name from the [Node] structure.
fn extract_tag_name(node: &Node) -> Result<Option<String>, DomError> {
    let qual = match &node.data {
        NodeData::Element { name, .. } => name,
        _ => return Ok(None), // if the node is not an element, there is no check to do
    };
    match qual {
        QualName { local, .. } => Ok(Some(local.to_string())),
        _ => Err(DomError::Unsupported(
            format!("Unknown error during the extraction of the tag name!"),
            node.clone(),
        )),
    }
}

/// Returns true if the tag name is allowed in the given parent name.
///
/// Throws an error to the log if tag_name is present in the not_allowed list.
/// parent_tag_name is only used for the log message.
fn is_tagname_allowed(
    parent_tag_name: &str,
    tag_name: &str,
    not_allowed: Vec<&str>,
) -> bool {
    if not_allowed.iter().any(|e| e == &tag_name) {
        log_error(parent_tag_name, tag_name);
        false
    } else {
        true
    }
}

/// Logs an error to the console.
fn log_error(parent_tag_name: &str, tag_name: &str) {
    log::error!(
        "Node of type \"{}\" contains a non permitted node of type \"{}\"!",
        parent_tag_name,
        tag_name
    );
}

fn a(child: &Node) -> Result<(), DomError> {
    let tag_name = match extract_tag_name(child)? {
        Some(t) => t,
        None => return Ok(()),
    };
    let not_allowed = lists::interactive_content();
    is_tagname_allowed("a", &tag_name, not_allowed);

    let not_allowed = match &tag_name[..] {
        "audio" => child.find_attribute("controls").is_some(),
        "img" => child.find_attribute("usemap").is_some(),
        "input" => child.attribute_eq("type", "hidden"),
        "menu" => child.attribute_eq("type", "toolbar"),
        "object" => child.find_attribute("usemap").is_some(),
        "video" => child.find_attribute("controls").is_some(),
        _ => return Ok(()),
    };
    if not_allowed {
        log_error("a", &tag_name);
    }
    Ok(())
}
