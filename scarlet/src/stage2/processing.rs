use super::structure::{Environment, ItemId, ScopeId, Value};
use crate::stage2::structure::{BuiltinValue, Definitions, VariableId, VariableReplacements};

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

    fn resolve_ident(&mut self, name: &str, defined_in: Option<ScopeId>) -> Option<ItemId> {
        if let Some(defined_in) = defined_in {
            let scope_def = self[defined_in].definition;
            if let Some(Value::Defining { definitions, .. }) = &self[scope_def].value {
                for (candidate_name, val) in definitions {
                    if candidate_name == name {
                        return Some(*val);
                    }
                }
                self.resolve_ident(name, self[scope_def].defined_in)
            } else {
                unreachable!("ICE: scope is not defined as Defining")
            }
        } else {
            None
        }
    }

    fn resolve_member(&mut self, base: ItemId, name: &str) -> Option<ItemId> {
        match self[base].value.as_ref().expect("ICE: undefined item") {
            Value::Defining {
                base, definitions, ..
            } => {
                let base = *base;
                let mut here = None;
                // Do this first so we don't have to clone it before mutably borrowing self.
                for (candidate_name, value) in definitions {
                    if candidate_name == name {
                        here = Some(*value)
                    }
                }
                if let Some(resolved) = self.resolve_member(base, name) {
                    Some(resolved)
                } else {
                    here
                }
            }
            Value::ReplacingItems { .. } => todo!(),
            Value::ReplacingVariables { .. } => todo!(),
            _ => {
                let reduced = self.reduce(base);
                if reduced == base {
                    None
                } else {
                    self.resolve_member(reduced, name)
                }
            }
        }
    }

    // fn get_base_value(&mut self, of: ItemId) -> ItemId {
    //     match self[of].value.as_ref().expect("ICE: undefined item") {
    //         Value::Defining { base, .. } => self.get_base_value(of),
    //         Value::
    //         _ => of,
    //     }
    // }

    /// Called when an item doesn't have its type annotated.
    fn infer_type(&mut self, item_id: ItemId) -> Option<ItemId> {
        let defined_in = self[item_id].defined_in;
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
            Value::Identifier { name } => {
                let name = name.clone();
                self.resolve_ident(&name, defined_in)
                    .map(|i| self.type_of(i))
                    .flatten()
            }
            Value::Item { item } => todo!(),
            Value::Member { base, name } => {
                let (base, name) = (*base, name.clone());
                self.resolve_member(base, &name)
                    .map(|i| self.type_of(i))
                    .flatten()
            }
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
        if let Some(cached_replacement) = self[item_id].cached_reduction {
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
            self[item_id].cached_reduction = Some(result);
        }
        result
    }

    fn reduce_impl(&mut self, item_id: ItemId, typee: ItemId) -> ItemId {
        let defined_in = self[item_id].defined_in;
        match self[item_id].value.as_ref().expect("ICE: Undefined value") {
            Value::Any { variable } => todo!(),
            Value::BuiltinOperation { operation } => todo!(),
            Value::BuiltinValue { .. } => item_id,
            Value::Defining {
                base,
                definitions,
                this_scope,
            } => {
                let (base, old_definitions, this_scope) = (*base, definitions.clone(), *this_scope);
                let base = self.reduce(base);
                let mut definitions = Definitions::new();
                for (name, value) in old_definitions {
                    definitions.insert_no_replace(name, self.reduce(value));
                }
                let value = Value::Defining {
                    base,
                    definitions,
                    this_scope,
                };
                self.get_or_insert_value(value, defined_in)
            }
            Value::FromItems { base, items } => todo!(),
            Value::FromVariables { base, variables } => todo!(),
            Value::Identifier { name } => {
                let name = name.clone();
                let resolved = self
                    .resolve_ident(&name, defined_in)
                    .expect("TODO: nice error, bad ident");
                self.reduce(resolved)
            }
            Value::Item { item } => todo!(),
            Value::Member { base, name } => {
                let (base, name) = (*base, name.clone());
                let resolved = self
                    .resolve_member(base, &name)
                    .expect("TODO: nice error, bad member");
                self.reduce(resolved)
            }
            Value::ReplacingItems { base, replacements } => todo!(),
            Value::ReplacingVariables { base, replacements } => todo!(),
            Value::Variant { variant } => todo!(),
        }
    }
}
