use crate::{
    shared::{
        Definitions, IntegerMathOperation, Item, ItemId, PrimitiveOperation, PrimitiveValue,
        Replacements,
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
            Item::Defining { base, definitions } => {
                self.get_defining_code(item_id, base, definitions, ctx)
            }
            Item::FromType { base, vars } => self.get_from_type_code(base, vars, ctx),
            Item::GodType => Some(format!("TYPE")),
            Item::InductiveType(id) => self.get_inductive_type_code(id, ctx),
            Item::InductiveValue {
                records,
                typee,
                variant_name,
            } => self.get_inductive_value_code(records, typee, variant_name, ctx),
            Item::IsSameVariant { base, other } => self.get_is_variant_code(base, other, ctx),
            Item::Pick {
                elif_clauses,
                else_clause,
                initial_clause,
            } => self.get_pick_code(elif_clauses, else_clause, initial_clause, ctx),
            Item::PrimitiveOperation(op) => self.get_primitive_operation_code(op, ctx),
            Item::PrimitiveValue(val) => self.get_primitive_value_code(*val),
            Item::Replacing {
                base, replacements, ..
            } => self.get_replacing_code(base, replacements, ctx),
            Item::TypeIs { base, .. } => self.get_item_code(base, ctx),
            Item::Variable { typee, .. } => self.get_variable_code(typee, ctx),
            _ => None,
        }
    }

    fn is_inductive_value(&self, subject: &ItemId) -> bool {
        let def = &self.items[subject.0].definition;
        match def {
            Item::Defining { base, .. } | Item::TypeIs { base, .. } => {
                self.is_inductive_value(base)
            }
            Item::InductiveValue { .. } => true,
            _ => false,
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

    fn get_from_type_code(
        &self,
        base: &ItemId,
        vars: &Vec<ItemId>,
        ctx: Context,
    ) -> Option<String> {
        let mut res = self.get_item_name_or_code(*base, ctx);
        res.push_str(" From{ ");
        for var in vars {
            res.push_str(&self.get_item_name_or_code(*var, ctx));
            res.push_str(" ");
        }
        res.push_str("}");
        Some(res)
    }

    fn get_inductive_type_code(&self, defined_in: &ItemId, ctx: Context) -> Option<String> {
        let base_def = &self.items[defined_in.0].definition;
        let defines = if let Item::Defining { definitions, .. } = base_def {
            definitions
        } else {
            unreachable!("Type definition should always be a defining construct.")
        };
        let mut res = format!("Type{{");
        for (name, val_id) in defines {
            if name == "Self" {
                continue;
            }
            let val = self.get_item_code_or_name(*val_id, ctx.with_in_type(*defined_in));
            if self.is_inductive_value(val_id) {
                res.push_str(&format!("\n    {}", indented(&val)));
            } else {
                res.push_str(&format!("\n    {} is {}", name, indented(&val)));
            }
        }
        res.push_str("\n}");
        Some(res)
    }

    fn get_inductive_value_code(
        &self,
        records: &Vec<ItemId>,
        typee: &ItemId,
        variant_name: &String,
        ctx: Context,
    ) -> Option<String> {
        if let Some(typee) = ctx.in_type {
            let from = self.get_from_type_code(&typee, records, ctx)?;
            Some(format!("variant {} :{}", variant_name, from))
        } else {
            let mut res = format!(
                "{}::{}[",
                self.get_item_name_or_code(*typee, ctx),
                variant_name
            );
            for value in records {
                let value = indented(&self.get_item_name_or_code(*value, ctx));
                res.push_str(&format!("\n    {}", value))
            }
            res.push_str("\n]");
            Some(res)
        }
    }

    fn get_is_variant_code(&self, base: &ItemId, other: &ItemId, ctx: Context) -> Option<String> {
        Some(format!(
            "{} is_variant{{{}}}",
            self.get_item_name_or_code(*base, ctx),
            self.get_item_name_or_code(*other, ctx)
        ))
    }

    fn get_pick_code(
        &self,
        elif_clauses: &Vec<(ItemId, ItemId)>,
        else_clause: &ItemId,
        initial_clause: &(ItemId, ItemId),
        ctx: Context,
    ) -> Option<String> {
        let mut res = String::from("pick{");

        let condition = indented(&self.get_item_name_or_code(initial_clause.0, ctx));
        let value = indented(&self.get_item_name_or_code(initial_clause.1, ctx));
        res.push_str(&format!("\n   if {}, {}", condition, value));

        for (condition, value) in elif_clauses.iter().copied() {
            let condition = indented(&self.get_item_name_or_code(condition, ctx));
            let value = indented(&self.get_item_name_or_code(value, ctx));
            res.push_str(&format!("\n   elif {}, {}", condition, value));
        }

        let value = indented(&self.get_item_name_or_code(*else_clause, ctx));
        res.push_str(&format!("\n   else {}", value));

        res.push_str("\n}");

        Some(res)
    }

    fn get_integer_operation_code(&self, op: &IntegerMathOperation, ctx: Context) -> String {
        use IntegerMathOperation as Imo;
        match op {
            Imo::Sum(a, b) => format!(
                "sum[{} {}]",
                self.get_item_name_or_code(*a, ctx),
                self.get_item_name_or_code(*b, ctx)
            ),
            Imo::Difference(a, b) => format!(
                "difference[{} {}]",
                self.get_item_name_or_code(*a, ctx),
                self.get_item_name_or_code(*b, ctx)
            ),
        }
    }

    fn get_primitive_operation_code(
        &self,
        op: &PrimitiveOperation,
        ctx: Context,
    ) -> Option<String> {
        match op {
            PrimitiveOperation::I32Math(op) => Some(format!(
                "Integer32::{}",
                self.get_integer_operation_code(op, ctx)
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