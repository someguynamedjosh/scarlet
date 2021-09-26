use super::ReduceOptions;
use crate::{
    shared::{Item, ItemId},
    stage4::structure::Environment,
};

impl Environment {
    pub fn reduce_inductive_value(
        &mut self,
        opts: ReduceOptions,
        typee: ItemId,
        values: Vec<ItemId>,
        variant_id: ItemId,
    ) -> ItemId {
        let rtypee = self.reduce(opts.with_item(typee));
        let mut new_values = Vec::new();
        for value in &values {
            let rvalue = self.reduce(opts.with_item(*value));
            new_values.push(rvalue);
        }
        if new_values == values && rtypee == typee {
            opts.item
        } else {
            let item = Item::VariantInstance {
                typee: rtypee,
                values: new_values,
                variant_id,
            };
            let id = self.insert(item, opts.defined_in);
            self.compute_type(id, vec![]).unwrap();
            id
        }
    }
}
