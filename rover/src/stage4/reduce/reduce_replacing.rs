use super::ReduceOptions;
use crate::{
    shared::{Item, ItemId, Replacements},
    stage4::structure::Environment,
};

impl Environment {
    pub fn reduce_replacing(
        &mut self,
        opts: ReduceOptions,
        base: ItemId,
        replacements: Replacements,
        base_defined_in: Option<ItemId>,
    ) -> ItemId {
        // Do not replace anything this new replacement statement
        // replaces, because this statement is replacing those with
        // potentially different values. Only replace ones it does not
        // mention.
        let mut replacements_after = opts.reps.clone();
        let replacements_here = replacements.clone();
        let mut remaining_replacements = Vec::new();
        for (target, value) in &replacements_here {
            let value = self.reduce(opts.with_item(*value));
            let typee = self.items[value.0].typee.unwrap();
            if self.type_is_not_from(typee) {
                // If the value to replace with does not depend on other
                // variables, we should try to plug it in.
                replacements_after.insert(*target, value);
            } else {
                // Otherwise, leave it be.
                remaining_replacements.push((*target, value));
                replacements_after.remove(target);
            }
        }
        if !remaining_replacements.is_empty() {
            return opts.item;
        }
        let new_opts = ReduceOptions {
            item: base,
            defined_in: base_defined_in,
            reps: &replacements_after,
            reduce_defs: true,
        };
        let rbase = self.reduce(new_opts);
        if remaining_replacements.is_empty() {
            rbase
        } else {
            let item = Item::Replacing {
                base: rbase,
                replacements: remaining_replacements,
                unlabeled_replacements: Vec::new(),
            };
            let id = self.insert(item, opts.defined_in);
            self.compute_type(id, vec![]).unwrap();
            id
        }
    }
}
