use crate::stage2::structure::{BuiltinValue, Environment, ItemId, ScopeId, Value};

impl Environment {
    pub fn item_code(&self, item: ItemId, context: ScopeId) -> Option<String> {
        match self[item].value.as_ref().expect("ICE: Undefined value") {
            Value::Any { variable } => {
                let type_code = self.item_name_or_code(self[*variable].original_type, context);
                Some(format!("any{{{}}}", type_code))
            }
            Value::BuiltinOperation { .. } => todo!(),
            Value::BuiltinValue { value } => match value {
                BuiltinValue::OriginType | BuiltinValue::U8Type => None,
                BuiltinValue::U8(value) => Some(format!("{}", value)),
            },
            Value::Defining {
                base, definitions, ..
            } => {
                let mut result = self.item_code_or_name(*base, context);
                result.push_str("\ndefining{");
                for (name, _) in definitions {
                    result.push_str(&format!("\n    {} is ..", name));
                }
                result.push_str("\n}");
                Some(result)
            }
            Value::FromItems { .. } => unreachable!("Should be reduced"),
            Value::FromVariables { base, variables } => {
                let mut result = self.item_name_or_code(*base, context);
                result.push_str("\nFrom{");
                for (variable, _) in variables {
                    let var_def = self[*variable].definition;
                    let text = self.item_name_or_code(var_def, context);
                    result.push_str(&format!("\n    {}", text));
                }
                result.push_str("\n}");
                Some(result)
            }
            Value::Identifier { .. }
            | Value::Item { .. }
            | Value::Member { .. }
            | Value::ReplacingItems { .. } => unreachable!("Should be reduced"),
            Value::ReplacingVariables { .. } => todo!(),
            Value::Variant { variant } => {
                let type_code = self.item_name_or_code(self[*variant].original_type, context);
                Some(format!("variant{{{}}}", type_code))
            }
        }
    }
}
