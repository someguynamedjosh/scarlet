use super::structure::{Definition, Environment, ItemId};

impl<'x> Environment<'x> {
    pub fn get_definition(&self, of: ItemId<'x>) -> &Definition<'x> {
        self.items[of].definition.as_ref().unwrap()
    }
}
