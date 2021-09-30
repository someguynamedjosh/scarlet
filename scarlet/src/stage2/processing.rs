use super::structure::{Environment, ItemId, ScopeId, Value};
use crate::stage2::structure::{BuiltinValue, VariableId, VariableReplacements};

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

    fn get_or_insert_value(&mut self, value: Value, defined_in: Option<ScopeId>) -> ItemId {
        let defined_in = defined_in.unwrap_or(self.get_root_scope());
        for (id, item) in &self.items {
            if item.value.as_ref().expect("ICE: Undefined value") == &value {
                return id;
            }
        }
        self.insert_value(defined_in, value)
    }

    fn get_or_insert_origin_type(&mut self) -> ItemId {
        self.get_or_insert_value(
            Value::BuiltinValue {
                value: BuiltinValue::OriginType,
            },
            None,
        )
    }

    fn get_or_insert_u8_type(&mut self) -> ItemId {
        self.get_or_insert_value(
            Value::BuiltinValue {
                value: BuiltinValue::U8Type,
            },
            None,
        )
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

    fn reduce_expecting_variable(&mut self, item_id: ItemId) -> VariableId {
        let reduced = self.reduce(item_id);
        match self[reduced]
            .value
            .as_ref()
            .expect("ICE: Resolved item is undefined")
        {
            Value::Any { variable } => *variable,
            _ => todo!("nice error: {:?} is not a variable", reduced),
        }
    }

    fn replace(&mut self, target: ItemId, reduced_replacements: VariableReplacements) -> ItemId {
        todo!()
    }

    /// Called when an item doesn't have its type annotated.
    fn infer_type(&mut self, item_id: ItemId) -> Option<ItemId> {
        match self[item_id].value.as_ref().expect("ICE: Undefined value") {
            Value::Any { variable } => todo!(),
            Value::BuiltinOperation { operation } => todo!(),
            Value::BuiltinValue { value } => match value {
                BuiltinValue::OriginType | BuiltinValue::U8Type => {
                    Some(self.get_or_insert_origin_type())
                }
                BuiltinValue::U8(..) => Some(self.get_or_insert_u8_type()),
            },
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
                let base_type = self.type_of(base)?;
                let mut reduced_replacements = VariableReplacements::new();
                for (target, value) in replacements.clone() {
                    let target = self.reduce_expecting_variable(target);
                    let value = self.reduce(value);
                    if reduced_replacements.contains_key(&target) {
                        todo!("Nice error, variable assigned twice.");
                    }
                    reduced_replacements.insert_no_replace(target, value);
                }
                Some(self.replace(base_type, reduced_replacements))
            }
            Value::ReplacingVariables { base, replacements } => {
                let (base, replacements) = (*base, replacements.clone());
                let base_type = self.type_of(base)?;
                let mut reduced_replacements = VariableReplacements::new();
                for (target, value) in replacements.clone() {
                    let value = self.reduce(value);
                    reduced_replacements.insert_no_replace(target, value);
                }
                Some(self.replace(base_type, reduced_replacements))
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
