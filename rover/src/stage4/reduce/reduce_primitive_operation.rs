use super::ReduceOptions;
use crate::{
    shared::{Item, ItemId, PrimitiveOperation},
    stage4::structure::Environment,
};

impl Environment {
    pub fn reduce_primitive_operation(
        &mut self,
        opts: ReduceOptions,
        op: PrimitiveOperation,
    ) -> ItemId {
        let inputs = op.inputs();
        let mut reduced_inputs = Vec::new();
        let mut input_values = Vec::new();
        for input in &inputs {
            let reduced = self.reduce(opts.with_item(*input));
            reduced_inputs.push(reduced);
            if let Item::PrimitiveValue(val) = &self.items[reduced.0].base {
                input_values.push(*val);
            }
        }
        if input_values.len() == reduced_inputs.len() {
            let computed = op.compute(input_values);
            self.insert_with_type(Item::PrimitiveValue(computed), self.op_type(&op))
        } else if reduced_inputs == inputs {
            opts.item
        } else {
            let op = op.with_inputs(reduced_inputs);
            let id = self.insert(Item::PrimitiveOperation(op));
            self.compute_type(id).unwrap();
            id
        }
    }
}
