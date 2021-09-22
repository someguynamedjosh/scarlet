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
    /// Returns the type of the given item, with any from clauses discarded.
    fn item_base_type(&self, of: ItemId) -> ItemId {
        self.after_from(self.items[of.0].typee.unwrap())
    }

    /// Checks that, if this item is a Replacing item, that it obeys a type check.
    fn type_check(&self, item: ItemId) -> Result<(), String> {
        match &self.items[item.0].base {
            Item::Replacing { replacements, .. } => {
                for (target, val) in replacements {
                    let var_type = self.item_base_type(*target);
                    let val_type = self.item_base_type(*val);
                    if !self.are_def_equal(var_type, val_type) {
                        return Err(format!(
                            "(at {:?}) {:?} and {:?} have differing types",
                            item, target, val
                        ));
                    }
                }
                Ok(())
            }
            Item::IsSameVariant { base, other } => {
                let btype = self.item_base_type(*base);
                let otype = self.item_base_type(*other);
                if !self.are_def_equal(btype, otype) {
                    Err(format!(
                        "(at {:?}) {:?} and {:?} have differing types",
                        item, base, other,
                    ))
                } else {
                    Ok(())
                }
            }
            Item::Pick {
                initial_clause,
                elif_clauses,
                else_clause,
            } => {
                let bool_type = self.bool_type();

                let initial_cond_type = self.item_base_type(initial_clause.0);
                if !self.are_def_equal(initial_cond_type, bool_type) {
                    todo!("nice error, condition is not a Boolean");
                }
                let initial_value_type = self.item_base_type(initial_clause.1);

                for (cond, value) in elif_clauses {
                    let cond_type = self.item_base_type(*cond);
                    if !self.are_def_equal(cond_type, bool_type) {
                        todo!("nice error, condition is not a Boolean")
                    }
                    let value_type = self.item_base_type(*value);
                    if !self.are_def_equal(initial_value_type, value_type) {
                        todo!("nice error, elif value type does not match initial value type");
                    }
                }

                let else_value_type = self.item_base_type(*else_clause);
                if !self.are_def_equal(initial_value_type, else_value_type) {
                    todo!("nice error, else value type does not match initial value type");
                }
                Ok(())
            }
            Item::TypeIs { base, typee } => {
                let base_type = self.item_base_type(*base);
                let typee = self.after_from(*typee);
                if self.are_def_equal(base_type, typee) {
                    Ok(())
                } else {
                    todo!("nice error, value does not match type assert")
                }
            }
            _ => Ok(()),
        }
    }

    /// If the given item is a From type, returns the base type aka the type
    /// that will be produced after the variables are replaced.
    pub(super) fn after_from(&self, typee: ItemId) -> ItemId {
        match &self.items[typee.0].base {
            Item::Defining { base, .. } => self.after_from(*base),
            Item::FromType { base, .. } => self.after_from(*base),
            Item::GodType
            | Item::InductiveType(..)
            | Item::InductiveValue { .. }
            | Item::IsSameVariant { .. }
            | Item::Pick { .. }
            | Item::PrimitiveOperation(..)
            | Item::PrimitiveType(..)
            | Item::PrimitiveValue(..) => typee,
            Item::Replacing { .. } => todo!(),
            Item::TypeIs { base, .. } => self.after_from(*base),
            Item::Variable { .. } => typee,
        }
    }
}
