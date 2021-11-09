use std::{collections::HashMap, marker::PhantomData};

use super::structure::{
    BuiltinValue, Definition, Environment, Item, ItemId, Token, VarType, Variable,
};

impl<'x> Environment<'x> {
    pub fn get_definition(&self, of: ItemId<'x>) -> &Definition<'x> {
        self.items[of].definition.as_ref().unwrap()
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

    pub(super) fn push_def(&mut self, def: Definition<'x>) -> ItemId<'x> {
        let item = Item {
            cached_reduction: None,
            definition: Some(def),
            dependencies: None,
            original_definition: &Token::Plain("Internal"),
            scope: HashMap::new(),
            shown_from: Vec::new(),
        };
        self.items.push(item)
    }

    pub(super) fn push_token(&mut self, token: Token<'x>) -> ItemId<'x> {
        if let Token::Item(item) = token {
            item
        } else {
            self.push_def(Definition::Resolvable(token))
        }
    }

    pub(super) fn push_var(&mut self, typee: VarType<'x>) -> ItemId<'x> {
        let var = self.vars.push(Variable { pd: PhantomData });
        let def = Definition::Variable { var, typee };
        self.push_def(def)
    }

    pub(super) fn item_with_new_definition(
        &mut self,
        original: ItemId<'x>,
        new_def: Definition<'x>,
        is_fundamentally_different: bool,
    ) -> ItemId<'x> {
        if &new_def == self.get_definition(original) {
            return original;
        }
        let mut new_item = self.items[original].clone();
        new_item.definition = Some(new_def);
        if is_fundamentally_different {
            new_item.dependencies = None;
            new_item.cached_reduction = None;
        }
        new_item.shown_from = vec![];
        self.items.get_or_push(new_item)
    }
}
