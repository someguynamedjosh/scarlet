use crate::{
    shared::{Item, ItemId},
    stage4::structure::Environment,
};

impl Environment {
    pub fn compute_type(&mut self, of: ItemId) -> Result<ItemId, String> {
        assert!(of.0 < self.items.len());
        let item = &self.items[of.0];
        if let Some(typee) = item.typee {
            return Ok(typee);
        }
        let typee = self.compute_type_id(of)?;
        self.set_type(of, typee);
        Ok(typee)
    }

    fn compute_type_id(&mut self, of: ItemId) -> Result<ItemId, String> {
        match &self.items[of.0].base {
            Item::Defining { base, .. } => {
                let base = *base;
                self.compute_type(base)
            }
            Item::FromType { base, .. } => {
                let base = *base;
                self.compute_type(base)
            }
            Item::GodType { .. } => Ok(self.god_type()),
            Item::InductiveType(..) => Ok(self.god_type()),
            Item::InductiveValue { typee, records, .. } => {
                let typee = *typee;
                let records = records.clone();
                self.type_of_inductive_value(typee, records)
            }
            Item::IsSameVariant { base, other } => {
                // The type is a boolean dependent on the variables of the two expressions.
                let base = *base;
                let other = *other;
                self.type_of_is_same_variant(base, other)
            }
            Item::Pick {
                initial_clause,
                elif_clauses,
                else_clause,
            } => {
                let initial_clause = *initial_clause;
                let elif_clauses = elif_clauses.clone();
                let else_clause = *else_clause;
                self.type_of_pick(initial_clause, elif_clauses, else_clause)
            }
            Item::PrimitiveOperation(op) => {
                let op = op.clone();
                self.type_of_primitive_operation(op)
            }
            Item::PrimitiveType(..) => Ok(self.god_type()),
            Item::PrimitiveValue(pv) => self.type_of_primitive_value(pv),
            Item::Replacing {
                base, replacements, ..
            } => {
                let base = *base;
                let replacements = replacements.clone();
                self.type_of_replacing(of, base, replacements)
            }
            Item::TypeIs { exact, typee, base } => {
                let (exact, typee, base) = (*exact, *typee, *base);
                self.type_of_type_is(exact, typee, base)
            }
            Item::Variable { typee, selff } => {
                let (typee, selff) = (*typee, *selff);
                self.type_of_variable(typee, selff)
            }
        }
    }
}
