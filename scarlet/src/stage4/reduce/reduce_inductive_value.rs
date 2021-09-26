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
        params: Vec<ItemId>,
        variant_id: ItemId,
    ) -> ItemId {
        let rtypee = self.reduce(opts.with_item(typee));
        let mut new_params = Vec::new();
        for param in &params {
            let rparam = self.reduce(opts.with_item(*param));
            new_params.push(rparam);
        }
        if new_params == params && rtypee == typee {
            opts.item
        } else {
            let item = Item::InductiveValue {
                typee: rtypee,
                params: new_params,
                variant_id,
            };
            let id = self.insert(item, opts.defined_in);
            self.compute_type(id, vec![]).unwrap();
            id
        }
    }
}
