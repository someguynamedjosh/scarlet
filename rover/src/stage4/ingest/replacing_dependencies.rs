use crate::{
    shared::{Item, ItemId, Replacements},
    stage4::{ingest::var_list::VarList, structure::Environment},
    util::*,
};

impl Environment {
    pub fn replacing_dependencies(
        &mut self,
        this: ItemId,
        base: ItemId,
        replacements: Replacements,
        unlabeled_replacements: Vec<ItemId>,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<VarList, String> {
        let base_deps = self.compute_dependencies(base, currently_computing.clone())?;

        // All dependencies that haven't been replaced after applying the labeled
        // replacements.
        let mut remaining_deps_after_replacements = base_deps
            .as_slice()
            .iter()
            .filter(|dep| !replacements.iter().any(|rep| rep.0 == **dep))
            .cloned()
            .collect::<Vec<_>>();
        // Convert unlabeled to labeled. TODO: Make this take in to account the types of
        // the things being assigned to make more intelligent picks.
        let mut fully_labeled_replacements = replacements.clone();
        for unlabeled in unlabeled_replacements {
            if remaining_deps_after_replacements.len() == 0 {
                todo!("nice error, no more variables to replace.");
            } else {
                let target = remaining_deps_after_replacements.remove(0);
                fully_labeled_replacements.push((target, unlabeled));
            }
        }

        if fully_labeled_replacements.len() != replacements.len() {
            if let Item::Replacing {
                replacements,
                unlabeled_replacements,
                ..
            } = &mut self.items[this.0].definition
            {
                unlabeled_replacements.clear();
                *replacements = fully_labeled_replacements.clone();
            } else {
                unreachable!()
            }
        }

        let mut result = VarList::new();
        let replacements = fully_labeled_replacements;
        for dep in base_deps.into_vec() {
            if let Some(rep) = replacements.iter().find(|r| r.0 == dep && r.0 != r.1) {
                let replaced_with = rep.1;
                let replaced_deps =
                    self.compute_dependencies(replaced_with, currently_computing.clone())?;
                result.append(replaced_deps.as_slice())
            } else {
                result.push(dep);
            }
        }
        MOk(result)
    }
}
