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
        let deps = self.compute_dependencies(selff, vec![]).unwrap();
        let mut relevant_replacements = Replacements::new();
        for dep in deps.as_slice() {
            if let Some(replace_with) = opts.reps.get(dep) {
                relevant_replacements.push((*dep, *replace_with));
            }
        }
        let item = Item::Variable {
            typee: rtype,
            selff,
        };
        let base = self.insert_and_compute_type(item, opts.defined_in).unwrap();
        let item = Item::Replacing {
            base,
            replacements: relevant_replacements,
            unlabeled_replacements: Vec::new(),
        };
        self.insert_and_compute_type(item, opts.defined_in).unwrap()
    }

    pub fn reduce_variant_instance(
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
        let item = Item::VariantInstance {
            typee: rtypee,
            values: new_values,
            variant_id,
        };
        self.insert_and_compute_type(item, opts.defined_in).unwrap()
    }
}
