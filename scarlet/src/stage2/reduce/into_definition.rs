use crate::stage2::structure::{BuiltinOperation, BuiltinValue, Definition, Environment, ItemId};

impl<'x> Environment<'x> {
    fn reduce_op(
        &mut self,
        original_def: Definition<'x>,
        args: Vec<ItemId<'x>>,
        compute: impl Fn(Vec<BuiltinValue>) -> BuiltinValue,
    ) -> Definition<'x> {
        if let Some(args) = self.args_as_builtin_values(&args[..]) {
            Definition::BuiltinValue(compute(args))
        } else {
            original_def
        }
    }

    fn reduce_32u_32u_x_op(
        &mut self,
        original_def: Definition<'x>,
        args: Vec<ItemId<'x>>,
        compute: impl Fn(u32, u32) -> BuiltinValue,
    ) -> Definition<'x> {
        self.reduce_op(original_def, args, |args| {
            compute(args[0].unwrap_32u(), args[1].unwrap_32u())
        })
    }

    fn reduce_32u_32u_32u_op(
        &mut self,
        original_def: Definition<'x>,
        args: Vec<ItemId<'x>>,
        compute: impl Fn(u32, u32) -> u32,
    ) -> Definition<'x> {
        self.reduce_32u_32u_x_op(original_def, args, |a, b| BuiltinValue::_32U(compute(a, b)))
    }

    fn reduce_32u_32u_bool_op(
        &mut self,
        original_def: Definition<'x>,
        args: Vec<ItemId<'x>>,
        compute: impl Fn(u32, u32) -> bool,
    ) -> Definition<'x> {
        self.reduce_32u_32u_x_op(original_def, args, |a, b| BuiltinValue::Bool(compute(a, b)))
    }

    pub(super) fn reduce_builtin_op(
        &mut self,
        def: Definition<'x>,
        op: BuiltinOperation,
        args: Vec<ItemId<'x>>,
    ) -> Definition<'x> {
        match op {
            BuiltinOperation::Sum32U => self.reduce_32u_32u_32u_op(def, args, |a, b| a + b),
            BuiltinOperation::Difference32U => self.reduce_32u_32u_32u_op(def, args, |a, b| a - b),
            BuiltinOperation::Product32U => self.reduce_32u_32u_32u_op(def, args, |a, b| a * b),
            BuiltinOperation::Quotient32U => self.reduce_32u_32u_32u_op(def, args, |a, b| a / b),
            BuiltinOperation::Modulo32U => self.reduce_32u_32u_32u_op(def, args, |a, b| a % b),
            BuiltinOperation::Power32U => self.reduce_32u_32u_32u_op(def, args, |a, b| a.pow(b)),

            BuiltinOperation::GreaterThan32U => {
                self.reduce_32u_32u_bool_op(def, args, |a, b| a > b)
            }
            BuiltinOperation::GreaterThanOrEqual32U => {
                self.reduce_32u_32u_bool_op(def, args, |a, b| a >= b)
            }
            BuiltinOperation::LessThan32U => self.reduce_32u_32u_bool_op(def, args, |a, b| a < b),
            BuiltinOperation::LessThanOrEqual32U => {
                self.reduce_32u_32u_bool_op(def, args, |a, b| a <= b)
            }
        }
    }
}
