use super::helpers::Context;
use crate::{
    shared::{
        BuiltinOperation, Definitions, IntegerMathOperation, Item, ItemId, Replacements, VarList,
    },
    stage2::structure::UnresolvedItem,
    stage3::structure::ItemDefinition,
};

impl Context {
    fn convert_integer_math_operation(
        &mut self,
        imo: &IntegerMathOperation,
    ) -> Result<IntegerMathOperation, String> {
        Ok(match imo {
            IntegerMathOperation::Sum(a, b) => {
                IntegerMathOperation::Sum(self.convert_iid(*a)?, self.convert_iid(*b)?)
            }
            IntegerMathOperation::Difference(a, b) => {
                IntegerMathOperation::Difference(self.convert_iid(*a)?, self.convert_iid(*b)?)
            }
        })
    }

    fn convert_builtin_operation(
        &mut self,
        op: &BuiltinOperation,
    ) -> Result<BuiltinOperation, String> {
        Ok(match op {
            BuiltinOperation::AreSameVariant { base, other } => BuiltinOperation::AreSameVariant {
                base: self.convert_iid(*base)?,
                other: self.convert_iid(*other)?,
            },
            BuiltinOperation::I32Math(imo) => {
                BuiltinOperation::I32Math(self.convert_integer_math_operation(imo)?)
            }
            BuiltinOperation::Reinterpret {
                proof_equal,
                original_type,
                new_type,
                original,
            } => BuiltinOperation::Reinterpret {
                proof_equal: self.convert_iid(*proof_equal)?,
                original_type: self.convert_iid(*original_type)?,
                new_type: self.convert_iid(*new_type)?,
                original: self.convert_iid(*original)?,
            },
        })
    }

    pub fn convert_item(&mut self, item: &Item) -> Result<Item, String> {
        Ok(match item {
            Item::Any { selff, typee } => Item::Any {
                selff: self.convert_iid(*selff)?,
                typee: self.convert_iid(*typee)?,
            },
            Item::BuiltinOperation(op) => {
                Item::BuiltinOperation(self.convert_builtin_operation(op)?)
            }
            Item::BuiltinValue(val) => Item::BuiltinValue(*val),
            Item::Defining { base, definitions } => Item::Defining {
                base: self.convert_iid(*base)?,
                definitions: self.convert_definitions(definitions)?,
            },
            Item::FromType { base, vals: vars } => Item::FromType {
                base: self.convert_iid(*base)?,
                vals: self.convert_var_list(vars)?,
            },
            Item::Pick { .. } => todo!(),
            Item::Replacing { base, replacements } => Item::Replacing {
                base: self.convert_iid(*base)?,
                replacements: self.convert_replacements(replacements)?,
            },
            Item::TypeIs {
                base_type_only,
                base,
                typee,
            } => Item::TypeIs {
                base_type_only: *base_type_only,
                base: self.convert_iid(*base)?,
                typee: self.convert_iid(*typee)?,
            },
            Item::Variant { selff, typee } => Item::Variant {
                selff: self.convert_iid(*selff)?,
                typee: self.convert_iid(*typee)?,
            },
        })
    }
}
