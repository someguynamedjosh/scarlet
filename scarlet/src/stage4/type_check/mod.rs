use crate::{
    shared::{Item, ItemId},
    stage4::structure::Environment,
};
mod type_check_basics;
mod type_check_pick;

pub fn type_check(env: &Environment) -> Result<(), String> {
    let mut next_item = ItemId(0);
    while next_item.0 < env.items.len() {
        env.type_check(next_item)?;
        next_item.0 += 1;
    }
    Ok(())
}

impl Environment {
    /// Checks that, if this item is a Replacing item, that it obeys a type
    /// check.
    fn type_check(&self, item: ItemId) -> Result<(), String> {
        match &self.items[item.0].definition {
            // TODO: BuiltinOperation
            Item::FromType { values, .. } => self.type_check_from_type(values),
            Item::Pick {
                initial_clause,
                elif_clauses,
                else_clause,
            } => self.type_check_pick(*initial_clause, elif_clauses, *else_clause),
            Item::Replacing { replacements, .. } => self.type_check_replacing(item, replacements),
            Item::TypeIs { exact, base, typee } => self.type_check_type_is(*exact, *base, *typee),
            Item::VariantInstance {
                typee, variant_id, ..
            } => self.type_check_inductive_value(*typee, *variant_id),
            _ => Ok(()),
        }
    }
}
