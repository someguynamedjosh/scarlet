use crate::{stage2::structure::ItemId, stage3::structure::Item, stage4::structure::Environment};

pub fn type_check(env: &Environment) -> Result<(), String> {
    let mut next_item = ItemId(0);
    while next_item.0 < env.items.len() {
        env.type_check(next_item)?;
        next_item.0 += 1;
    }
    Ok(())
}

impl Environment {
    /// Checks that, if this item is a Replacing item, that it obeys a type check.
    fn type_check(&self, item: ItemId) -> Result<(), String> {
        match &self.items[item.0].base {
            Item::Replacing { replacements, .. } => {
                for (target, val) in replacements {
                    let var_type = self.after_from(self.get_var_type(*target));
                    let val_type = self.after_from(self.items[val.0].typee.unwrap());
                    if !self.are_def_equal(var_type, val_type) {
                        return Err(format!(
                            "(at {:?}) {:?} and {:?} have differing types",
                            item, target, val
                        ));
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// If the given item is a From type, returns the base type aka the type
    /// that will be produced after the variables are replaced.
    fn after_from(&self, typee: ItemId) -> ItemId {
        match &self.items[typee.0].base {
            Item::Defining { base, .. } => self.after_from(*base),
            Item::FromType { base, .. } => self.after_from(*base),
            Item::GodType
            | Item::InductiveType(..)
            | Item::InductiveValue { .. }
            | Item::PrimitiveType(..)
            | Item::PrimitiveValue(..) => typee,
            Item::Replacing { base, .. } => todo!(),
            Item::Variable { .. } => typee,
        }
    }
}
