use std::collections::HashMap;

use super::ReduceOptions;
use crate::{
    shared::{
        Definitions, IntegerMathOperation, Item, ItemId, PrimitiveOperation, PrimitiveValue,
        Replacements,
    },
    stage4::structure::Environment,
};

impl Environment {
    pub fn reduce_inductive_value(
        &mut self,
        opts: ReduceOptions,
        typee: ItemId,
        records: Vec<ItemId>,
        variant_name: String,
    ) -> ItemId {
        let mut new_records = Vec::new();
        for record in &records {
            let rrecord = self.reduce(opts.with_item(*record));
            new_records.push(rrecord);
        }
        if new_records == records {
            opts.item
        } else {
            let item = Item::InductiveValue {
                typee,
                records: new_records,
                variant_name,
            };
            let id = self.insert(item);
            self.compute_type(id).unwrap();
            id
        }
    }
}
