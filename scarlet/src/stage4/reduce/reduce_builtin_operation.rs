use super::ReduceOptions;
use crate::{
    shared::{BuiltinOperation, IntegerMathOperation, Item, ItemId, PrimitiveValue},
    stage4::structure::Environment,
};

enum ItemOrId {
    Item(Item),
    ItemId(ItemId),
}

impl From<Item> for ItemOrId {
    fn from(item: Item) -> Self {
        Self::Item(item)
    }
}

impl From<ItemId> for ItemOrId {
    fn from(item_id: ItemId) -> Self {
        Self::ItemId(item_id)
    }
}

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
            BuiltinOperation::AreSameVariant { .. } | BuiltinOperation::Reinterpret { .. } => {
                unreachable!()
            }
        }
    }

    fn compute(&mut self, op: BuiltinOperation, input_values: Vec<ItemId>) -> Option<ItemOrId> {
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
                Some(Item::PrimitiveValue(val).into())
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
                    Some(Item::PrimitiveValue(PrimitiveValue::Bool(same)).into())
                }
                (Item::PrimitiveValue(a), Item::PrimitiveValue(b)) => {
                    let same = a == b;
                    Some(Item::PrimitiveValue(PrimitiveValue::Bool(same)).into())
                }
                _ => None,
            },
            BuiltinOperation::Reinterpret { .. } => {
                let item = self.get(input_values[3]);
                let def = item.definition.clone();
                let defined_in = item.defined_in;
                Some(
                    self.insert_with_type(def, input_values[2], defined_in)
                        .into(),
                )
            }
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
        if let Some(item) = self.compute(op.clone(), reduced_inputs.clone()) {
            match item {
                ItemOrId::Item(item) => {
                    self.insert_and_compute_type(item, opts.defined_in).unwrap()
                }
                ItemOrId::ItemId(id) => id,
            }
        } else {
            let op = op.with_inputs(reduced_inputs);
            self.insert_and_compute_type(Item::BuiltinOperation(op), opts.defined_in)
                .unwrap()
        }
    }
}
