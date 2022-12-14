use std::collections::HashMap;

use crate::{diagnostic::Diagnostic, item::ItemRef};

pub struct Environment<Definition, Analysis> {
    language_items: HashMap<String, ItemRef<Definition, Analysis>>,
}

impl<Definition, Analysis> Environment<Definition, Analysis> {
    pub fn define_language_item(
        &self,
        name: &str,
        value: ItemRef<Definition, Analysis>,
    ) -> Result<(), Diagnostic> {
        todo!()
    }

    pub fn get_language_item(
        &self,
        name: &str,
    ) -> Result<ItemRef<Definition, Analysis>, Diagnostic> {
        todo!()
    }
}
