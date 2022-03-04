// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{NamespacePrefix, NamespaceUrl},
    std::{collections::HashMap, rc::Rc},
};

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Namespaces {
    defaultNamespace: Option<NamespaceUrl>,
    namespaces: HashMap<NamespacePrefix, NamespaceUrl>,
}

impl Namespaces {
    #[inline(always)]
    pub fn default_namespace(&self) -> Option<NamespaceUrl> {
        self.defaultNamespace
            .as_ref().cloned()
    }

    #[inline(always)]
    pub fn namespace_for_prefix(
        &self,
        prefix: &NamespacePrefix,
    ) -> Option<NamespaceUrl> {
        self.namespaces
            .get(prefix).cloned()
    }

    #[inline(always)]
    pub(crate) fn empty() -> Rc<Self> {
        Rc::new(Self::default())
    }

    #[inline(always)]
    pub(crate) fn update(
        &mut self,
        prefix: Option<&NamespacePrefix>,
        url: &NamespaceUrl,
    ) {
        match prefix {
            None => self.defaultNamespace = Some(url.clone()),
            Some(prefix) => {
                self.namespaces.insert(prefix.clone(), url.clone());
            }
        }
    }
}
