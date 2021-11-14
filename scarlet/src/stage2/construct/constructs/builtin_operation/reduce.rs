use super::{COperation, Operation};
use crate::stage2::structure::{BuiltinValue, ConstructId, Environment};

impl<'x> Environment<'x> {
    fn reduce_op(
        &mut self,
        original_con: ConstructId<'x>,
        args: Vec<ConstructId<'x>>,
        compute: impl Fn(Vec<BuiltinValue>) -> BuiltinValue,
    ) -> ConstructId<'x> {
        if let Some(args) = self.args_as_builtin_values(&args[..]) {
            todo!("{:?}", compute(args))
        } else {
            original_con
        }
    }

    fn reduce_32u_32u_x_op(
        &mut self,
        original_con: ConstructId<'x>,
        args: Vec<ConstructId<'x>>,
        compute: impl Fn(u32, u32) -> BuiltinValue,
    ) -> ConstructId<'x> {
        self.reduce_op(original_con, args, |args| {
            compute(args[0].unwrap_32u(), args[1].unwrap_32u())
        })
    }

    fn reduce_32u_32u_32u_op(
        &mut self,
        original_con: ConstructId<'x>,
        args: Vec<ConstructId<'x>>,
        compute: impl Fn(u32, u32) -> u32,
    ) -> ConstructId<'x> {
        self.reduce_32u_32u_x_op(original_con, args, |a, b| BuiltinValue::_32U(compute(a, b)))
    }

    fn reduce_32u_32u_bool_op(
        &mut self,
        original_con: ConstructId<'x>,
        args: Vec<ConstructId<'x>>,
        compute: impl Fn(u32, u32) -> bool,
    ) -> ConstructId<'x> {
        self.reduce_32u_32u_x_op(original_con, args, |a, b| BuiltinValue::Bool(compute(a, b)))
    }
}

pub fn implementation<'x>(
    this: &COperation<'x>,
    this_id: ConstructId<'x>,
    env: &mut Environment<'x>,
) -> ConstructId<'x> {
    let args = this.args.clone();
    match this.op {
        Operation::Sum32U => env.reduce_32u_32u_32u_op(this_id, args, |a, b| a + b),
        Operation::Difference32U => env.reduce_32u_32u_32u_op(this_id, args, |a, b| a - b),
        Operation::Product32U => env.reduce_32u_32u_32u_op(this_id, args, |a, b| a * b),
        Operation::Quotient32U => env.reduce_32u_32u_32u_op(this_id, args, |a, b| a / b),
        Operation::Modulo32U => env.reduce_32u_32u_32u_op(this_id, args, |a, b| a % b),
        Operation::Power32U => env.reduce_32u_32u_32u_op(this_id, args, |a, b| a.pow(b)),

        Operation::GreaterThan32U => env.reduce_32u_32u_bool_op(this_id, args, |a, b| a > b),
        Operation::GreaterThanOrEqual32U => {
            env.reduce_32u_32u_bool_op(this_id, args, |a, b| a >= b)
        }
        Operation::LessThan32U => env.reduce_32u_32u_bool_op(this_id, args, |a, b| a < b),
        Operation::LessThanOrEqual32U => env.reduce_32u_32u_bool_op(this_id, args, |a, b| a <= b),
    }
}
