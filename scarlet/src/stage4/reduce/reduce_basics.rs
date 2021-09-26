use super::ReduceOptions;
use crate::{
    shared::{Item, ItemId, Replacements},
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
        values: Vec<ItemId>,
    ) -> ItemId {
        let rbase = self.reduce(opts.with_item(base));
        if rbase == base {
            opts.item
        } else {
            let item = Item::FromType {
                base: rbase,
                values,
            };
            let id = self.insert(item, opts.defined_in);
            self.compute_type(id, vec![]).unwrap();
            id
        }
    }

    pub fn reduce_variable(&mut self, opts: ReduceOptions, typee: ItemId, selff: ItemId) -> ItemId {
        let rtype = self.reduce(opts.with_item(typee));
        let vars = self.compute_dependencies(selff, vec![]).unwrap();
        let mut replacements = Replacements::new();
        for var in vars.as_slice() {
            if let Some(replace_with) = opts.reps.get(var) {
                replacements.push((*var, *replace_with));
            }
        }
        let base = if rtype == typee {
            opts.item
        } else {
            let item = Item::Variable {
                typee: rtype,
                selff,
            };
            let id = self.insert(item, opts.defined_in);
            self.compute_type(id, vec![]).unwrap();
            id
        };
        if replacements.len() == 0 {
            base
        } else {
            let item = Item::Replacing {
                base,
                replacements,
                unlabeled_replacements: Vec::new(),
            };
            let id = self.insert(item, opts.defined_in);
            self.compute_type(id, vec![]).unwrap();
            id
        }
    }
}
