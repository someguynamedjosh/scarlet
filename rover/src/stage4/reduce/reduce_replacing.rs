use std::collections::HashMap;

use super::ReduceOptions;
use crate::{
    shared::{Item, ItemId, Replacements},
    stage4::{ingest::var_list::VarList, structure::Environment},
};

impl Environment {
    pub fn reduce_replacing(
        &mut self,
        opts: ReduceOptions,
        base: ItemId,
        replacements: Replacements,
        base_defined_in: Option<ItemId>,
    ) -> ItemId {
        let (replacements_after, remaining_replacements) =
            self.compute_replacements_after_replacing(opts, replacements);
        if !remaining_replacements.is_empty() {
            return opts.item;
        }
        let rbase = self.reduce_replacement_base(base, base_defined_in, &replacements_after);
        self.wrap_reduced_base_with_remaining_replacements(opts, rbase, remaining_replacements)
    }

    /// Returns tuples with 0 being a variable and 1 being its type.
    fn annotate_vars(&self, vars: &VarList) -> Vec<(ItemId, ItemId)> {
        vars.as_slice()
            .iter()
            .map(|v| (*v, self.items[v.0].typee.unwrap()))
            .collect()
    }

    fn find_var_with_type(
        &mut self,
        vars: &Vec<(ItemId, ItemId)>,
        desired_type: ItemId,
    ) -> Option<usize> {
        let desired_type = self.after_from(desired_type);
        vars.iter().position(|(_, typ)| {
            let typ = self.after_from(*typ);
            self.are_def_equal(typ, desired_type)
        })
    }

    /// Returns the base value with its variables replaced to match the expected
    /// variables.
    fn replace_variable_to_match_expected(
        &mut self,
        defined_in: Option<ItemId>,
        original_value: ItemId,
        original_type: ItemId,
        desired_vars: &VarList,
    ) -> ItemId {
        let original_vars = self
            .get_from_variables(Some(original_value), original_type, vec![])
            .unwrap();
        let mut original_vars = self.annotate_vars(&original_vars);
        let desired_vars = self.annotate_vars(desired_vars);
        let mut replacements = Replacements::new();

        // Try to find an original variable that has a type matching each desired
        // variable.
        for (dvar, dtyp) in desired_vars {
            if let Some(matching_idx) = self.find_var_with_type(&original_vars, dtyp) {
                let (ovar, _) = original_vars.remove(matching_idx);
                replacements.push((ovar, dvar));
            }
        }

        if replacements.len() == 0 {
            original_value
        } else {
            let item = Item::Replacing {
                base: original_value,
                replacements,
                unlabeled_replacements: Vec::new(),
            };
            let id = self.insert(item, defined_in);
            self.compute_type(id, vec![]).unwrap();
            id
        }
    }

    /// Returns replacements to apply given a replacing construct, and the
    /// replacements that should be kept in the statement without immediately
    /// trying to substitute them.
    fn compute_replacements_after_replacing(
        &mut self,
        opts: ReduceOptions,
        replacements: Replacements,
    ) -> (HashMap<ItemId, ItemId>, Replacements) {
        let mut replacements_after = opts.reps.clone();
        let replacements_here = replacements.clone();
        let mut remaining_replacements = Vec::new();
        for (target, value) in &replacements_here {
            let target_type = self.get_var_type(*target);
            let target_expects_vars = self
                .get_from_variables(Some(*target), target_type, vec![])
                .unwrap();

            let value = self.reduce(opts.with_item(*value));
            let typee = self.items[value.0].typee.unwrap();
            let value = self.replace_variable_to_match_expected(
                opts.defined_in,
                value,
                typee,
                &target_expects_vars,
            );
            let typee = self.items[value.0].typee.unwrap();

            if self.type_depends_on_nothing_except(typee, &target_expects_vars) {
                // If the value to replace with does not depend on other
                // variables, we should try to plug it in.
                replacements_after.insert(*target, value);
            } else {
                // Otherwise, leave it be. Make sure to delete the previous replacement for this
                // value so that we don't try to plug it in.
                remaining_replacements.push((*target, value));
                replacements_after.remove(target);
            }
        }
        (replacements_after, remaining_replacements)
    }

    fn reduce_replacement_base(
        &mut self,
        base: ItemId,
        base_defined_in: Option<ItemId>,
        replacements_after: &HashMap<ItemId, ItemId>,
    ) -> ItemId {
        let new_opts = ReduceOptions {
            item: base,
            defined_in: base_defined_in,
            reps: replacements_after,
            reduce_defs: true,
        };
        self.reduce(new_opts)
    }

    fn wrap_reduced_base_with_remaining_replacements(
        &mut self,
        opts: ReduceOptions,
        rbase: ItemId,
        remaining_replacements: Replacements,
    ) -> ItemId {
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
