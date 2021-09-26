use std::thread::current;

use crate::{
    shared::{Item, ItemId, Replacements},
    stage4::{ingest::VarList, structure::Environment},
    util::MaybeResult,
};

impl Environment {
    fn get_replacement(base: ItemId, reps: &Replacements) -> Option<ItemId> {
        let mut res = base;
        while let Some(rep_idx) = reps.iter().position(|i| i.0 == res && i.0 != i.1) {
            res = reps[rep_idx].1;
        }
        if res == base {
            None
        } else {
            Some(res)
        }
    }

    // Collects all variables specified by From items pointed to by the provided ID.
    pub fn get_from_variables(
        &mut self,
        typee: ItemId,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<VarList, String> {
        MaybeResult::Ok(match &self.items[typee.0].definition {
            Item::Defining { base: id, .. } => {
                let id = *id;
                self.get_from_variables(id, currently_computing)?
            }
            Item::FromType { base, vars } => {
                let base = *base;
                let vars = vars.clone();
                let mut result = self.flatten_vars(&vars, currently_computing.clone())?;
                let base_vars = self.get_from_variables(base, currently_computing)?;
                result.append(base_vars.as_slice());
                result
            }
            Item::Replacing {
                base, replacements, ..
            } => {
                let base = *base;
                let replacements = replacements.clone();
                let base_list = self.get_from_variables(base, currently_computing.clone())?;
                let mut after_list = VarList::new();
                for base_var in base_list.into_vec() {
                    if let Some(rep_idx) = replacements.iter().position(|i| i.0 == base_var) {
                        let rep_id = ItemId(rep_idx);
                        let rep_type = self.compute_type(rep_id, currently_computing.clone())?;
                        after_list.append(
                            &self
                                .get_from_variables(rep_type, currently_computing.clone())?
                                .into_vec(),
                        );
                    } else {
                        after_list.push(base_var);
                    }
                }
                after_list
            }
            _ => VarList::new(),
        })
    }

    /// Does things like converting From{dependant} to From{input1 input2
    /// dependant}
    pub fn flatten_type(
        &mut self,
        typee: ItemId,
        currently_computing: Vec<ItemId>,
        extra_vars: &[ItemId],
    ) -> MaybeResult<ItemId, String> {
        let defined_in = self.items[typee.0].defined_in;
        let mut vars = self.get_from_variables(typee, currently_computing.clone())?;
        vars.append(extra_vars);
        let vars = vars.into_vec();
        let base = self.deref_replacing_and_defining(typee);
        let item = Item::FromType { base, vars };
        let id = self.insert(item, defined_in);
        MaybeResult::Ok(id)
    }

    pub fn flatten_vars(
        &mut self,
        vars: &[ItemId],
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<VarList, String> {
        let mut res = VarList::new();
        for var in vars {
            let var_type = self.compute_type(*var, currently_computing.clone())?;
            let var_vars = self.get_from_variables(var_type, currently_computing.clone())?;
            res.append(var_vars.as_slice());
        }
        MaybeResult::Ok(res)
    }

    pub fn with_from_vars(
        &mut self,
        mut base: ItemId,
        mut from_vars: VarList,
        defined_in: Option<ItemId>,
    ) -> ItemId {
        if from_vars.len() > 0 {
            if let Item::FromType {
                base: other_base,
                vars: other_vars,
            } = &self.items[base.0].definition
            {
                base = *other_base;
                let mut all_vars = VarList::from(other_vars.clone());
                all_vars.append(from_vars.as_slice());
                from_vars = all_vars;
            }
            self.insert(
                Item::FromType {
                    base,
                    vars: from_vars.into_vec(),
                },
                defined_in,
            )
        } else {
            base
        }
    }
}
