use super::structure::Environment;
use crate::{
    shared::{
        IntegerMathOperation, Item, ItemId, PrimitiveOperation, PrimitiveValue, Replacements,
    },
    util::indented,
};

#[derive(Clone)]
enum ChildOf {
    Definition { scope: ItemId, name: String },
    Base(ItemId),
}

impl ChildOf {
    pub fn parent_id(&self) -> ItemId {
        match self {
            Self::Definition { scope, .. } => *scope,
            Self::Base(id) => *id,
        }
    }
}

impl Environment {
    pub fn display_infos(&self) {
        for (id, item) in self.iter() {
            if let Some(scope) = item.info_requested {
                let repr = self.get_item_code_or_name(id, scope);
                println!("{}", repr);
            }
        }
    }

    /// Tries to get code. If that fails, gets a name instead.
    pub fn get_item_code_or_name(&self, item_id: ItemId, in_scope: ItemId) -> String {
        if let Some(code) = self.get_item_code(&self.items[item_id.0].definition, in_scope) {
            return code;
        } else if let Some(name) = self.get_item_name(item_id, in_scope) {
            return name;
        } else {
            return format!("anonymous");
        }
    }

    /// Tries to get a name. If that fails, gets code instead.
    pub fn get_item_name_or_code(&self, item_id: ItemId, in_scope: ItemId) -> String {
        if let Some(name) = self.get_item_name(item_id, in_scope) {
            return name;
        } else if let Some(code) = self.get_item_code(&self.items[item_id.0].definition, in_scope) {
            return code;
        } else {
            return format!("anonymous");
        }
    }

    pub fn get_item_code(&self, item: &Item, in_scope: ItemId) -> Option<String> {
        match item {
            Item::Defining { base, .. } | Item::TypeIs { base, .. } => {
                self.get_item_code(&self.items[base.0].definition, in_scope)
            }
            Item::GodType => Some(format!("TYPE")),
            Item::InductiveValue {
                records,
                typee,
                variant_name,
            } => self.get_inductive_value_code(records, typee, variant_name, in_scope),
            Item::IsSameVariant { base, other } => self.get_is_variant_code(base, other, in_scope),
            Item::Pick {
                elif_clauses,
                else_clause,
                initial_clause,
            } => self.get_pick_code(elif_clauses, else_clause, initial_clause, in_scope),
            Item::PrimitiveOperation(op) => self.get_primitive_operation_code(op, in_scope),
            Item::PrimitiveValue(val) => self.get_primitive_value_code(*val),
            Item::Replacing {
                base, replacements, ..
            } => self.get_replacing_code(base, replacements, in_scope),
            _ => None,
        }
    }

    fn get_item_name(&self, id: ItemId, in_scope: ItemId) -> Option<String> {
        self.get_item_name_impl(id, in_scope, vec![]).ok().flatten()
    }

    fn get_parents(&self, of: ItemId) -> Vec<ChildOf> {
        let mut parents = Vec::new();
        for (id, def) in self.iter() {
            match &def.definition {
                Item::Defining { base, definitions } => {
                    if *base == of {
                        parents.push(ChildOf::Base(id))
                    }
                    for (name, def) in definitions {
                        if *def == of {
                            parents.push(ChildOf::Definition {
                                name: name.clone(),
                                scope: id,
                            });
                        }
                    }
                }
                _ => (),
            }
        }
        parents
    }

    fn a_is_b_or_parent_of_b(&self, a: ItemId, b: ItemId, already_checked: Vec<ItemId>) -> bool {
        if already_checked.contains(&b) {
            // Prevent infinite loops
            false
        } else if a == b {
            // If item is scope, return true.
            true
        } else {
            // Otherwise, if a parent of item or any of their parents matches the scope,
            // return true.
            for b_as_child in self.get_parents(b) {
                let b_parent = b_as_child.parent_id();
                let new_already_checked = [already_checked.clone(), vec![b]].concat();
                if self.a_is_b_or_parent_of_b(a, b_parent, new_already_checked) {
                    return true;
                }
            }
            false
        }
    }

