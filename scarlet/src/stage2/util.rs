use std::{collections::HashMap, marker::PhantomData};

use super::structure::{
    BuiltinValue, Definition, Environment, Item, ItemId, Token, VarType, Variable,
};
use crate::stage2::structure::Member;

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

    pub(super) fn begin_item(&mut self) -> ItemId<'x> {
        let item = Item {
            cached_reduction: None,
            definition: None,
            dependencies: None,
            original_definition: &Token::Plain("Internal"),
            parent_scope: None,
            shown_from: Vec::new(),
        };
        self.items.push(item)
    }

    pub(super) fn push_def(&mut self, def: Definition<'x>) -> ItemId<'x> {
        let item = self.begin_item();
        self.items[item].definition = Some(def);
        self.check(item);
        item
    }

    pub(super) fn push_token(&mut self, token: Token<'x>) -> ItemId<'x> {
        if let Token::Item(item) = token {
            item
        } else {
            self.push_def(Definition::Unresolved(token))
        }
    }

    pub(super) fn get_or_push_var(&mut self, typee: VarType<'x>) -> ItemId<'x> {
        for (id, item) in &self.items {
            if let Some(Definition::Variable {
                typee: candidate_typee,
                ..
            }) = &item.definition
            {
                if &typee == candidate_typee {
                    return id;
                }
            }
        }
        self.push_var(typee)
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
        let id = self.items.get_or_push(new_item);
        self.check(id);
        id
    }

    pub(super) fn has_member(&mut self, base: ItemId<'x>, member_name: &str) -> bool {
        self.get_member(base, member_name).is_some()
    }

    pub(super) fn get_member(&mut self, base: ItemId<'x>, member_name: &str) -> Option<ItemId<'x>> {
        let def = Definition::Member(base, Member::Named(member_name.to_owned()));
        match self.get_definition(base).clone() {
            Definition::BuiltinOperation(..) => None,
            Definition::BuiltinValue(..) => None,
            Definition::Match {
                conditions,
                else_value,
                ..
            } => {
                for c in conditions {
                    if !self.has_member(c.value, member_name) {
                        return None;
                    }
                }
                if self.has_member(else_value, member_name) {
                    Some(self.push_def(def))
                } else {
                    None
                }
            }
            Definition::Member(_, _) => todo!(),
            Definition::Unresolved(..) => {
                let base = self.resolve(base);
                self.get_member(base, member_name)
            }
            Definition::SetEager { base, .. } => self.get_member(base, member_name),
            Definition::Struct(fields) => {
                for (index, field) in fields.into_iter().enumerate() {
                    if field.name == Some(member_name) || member_name == &format!("{}", index) {
                        return Some(field.value);
                    }
                }
                None
            }
            Definition::Substitute(_, _) => todo!(),
            Definition::Variable { .. } => {
                todo!()
            }
        }
    }
}
