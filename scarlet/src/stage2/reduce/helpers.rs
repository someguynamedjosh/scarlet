use crate::stage2::structure::{After, BuiltinValue, Definition, Environment, ItemId, VariableId};

impl<'x> Environment<'x> {
    pub fn definition_of(&self, item: ItemId<'x>) -> &Definition<'x> {
        self.items[item].definition.as_ref().unwrap()
    }

    pub(super) fn args_as_builtin_values(
        &mut self,
        args: &[ItemId<'x>],
    ) -> Option<Vec<BuiltinValue>> {
        let mut result = Vec::new();
        for arg in args {
            let arg = self.reduce(*arg);
            if let Definition::BuiltinValue(value) = self.items[arg].definition.as_ref().unwrap() {
                result.push(*value);
            } else {
                return None;
            }
        }
        Some(result)
    }

    pub(super) fn item_with_new_definition(
        &mut self,
        original: ItemId<'x>,
        new_def: Definition<'x>,
        is_fundamentally_different: bool,
    ) -> ItemId<'x> {
        let mut new_item = self.items[original].clone();
        new_item.definition = Some(new_def);
        if is_fundamentally_different {
            new_item.after = None;
            new_item.dependencies = None;
            new_item.cached_reduction = None;
        }
        self.items.get_or_push(new_item)
    }

    pub fn item_as_variable(&self, item: ItemId<'x>) -> VariableId<'x> {
        match self.items[item].definition.as_ref().unwrap() {
            Definition::Member(_, _) => todo!(),
            Definition::Other(id) => self.item_as_variable(*id),
            Definition::Variable(id) => *id,
            _ => todo!("Nice error, {:?} is not a variable", item),
        }
    }
}
