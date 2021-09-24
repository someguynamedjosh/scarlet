use super::ReduceOptions;
use crate::{
    shared::{Item, ItemId},
    stage4::structure::Environment,
};

impl Environment {
    pub fn reduce_def(&mut self, opts: ReduceOptions, base: ItemId) -> ItemId {
        if opts.reduce_defs {
            let base = base;
            self.reduce(opts.with_item(base))
        } else {
            opts.item
        }
    }

    pub fn reduce_from_type(
        &mut self,
        opts: ReduceOptions,
        base: ItemId,
        vars: Vec<ItemId>,
    ) -> ItemId {
        let rbase = self.reduce(opts.with_item(base));
        if rbase == base {
            opts.item
        } else {
            let item = Item::FromType { base: rbase, vars };
            let id = self.insert(item, opts.defined_in);
            self.compute_type(id).unwrap();
            id
        }
    }
}
