use std::collections::HashMap;

use crate::{diagnostic::Diagnostic, item::ItemPtr};

const LANGUAGE_ITEM_NAMES: &[&str] = &["Type0", "Type1"];

pub struct Environment {
    language_items: HashMap<&'static str, ItemPtr>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            language_items: HashMap::new()
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
}
