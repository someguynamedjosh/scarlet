use crate::stage2::structure::{
    BuiltinOperation, Definition, Environment, ItemId, StructField, VarType,
};

impl<'x> Environment<'x> {
    pub(in crate::stage2) fn find_bounding_pattern(&mut self, pattern: ItemId<'x>) -> ItemId<'x> {
        match self.get_definition(pattern).clone() {
            Definition::BuiltinOperation(op, _) => match op {
                BuiltinOperation::Sum32U
                | BuiltinOperation::Difference32U
                | BuiltinOperation::Product32U
                | BuiltinOperation::Quotient32U
                | BuiltinOperation::Modulo32U
                | BuiltinOperation::Power32U => self.get_or_push_var(VarType::_32U),
                BuiltinOperation::LessThan32U
                | BuiltinOperation::LessThanOrEqual32U
                | BuiltinOperation::GreaterThan32U
                | BuiltinOperation::GreaterThanOrEqual32U => self.get_or_push_var(VarType::Bool),
            },
            Definition::BuiltinValue(..) => pattern,
            Definition::Unresolved { .. } => {
                let pattern = self.resolve(pattern);
                self.find_bounding_pattern(pattern)
            }
            Definition::Match {
                conditions,
                else_value,
                ..
            } => {
                let mut result = else_value;
                for condition in conditions {
                    let valtype = self.find_bounding_pattern(condition.value);
                    result = self.push_var(VarType::Or(valtype, result));
                }
                result
            }
            Definition::Member(..) => todo!(),
            Definition::Substitute(base, subs) => {
                let subbed = self.substitute(base, &subs).unwrap();
                self.find_bounding_pattern(subbed)
            },
            Definition::SetEager { base, vals, eager } => {
                let base = self.find_bounding_pattern(base);
                let def = Definition::SetEager { base, vals, eager };
                self.item_with_new_definition(pattern, def, true)
            }
            Definition::Struct(fields) => {
                let mut new_fields = Vec::new();
                for field in fields {
                    new_fields.push(StructField {
                        name: field.name,
                        value: self.find_bounding_pattern(field.value),
                    })
                }
                let def = Definition::Struct(new_fields);
                self.item_with_new_definition(pattern, def, true)
            }
            Definition::Variable { typee, var } => {
                // TODO: Make a function to map a var type.
                let typee = match typee {
                    VarType::God | VarType::_32U | VarType::Bool => typee,
                    VarType::Just(other) => VarType::Just(self.find_bounding_pattern(other)),
                    VarType::And(l, r) => {
                        VarType::And(self.find_bounding_pattern(l), self.find_bounding_pattern(r))
                    }
                    VarType::Or(l, r) => {
                        VarType::Or(self.find_bounding_pattern(l), self.find_bounding_pattern(r))
                    }
                };
                let def = Definition::Variable { typee, var };
                self.item_with_new_definition(pattern, def, true)
            }
        }
    }
}
