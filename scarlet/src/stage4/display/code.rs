use crate::{
    shared::{
        BuiltinOperation, BuiltinValue, ConditionalClause, Definitions, IntegerMathOperation, Item,
        ItemId, Replacements, VarList,
    },
    stage4::{display::Context, structure::Environment},
    util::indented,
};

impl Environment {
    pub(super) fn get_item_code(&self, item_id: &ItemId, ctx: Context) -> Option<String> {
        if ctx.in_type == Some(*item_id) {
            return Some(format!("Self"));
        }
        let item = &self.items[item_id.0].definition;
        match item {
            Item::Any { typee, .. } => self.get_variable_code(typee, ctx),
            Item::BuiltinOperation(op) => self.get_primitive_operation_code(op, ctx),
            Item::BuiltinValue(val) => self.get_builtin_value_code(*val),
            Item::Defining { base, definitions } => {
                self.get_defining_code(item_id, base, definitions, ctx)
            }
            Item::FromType { base, vars } => self.get_from_type_code(base, vars, ctx),
            Item::Pick { clauses, default } => self.get_pick_code(clauses, default, ctx),
            Item::Replacing {
                base, replacements, ..
            } => self.get_replacing_code(base, replacements, ctx),
            Item::TypeIs { base, .. } => self.get_item_code(base, ctx),
            Item::Variant { .. } => None,
        }
    }

    fn get_defining_code(
        &self,
        selff: &ItemId,
        base: &ItemId,
        defines: &Definitions,
        ctx: Context,
    ) -> Option<String> {
        let mut res = self.get_item_code(base, ctx.with_in_scope(*selff))?;
        res.push_str("\ndefining{");
        for (name, val) in defines {
            let val = self.get_item_name_or_code(*val, ctx);
            res.push_str(&format!("\n    {} is {}", name, indented(&val)));
        }
        res.push_str("\n}");
        Some(res)
    }

    fn get_from_type_code(&self, base: &ItemId, vars: &VarList, ctx: Context) -> Option<String> {
        let mut res = self.get_item_name_or_code(*base, ctx);
        res.push_str(" From{ ");
        for var in vars {
            res.push_str(&self.get_item_name_or_code(*var, ctx));
            res.push_str(" ");
        }
        res.push_str("}");
        Some(res)
    }

    fn get_pick_code(
        &self,
        clauses: &Vec<ConditionalClause>,
        default: &ItemId,
        ctx: Context,
    ) -> Option<String> {
        let mut res = String::from("pick{");

        let mut first = true;
        for (condition, value) in clauses.iter().copied() {
            let condition = indented(&self.get_item_name_or_code(condition, ctx));
            let value = indented(&self.get_item_name_or_code(value, ctx));
            if first {
                res.push_str("\n   if ");
                first = false;
            } else {
                res.push_str("\n   elif ");
            }
            res.push_str(&format!("{}, {}", condition, value));
        }

        let value = indented(&self.get_item_name_or_code(*default, ctx));
        res.push_str(&format!("\n   else {}", value));

        res.push_str("\n}");

        Some(res)
    }

    fn get_integer_operation_code(&self, op: &IntegerMathOperation, ctx: Context) -> String {
        use IntegerMathOperation as Imo;
        match op {
            Imo::Sum(a, b) => format!(
                "sum {} {}",
                self.get_item_name_or_code(*a, ctx),
                self.get_item_name_or_code(*b, ctx)
            ),
            Imo::Difference(a, b) => format!(
                "difference {} {}",
                self.get_item_name_or_code(*a, ctx),
                self.get_item_name_or_code(*b, ctx)
            ),
        }
    }

    fn get_primitive_operation_code(&self, op: &BuiltinOperation, ctx: Context) -> Option<String> {
        match op {
            BuiltinOperation::I32Math(op) => Some(format!(
                "builtin_item{{i32_{}}}",
                self.get_integer_operation_code(op, ctx)
            )),
            BuiltinOperation::AreSameVariant { base, other } => Some(format!(
                "builtin_item{{are_same_variant {:?} {:?}}}",
                base, other
            )),
            BuiltinOperation::Reinterpret {
                proof_equal,
                original_type,
                new_type,
                original,
            } => Some(format!(
                "builtin_item{{reinterpret {} {} {} {}}}",
                self.get_item_name_or_code(*proof_equal, ctx),
                self.get_item_name_or_code(*original_type, ctx),
                self.get_item_name_or_code(*new_type, ctx),
                self.get_item_name_or_code(*original, ctx),
            )),
        }
    }

    fn get_builtin_value_code(&self, value: BuiltinValue) -> Option<String> {
        match value {
            BuiltinValue::OriginType => Some(format!("builtin_item{{TYPE}}")),
            BuiltinValue::BoolType => Some(format!("builtin_item{{Boolean}}")),
            BuiltinValue::Bool(val) => Some(format!("builtin_item{{{}}}", val)),
            BuiltinValue::I32Type => Some(format!("builtin_item{{Integer32}}")),
            BuiltinValue::I32(val) => Some(format!("{}", val)),
        }
    }

    fn get_replacing_code(
        &self,
        base: &ItemId,
        replacements: &Replacements,
        ctx: Context,
    ) -> Option<String> {
        let mut res = format!("{}[", self.get_item_name_or_code(*base, ctx));
        for (target, value) in replacements {
            let target = self.get_item_name_or_code(*target, ctx);
            let value = indented(&self.get_item_name_or_code(*value, ctx));
            res.push_str(&format!("\n    {} is {}", target, value))
        }
        res.push_str("\n]");
        Some(res)
    }

    fn get_variable_code(&self, typee: &ItemId, ctx: Context) -> Option<String> {
        let type_code = self.get_item_name_or_code(*typee, ctx);
        if type_code.contains("\n") {
            Some(format!("any{{\n    {}\n}}", indented(&type_code)))
        } else {
            Some(format!("any{{{}}}", type_code))
        }
    }
}
