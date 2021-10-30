use crate::stage2::structure::{
    BuiltinOperation, BuiltinValue, Definition, Environment, ItemId, Pattern, StructField,
    VariableId, VariableInfo,
};

impl<'x> Environment<'x> {
    fn reduce_builtin_op_impl(
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
        }
    }

    pub(super) fn reduce_builtin_op(
        &mut self,
        op: BuiltinOperation,
        args: Vec<ItemId<'x>>,
    ) -> Definition<'x> {
        let args = args.into_iter().map(|arg| self.reduce(arg)).collect();
        self.reduce_builtin_op_impl(op, args)
    }

    pub fn reduce_pattern(&mut self, pat: Pattern<'x>) -> Definition<'x> {
        match pat {
            Pattern::Bool | Pattern::_32U | Pattern::God | Pattern::Pattern => pat,
            Pattern::Capture(info) => Pattern::Capture(VariableInfo {
                var_item: self.reduce(info.var_item),
                var: info.var,
                pattern: self.reduce(info.pattern),
            }),
            Pattern::And(left, right) => {
                let left = self.reduce(left);
                let right = self.reduce(right);
                Pattern::And(left, right)
            }
        }
        .into()
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

    pub(super) fn reduce_var(
        &mut self,
        var: VariableId<'x>,
        pattern: ItemId<'x>,
        _def: Definition<'x>,
    ) -> Definition<'x> {
        let pattern = self.reduce(pattern);
        Definition::Variable { var, pattern }
    }
}
