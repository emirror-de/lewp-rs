use {
    super::super::{ContentCategory, ContentModel, TypedNode},
    crate::NodeExt,
};

typed_node! {
    /// The anchor element.
    Anchor
    "a"
}

impl TypedNode for Anchor {
    fn categories(&self) -> Vec<ContentCategory> {
        use ContentCategory::*;
        let mut v = vec![Flow, Phrasing, Palpable];
        if self.find_attribute("href").is_some() {
            v.push(Interactive);
        }
        v
    }

    fn content_model(&self, node: &Self) -> Vec<ContentModel> {
        vec![
            ContentModel::Specific(
                !node.categories().contains(&ContentCategory::Interactive),
            ),
            ContentModel::Specific(node.tag_name() != self.tag_name()),
            ContentModel::Specific(node.find_attribute("tabindex").is_none()),
            ContentModel::Category(ContentCategory::Transparent),
        ]
        //if node.categories().contains(&ContentCategory::Interactive)
        //    || node.tag_name() == self.tag_name()
        //    || node.find_attribute("tabindex").is_some()
        //{
        //    return false;
        //}
        //true
        //vec![]
    }
}
