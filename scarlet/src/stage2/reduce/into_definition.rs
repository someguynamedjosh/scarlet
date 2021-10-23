use crate::stage2::structure::{
    BuiltinOperation, BuiltinValue, Definition, Environment, ItemId, StructField,
};

impl<'x> Environment<'x> {
    pub(super) fn reduce_builtin_op(
        &mut self,
        op: BuiltinOperation,
        args: Vec<ItemId<'x>>,
    ) -> Definition<'x> {
        match op {
            BuiltinOperation::Sum32U => {
                if let Some(arg_values) = self.args_as_builtin_values(&args[..]) {
                    Definition::BuiltinValue(BuiltinValue::_32U(
                        arg_values[0].unwrap_32u() + arg_values[1].unwrap_32u(),
                    ))
                } else {
                    Definition::BuiltinOperation(op, args)
                }
            }
            BuiltinOperation::Dif32U => {
                if let Some(arg_values) = self.args_as_builtin_values(&args[..]) {
                    Definition::BuiltinValue(BuiltinValue::_32U(
                        arg_values[0].unwrap_32u() - arg_values[1].unwrap_32u(),
                    ))
                } else {
                    Definition::BuiltinOperation(op, args)
                }
            }
            BuiltinOperation::_32UPattern | BuiltinOperation::BoolPattern => {
                Definition::BuiltinOperation(op, args)
            }
        }
    }

    pub(super) fn reduce_struct(&mut self, fields: Vec<StructField<'x>>) -> Definition<'x> {
        let new_fields = fields
            .into_iter()
            .map(|field| {
                let name = field.name;
                let value = self.reduce(field.value);
                StructField { name, value }
            })
            .collect();
        Definition::Struct(new_fields)
    }
}
