use std::collections::HashMap;

use crate::{
    definitions::{builtin::DBuiltin, struct_literal::DStructLiteral},
    diagnostic::Diagnostic,
    entry::OnlyConstructedByEntry,
    item::{
        query::{Query, QueryContext, RootQuery, TypeQuery, TypeCheckQuery},
        IntoItemPtr, ItemPtr,
    },
};

const LANGUAGE_ITEM_NAMES: &[&str] = &["Type0", "Type1"];

/// This struct guarantees certain parts of the code remain internal to the
/// library without having to put them in the same module.
pub(crate) struct OnlyConstructedByEnvironment(());

pub struct Environment {
    language_items: HashMap<&'static str, ItemPtr>,
    root: ItemPtr,
}

impl Environment {
    pub(crate) fn new(_: OnlyConstructedByEntry) -> Self {
        Self {
            language_items: HashMap::new(),
            root: DStructLiteral::new_module(vec![]).into_ptr(),
        }
    }

    pub fn define_language_item(
        &mut self,
        name: &str,
        definition: ItemPtr,
    ) -> Result<(), Diagnostic> {
        if self.language_items.contains_key(name) {
            Err(Diagnostic::new().with_text_error(format!(
                "Cannot define language item \"{}\" multiple times.",
                name
            )))
        } else if let Some(name) = LANGUAGE_ITEM_NAMES
            .iter()
            .find(|candidate| **candidate == name)
        {
            self.language_items.insert(name, definition);
            Ok(())
        } else {
            Err(Diagnostic::new().with_text_error(format!("\"{}\" is not a language item.", name)))
        }
    }

    pub(crate) fn set_root(&mut self, root: ItemPtr) {
        self.root = root;
    }

    fn root_query() -> QueryContext<RootQuery> {
        QueryContext::root(OnlyConstructedByEnvironment(()))
    }

    pub fn query_root_type(&self) -> <TypeQuery as Query>::Result {
        self.root.query_type(&mut Self::root_query())
    }

    pub fn query_root_type_check(&self) -> <TypeCheckQuery as Query>::Result {
        self.root.query_type_check(&mut Self::root_query())
    }
}