    fn get_item_name_impl(
        &self,
        id: ItemId,
        in_scope: ItemId,
        already_checked: Vec<ItemId>,
    ) -> Result<Option<String>, ()> {
        if already_checked.contains(&id) {
            // Prevent infinite loops.
            Err(())
        } else if self.a_is_b_or_parent_of_b(id, in_scope, vec![]) {
            // If we are trying to name something which is a parent of the scope from which
            // the name should be resolved, that's an item with no name. I.E. any children
            // can be referred to by name without prefixing it with anything extra.
            Ok(None)
        } else {
            let mut candidates = Vec::new();
            for id_as_child in self.get_parents(id) {
                let parent_id = id_as_child.parent_id();
                let new_already_checked = [already_checked.clone(), vec![id]].concat();
                let parent_name = self.get_item_name_impl(parent_id, in_scope, new_already_checked);
                match parent_name {
                    Ok(None) => match id_as_child {
                        ChildOf::Base(..) => return Ok(None),
                        ChildOf::Definition { name, .. } => candidates.push(name),
                    },
                    Ok(Some(parent_name)) => match id_as_child {
                        ChildOf::Base(..) => candidates.push(parent_name),
                        ChildOf::Definition { name, .. } => {
                            candidates.push(format!("{}::{}", parent_name, name))
                        }
                    },
                    Err(..) => (),
                }
            }
            let result = candidates.into_iter().min_by_key(|p| p.len());
            if result.is_none() {
                Err(())
            } else {
                Ok(result)
            }
        }
    }

    fn get_inductive_value_code(
        &self,
        records: &Vec<ItemId>,
        typee: &ItemId,
        variant_name: &String,
        in_scope: ItemId,
    ) -> Option<String> {
        let mut res = format!(
            "{}::{}[",
            self.get_item_name_or_code(*typee, in_scope),
            variant_name
        );
        for value in records {
            let value = indented(&self.get_item_name_or_code(*value, in_scope));
            res.push_str(&format!("\n    {}", value))
        }
        res.push_str("\n]");
        Some(res)
    }

    fn get_is_variant_code(
        &self,
        base: &ItemId,
        other: &ItemId,
        in_scope: ItemId,
    ) -> Option<String> {
        Some(format!(
            "{} is_variant{{{}}}",
            self.get_item_name_or_code(*base, in_scope),
            self.get_item_name_or_code(*other, in_scope)
        ))
    }

    fn get_pick_code(
        &self,
        elif_clauses: &Vec<(ItemId, ItemId)>,
        else_clause: &ItemId,
        initial_clause: &(ItemId, ItemId),
        in_scope: ItemId,
    ) -> Option<String> {
        let mut res = String::from("pick{");

        let condition = indented(&self.get_item_name_or_code(initial_clause.0, in_scope));
        let value = indented(&self.get_item_name_or_code(initial_clause.1, in_scope));
        res.push_str(&format!("\n   if {}, {}", condition, value));

        for (condition, value) in elif_clauses.iter().copied() {
            let condition = indented(&self.get_item_name_or_code(condition, in_scope));
            let value = indented(&self.get_item_name_or_code(value, in_scope));
            res.push_str(&format!("\n   elif {}, {}", condition, value));
        }

        let value = indented(&self.get_item_name_or_code(*else_clause, in_scope));
        res.push_str(&format!("\n   else {}", value));

        res.push_str("\n}");

        Some(res)
    }

    fn get_integer_operation_code(&self, op: &IntegerMathOperation, in_scope: ItemId) -> String {
        use IntegerMathOperation as Imo;
        match op {
            Imo::Sum(a, b) => format!(
                "sum[{} {}]",
                self.get_item_name_or_code(*a, in_scope),
                self.get_item_name_or_code(*b, in_scope)
            ),
            Imo::Difference(a, b) => format!(
                "difference[{} {}]",
                self.get_item_name_or_code(*a, in_scope),
                self.get_item_name_or_code(*b, in_scope)
            ),
        }
    }

    fn get_primitive_operation_code(
        &self,
        op: &PrimitiveOperation,
        in_scope: ItemId,
    ) -> Option<String> {
        match op {
            PrimitiveOperation::I32Math(op) => Some(format!(
                "Integer32::{}",
                self.get_integer_operation_code(op, in_scope)
            )),
        }
    }

    fn get_primitive_value_code(&self, value: PrimitiveValue) -> Option<String> {
        match value {
            PrimitiveValue::Bool(..) => None,
            PrimitiveValue::I32(val) => Some(format!("{}", val)),
        }
    }

    fn get_replacing_code(
        &self,
        base: &ItemId,
        replacements: &Replacements,
        in_scope: ItemId,
    ) -> Option<String> {
        let mut res = format!("{}[", self.get_item_name_or_code(*base, in_scope));
        for (target, value) in replacements {
            let target = self.get_item_name_or_code(*target, in_scope);
            let value = indented(&self.get_item_name_or_code(*value, in_scope));
            res.push_str(&format!("\n    {} is {}", target, value))
        }
        res.push_str("\n]");
        Some(res)
    }

    fn get_variable_code(&self, selff: ItemId, in_scope: ItemId) -> Option<String> {
        self.get_item_name(selff, in_scope)
    }
}
