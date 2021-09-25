use std::collections::HashMap;

use super::var_list::VarList;
use crate::{
    shared::{Item, ItemId, Replacements},
    stage4::structure::Environment,
    util::*,
};

impl Environment {
    /// Returns a hashmap with the keys being IDs to replace and the values
    /// being the variables the replaced value depends on.
    fn replacement_dependencies(
        &mut self,
        replacements: Replacements,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<HashMap<ItemId, VarList>, String> {
        let mut replacement_data = HashMap::new();
        for (target, value) in replacements {
            let valtype = self.compute_type(value, currently_computing.clone())?;
            let valtype_vars = self.get_from_variables(valtype)?;
            replacement_data.insert(target, valtype_vars);
        }
        MOk(replacement_data)
    }

    fn from_vars_after_replacing(
        &mut self,
        dependencies: &HashMap<ItemId, VarList>,
        _from_base: ItemId,
        from_vars: &Vec<ItemId>,
    ) -> VarList {
        let mut vars_after_reps = VarList::new();
        for var in from_vars {
            if let Some(replaced_value_vars) = dependencies.get(var) {
                // $var is being replaced with a value that depends on replaced_value_vars.
                vars_after_reps.append(replaced_value_vars.as_slice())
            } else {
                // $var is not being replaced so the expression still depends on it.
                vars_after_reps.push(*var);
            }
        }
        vars_after_reps
    }

    fn type_item_after_replacing(
        &mut self,
        type_id: ItemId,
        type_item: &Item,
        defined_in: Option<ItemId>,
        dependencies: &HashMap<ItemId, VarList>,
    ) -> ItemId {
        // TODO: Handle nested from types? Or maybe add something to prevent nested from
        // types. Also TODO: What if we have a type like Something[var] and we
        // replace var?
        match type_item {
            Item::FromType { base, vars } => {
                let vars_after_reps = self.from_vars_after_replacing(&dependencies, *base, vars);
                if vars_after_reps.as_slice() == vars {
                    type_id
                } else {
                    self.with_from_vars(*base, vars_after_reps, defined_in)
                }
            }
            _ => type_id,
        }
    }

    /// Returns the type of an item after applying the given replacements.
    /// E.G. a + b with replacements a: c should yield Int From{b c}
    pub fn compute_type_after_replacing(
        &mut self,
        base: ItemId,
        replacements: Replacements,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        let dependencies =
            self.replacement_dependencies(replacements, currently_computing.clone())?;
        let original_type_id = self.compute_type(base, currently_computing)?;
        let original_type = &self.items[original_type_id.0];
        let original_type_def = original_type.definition.clone();
        let defined_in = original_type.defined_in.clone();
        MOk(self.type_item_after_replacing(
            original_type_id,
            &original_type_def,
            defined_in,
            &dependencies,
        ))
    }
}
