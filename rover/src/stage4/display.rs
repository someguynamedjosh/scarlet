use super::structure::Environment;
use crate::{
    shared::{
        IntegerMathOperation, Item, ItemId, PrimitiveOperation, PrimitiveValue, Replacements,
    },
    util::indented,
};

impl Environment {
    pub fn display_infos(&self) {
        for (id, item) in self.iter() {
            if item.info_requested {
                let repr = self.get_item_code_or_name(id);
                println!("{}", repr);
            }
        }
    }

    /// Tries to get code. If that fails, gets a name instead.
    pub fn get_item_code_or_name(&self, item_id: ItemId) -> String {
        if let Some(code) = self.get_item_code(&self.items[item_id.0].definition) {
            return code;
        } else if let Some(name) = self.get_item_name(item_id) {
            return name;
        } else {
            return format!("anonymous");
        }
    }

    /// Tries to get code. If that fails, gets a name instead.
    pub fn get_item_name_or_code(&self, item_id: ItemId) -> String {
        if let Some(name) = self.get_item_name(item_id) {
            return name;
        } else if let Some(code) = self.get_item_code(&self.items[item_id.0].definition) {
            return code;
        } else {
            return format!("anonymous");
        }
    }

    pub fn get_item_code(&self, item: &Item) -> Option<String> {
        match item {
            Item::Defining { base, .. } | Item::TypeIs { base, .. } => {
                self.get_item_code(&self.items[base.0].definition)
            }
            Item::InductiveValue {
                records,
                typee,
                variant_name,
            } => self.get_inductive_value_code(records, typee, variant_name),
            Item::IsSameVariant { base, other } => self.get_is_variant_code(base, other),
            Item::Pick {
                elif_clauses,
                else_clause,
                initial_clause,
            } => self.get_pick_code(elif_clauses, else_clause, initial_clause),
            Item::PrimitiveOperation(op) => self.get_primitive_operation_code(op),
            Item::PrimitiveValue(val) => self.get_primitive_value_code(*val),
            Item::Replacing {
                base, replacements, ..
            } => self.get_replacing_code(base, replacements),
            _ => None,
        }
    }

    fn get_item_name(&self, id: ItemId) -> Option<String> {
        self.get_item_name_impl(id, vec![])
    }

    fn get_item_name_impl(&self, id: ItemId, already_checked: Vec<ItemId>) -> Option<String> {
        let mut choices = Vec::new();
        for (index, potential_parent) in self.items.iter().enumerate() {
            let parent_id = ItemId(index);
            if already_checked.contains(&parent_id) {
                continue;
            }
            if let Item::Defining {
                base, definitions, ..
            } = &potential_parent.definition
            {
                let new_checked = [already_checked.clone(), vec![parent_id]].concat();
                if base == &id {
                    if let Some(name) = self.get_item_name_impl(parent_id, new_checked) {
                        choices.push(name);
                    }
                } else if let Some(def_index) = definitions.iter().position(|def| def.1 == id) {
                    let parent_name = self.get_item_name_impl(parent_id, new_checked);
                    let this_name = &definitions[def_index].0;
                    if let Some(parent_name) = parent_name {
                        choices.push(format!("{}::{}", parent_name, this_name));
                    } else {
                        choices.push(this_name.clone());
                    }
                }
            }
        }
        choices.into_iter().min_by_key(|e| e.len())
    }

    fn get_inductive_value_code(
        &self,
        records: &Vec<ItemId>,
        typee: &ItemId,
        variant_name: &String,
    ) -> Option<String> {
        let mut res = format!("{}::{}[", self.get_item_name_or_code(*typee), variant_name);
        for value in records {
            let value = indented(&self.get_item_name_or_code(*value));
            res.push_str(&format!("\n    {}", value))
        }
        res.push_str("\n]");
        Some(res)
    }

    fn get_is_variant_code(&self, base: &ItemId, other: &ItemId) -> Option<String> {
        Some(format!(
            "{} is_variant{{{}}}",
            self.get_item_name_or_code(*base),
            self.get_item_name_or_code(*other)
        ))
    }

    fn get_pick_code(
        &self,
        elif_clauses: &Vec<(ItemId, ItemId)>,
        else_clause: &ItemId,
        initial_clause: &(ItemId, ItemId),
    ) -> Option<String> {
        let mut res = String::from("pick{");

        let condition = indented(&self.get_item_name_or_code(initial_clause.0));
        let value = indented(&self.get_item_name_or_code(initial_clause.1));
        res.push_str(&format!("\n   if {}, {}", condition, value));

        for (condition, value) in elif_clauses.iter().copied() {
            let condition = indented(&self.get_item_name_or_code(condition));
            let value = indented(&self.get_item_name_or_code(value));
            res.push_str(&format!("\n   elif {}, {}", condition, value));
        }

        let value = indented(&self.get_item_name_or_code(*else_clause));
        res.push_str(&format!("\n   else {}", value));

        res.push_str("\n}");

        Some(res)
    }

    fn get_integer_operation_code(&self, op: &IntegerMathOperation) -> String {
        use IntegerMathOperation as Imo;
        match op {
            Imo::Sum(a, b) => format!(
                "sum[{} {}]",
                self.get_item_name_or_code(*a),
                self.get_item_name_or_code(*b)
            ),
            Imo::Difference(a, b) => format!(
                "difference[{} {}]",
                self.get_item_name_or_code(*a),
                self.get_item_name_or_code(*b)
            ),
        }
    }

    fn get_primitive_operation_code(&self, op: &PrimitiveOperation) -> Option<String> {
        match op {
            PrimitiveOperation::I32Math(op) => Some(format!(
                "Integer32::{}",
                self.get_integer_operation_code(op)
            )),
        }
    }

    fn get_primitive_value_code(&self, value: PrimitiveValue) -> Option<String> {
        match value {
            PrimitiveValue::Bool(..) => None,
            PrimitiveValue::I32(val) => Some(format!("{}", val)),
        }
    }

    fn get_replacing_code(&self, base: &ItemId, replacements: &Replacements) -> Option<String> {
        let mut res = format!("{}[", self.get_item_name_or_code(*base));
        for (target, value) in replacements {
            let target = self.get_item_name_or_code(*target);
            let value = indented(&self.get_item_name_or_code(*value));
            res.push_str(&format!("\n    {} is {}", target, value))
        }
        res.push_str("\n]");
        Some(res)
    }

    fn get_variable_code(&self, selff: ItemId) -> Option<String> {
        self.get_item_name(selff)
    }
}
