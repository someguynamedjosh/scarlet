use crate::{
    shared::{
        IntegerMathOperation, Item, ItemId, PrimitiveOperation, PrimitiveValue, Replacements,
    },
    stage4::structure::Environment,
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
    pub(super) fn get_item_code(&self, item: &Item, in_scope: ItemId) -> Option<String> {
        match item {
            Item::Defining { base, .. } | Item::TypeIs { base, .. } => {
                self.get_item_code(&self.items[base.0].definition, in_scope)
            }
            Item::FromType { base, vars } => self.get_from_type_code(base, vars, in_scope),
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

    fn get_from_type_code(
        &self,
        base: &ItemId,
        vars: &Vec<ItemId>,
        in_scope: ItemId,
    ) -> Option<String> {
        let mut res = self.get_item_name_or_code(*base, in_scope);
        res.push_str(" From{ ");
        for var in vars {
            res.push_str(&self.get_item_name_or_code(*var, in_scope));
            res.push_str(" ");
        }
        res.push_str("}");
        Some(res)
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
}
