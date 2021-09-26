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
                let mut result = self.get_from_variables(base, currently_computing)?;
                result.append(&vars[..]);
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

    pub fn with_from_vars(
        &mut self,
        base: ItemId,
        from_vars: VarList,
        defined_in: Option<ItemId>,
    ) -> ItemId {
        if from_vars.len() > 0 {
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
