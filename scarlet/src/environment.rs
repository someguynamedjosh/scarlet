use std::collections::HashMap;

use crate::{
    diagnostic::Diagnostic,
    item::{ItemDefinition, ItemRef},
};

pub struct Environment<Definition, Analysis> {
    language_items: HashMap<String, ItemRef<Definition, Analysis>>,
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis> Environment<Definition, Analysis> {
    pub fn define_language_item(
        &mut self,
        name: &str,
        value: ItemRef<Definition, Analysis>,
    ) -> Result<(), Diagnostic> {
        if self
            .language_items
            .insert(name.to_owned(), value.ptr_clone())
            .is_some()
        {
            Err(Diagnostic::new()
                .with_text_error(format!("Language item \"{}\" defined twice!", name))
                .with_item_error(&value))
        } else {
            Ok(())
        }
    }

    pub fn get_language_item(
        &self,
        name: &str,
    ) -> Result<ItemRef<Definition, Analysis>, Diagnostic> {
        if let Some(item) = self.language_items.get(name) {
            Ok(item.ptr_clone())
        } else {
            Err(Diagnostic::new()
                .with_text_error(format!("Language item \"{}\" is not defined.", name)))
        }
    }

    pub fn new() -> Self {
        Self {
            language_items: HashMap::new(),
        }
    }
}
