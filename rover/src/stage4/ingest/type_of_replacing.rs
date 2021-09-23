use crate::{
    shared::{Item, ItemId, PrimitiveOperation, PrimitiveValue, Replacements},
    stage3::structure::{self as stage3},
    stage4::structure::Environment,
};

impl Environment {
    pub fn type_of_replacing(
        &mut self,
        replacing_id: ItemId,
        base: ItemId,
        replacements: Replacements,
    ) -> Result<ItemId, String> {
        let after_reps = self.compute_type_after_replacing(base, replacements)?;
        // These are the variables that unlabeled replacements might refer to.
        let mut remaining_variables_after_reps = self.get_from_variables(after_reps)?;
        // The same as above, but a mutable reference.
        match &mut self.items[replacing_id.0].base {
            Item::Replacing {
                replacements,
                unlabeled_replacements,
                ..
            } => {
                for unlabeled_replacement in unlabeled_replacements.drain(..) {
                    if remaining_variables_after_reps.len() == 0 {
                        todo!("Nice error, no more variables to replace.");
                    }
                    let target = remaining_variables_after_reps.pop_front().unwrap();
                    replacements.push((target, unlabeled_replacement))
                }
                let replacements = replacements.clone();
                self.compute_type_after_replacing(base, replacements)
            }
            _ => unreachable!(),
        }
    }
}
