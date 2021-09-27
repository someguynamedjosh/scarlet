use super::ReduceOptions;
use crate::{
    shared::{BuiltinOperation, IntegerMathOperation, Item, ItemId, PrimitiveValue},
    stage4::structure::Environment,
};

impl Environment {
    fn compute_primitive(
        op: BuiltinOperation,
        input_values: Vec<PrimitiveValue>,
    ) -> Option<PrimitiveValue> {
        match op {
            BuiltinOperation::I32Math(op) => {
                let vals: Vec<_> = input_values
                    .iter()
                    .map(PrimitiveValue::as_i32)
                    .collect::<Option<_>>()?;
                assert_eq!(vals.len(), 2);
                let val = match op {
                    IntegerMathOperation::Sum(..) => vals[0] + vals[1],
                    IntegerMathOperation::Difference(..) => vals[0] - vals[1],
                };
                Some(PrimitiveValue::I32(val))
            }
            BuiltinOperation::AreSameVariant { .. } => unreachable!(),
        }
    }

    fn compute(&mut self, op: BuiltinOperation, input_values: Vec<ItemId>) -> Option<Item> {
        match op {
            BuiltinOperation::I32Math(..) => {
                let val = Self::compute_primitive(
                    op,
                    input_values
                        .iter()
                        .copied()
                        .map(|i| self.get(i).definition.as_primitive_value())
                        .collect::<Option<_>>()?,
                )?;
                Some(Item::PrimitiveValue(val))
            }
            BuiltinOperation::AreSameVariant { .. } => match (
                &self.get(input_values[0]).definition,
                &self.get(input_values[1]).definition,
            ) {
                (
                    Item::VariantInstance { variant_id: a, .. },
                    Item::VariantInstance { variant_id: b, .. },
                ) => {
                    let (a, b) = (*a, *b);
                    let same = self.are_def_equal(a, b);
                    Some(Item::PrimitiveValue(PrimitiveValue::Bool(same)))
                }
                (Item::PrimitiveValue(a), Item::PrimitiveValue(b)) => {
                    let same = a == b;
                    Some(Item::PrimitiveValue(PrimitiveValue::Bool(same)))
                }
                _ => None,
            },
        }
    }

    pub fn reduce_builtin_operation(
        &mut self,
        opts: ReduceOptions,
        op: BuiltinOperation,
    ) -> ItemId {
        let inputs = op.inputs();
        let mut reduced_inputs = Vec::new();
        for input in &inputs {
            let reduced = self.reduce(opts.with_item(*input));
            reduced_inputs.push(reduced);
        }
        if let Some(value) = self.compute(op.clone(), reduced_inputs.clone()) {
            self.insert_and_compute_type(value, opts.defined_in)
                .unwrap()
        } else {
            let op = op.with_inputs(reduced_inputs);
            self.insert_and_compute_type(Item::BuiltinOperation(op), opts.defined_in)
                .unwrap()
        }
    }
}
