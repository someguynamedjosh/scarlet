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
        records: Vec<ItemId>,
        variant_id: ItemId,
    ) -> ItemId {
        let rtypee = self.reduce(opts.with_item(typee));
        let mut new_records = Vec::new();
        for record in &records {
            let rrecord = self.reduce(opts.with_item(*record));
            new_records.push(rrecord);
        }
        if new_records == records && rtypee == typee {
            opts.item
        } else {
            let item = Item::InductiveValue {
                typee: rtypee,
                records: new_records,
                variant_id,
            };
            let id = self.insert(item, opts.defined_in);
            self.compute_type(id, vec![]).unwrap();
            id
        }
    }
}
