use super::structure::{Environment, ItemId, Value};
use crate::stage2::structure::Item;

impl Environment {
    pub fn reduce_everything(&mut self) {
        let mut id = self.items.first().unwrap();
        loop {
            self.reduce(id);
            if let Some(new_id) = self.items.next(id) {
                id = new_id;
            } else {
                break;
            }
        }
    }

    fn type_of(&mut self, item_id: ItemId) -> Option<ItemId> {
        if let Some(typee) = self[item_id].typee {
            Some(typee)
        } else {
            let inferred = self.infer_type(item_id);
            self[item_id].typee = inferred;
            inferred
        }
    }

    /// Called when an item doesn't have its type annotated.
    fn infer_type(&mut self, item_id: ItemId) -> Option<ItemId> {
        match self[item_id].value.as_ref().expect("ICE: Undefined value") {
            Value::Any { variable } => todo!(),
            Value::BuiltinOperation { operation } => todo!(),
            Value::BuiltinValue { value } => todo!(),
            Value::Defining { base, .. } => {
                let base = *base;
                self.type_of(base)
            }
            Value::FromItems { base, items } => todo!(),
            Value::FromVariables { base, variables } => todo!(),
            Value::Identifier { name } => todo!(),
            Value::Item { item } => todo!(),
            Value::Member { base, name } => todo!(),
            Value::ReplacingItems { base, replacements } => {
                let (base, replacements) = (*base, replacements.clone());
                let base_type = self.type_of(base);
                todo!()
            }
            Value::ReplacingVariables { base, replacements } => {
                todo!()
            }
            Value::Variant { variant } => todo!(),
        }
    }

    fn reduce(&mut self, item_id: ItemId) -> ItemId {
        if let Some(cached_replacement) = self[item_id].cached_replacement {
            return cached_replacement;
        }
        let typee = if let Some(typee) = self[item_id].typee {
            typee
        } else if let Some(typee) = self.infer_type(item_id) {
            self[item_id].typee = Some(typee);
            typee
        } else {
            return item_id;
        };
        let result = self.reduce_impl(item_id, typee);
        if result != item_id {
            self[item_id].cached_replacement = Some(result);
        }
        result
    }

    fn reduce_impl(&mut self, item_id: ItemId, typee: ItemId) -> ItemId {
        match self[item_id].value.as_ref().expect("ICE: Undefined value") {
            Value::Any { variable } => todo!(),
            Value::BuiltinOperation { operation } => todo!(),
            Value::BuiltinValue { value } => todo!(),
            Value::Defining {
                base,
                definitions,
                this_scope,
            } => {
                let mut new_base = self[*base].clone();
                new_base.member_scopes.push(*this_scope);
                todo!();
            }
            Value::FromItems { base, items } => todo!(),
            Value::FromVariables { base, variables } => todo!(),
            Value::Identifier { name } => todo!(),
            Value::Item { item } => todo!(),
            Value::Member { base, name } => todo!(),
            Value::ReplacingItems { base, replacements } => todo!(),
            Value::ReplacingVariables { base, replacements } => todo!(),
            Value::Variant { variant } => todo!(),
        }
    }
}
