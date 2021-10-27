use crate::stage2::{
    matchh::MatchResult,
    structure::{
        BuiltinOperation, BuiltinPattern, BuiltinValue, Definition, Environment, ItemId,
        StructField, VariableId,
    },
};

impl<'x> Environment<'x> {
    fn reduce_finite_builtin_op(
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
        self.reduce_finite_builtin_op(op, args)
    }

    pub(super) fn reduce_builtin_pattern(&mut self, pat: BuiltinPattern<'x>) -> Definition<'x> {
        match pat {
            BuiltinPattern::Bool | BuiltinPattern::_32U | BuiltinPattern::God => {
                Definition::BuiltinPattern(pat)
            }
            BuiltinPattern::And(left, right) => {
                let left = self.reduce(left);
                let right = self.reduce(right);
                let pat = BuiltinPattern::And(left, right);
                Definition::BuiltinPattern(pat)
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

    pub(super) fn reduce_var(
        &mut self,
        var: VariableId<'x>,
        def: Definition<'x>,
    ) -> Definition<'x> {
        let pattern = self.vars[var].pattern;
        let pattern = self.reduce(pattern);
        self.vars[var].pattern = pattern;
        def
    }
}
