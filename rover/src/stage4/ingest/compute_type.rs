use crate::{
    shared::{Item, ItemId},
    stage4::structure::Environment,
    util::*,
};

impl Environment {
    pub fn compute_type(
        &mut self,
        of: ItemId,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        assert!(of.0 < self.items.len());
        if currently_computing.contains(&of) {
            return MNone;
        }
        let item = &self.items[of.0];
        if let Some(typee) = item.typee {
            return MOk(typee);
        }
        let item = &self.items[of.0];
        let defined_in = item.defined_in;
        let new_computing = [currently_computing, vec![of]].concat();
        let typee = self.compute_type_id(of, defined_in, new_computing)?;
        self.set_type(of, typee);
        MOk(typee)
    }

    fn compute_type_id(
        &mut self,
        of: ItemId,
        defined_in: Option<ItemId>,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        let item = &self.items[of.0];
        match &item.definition {
            Item::Defining { base, .. } => {
                let base = *base;
                self.compute_type(base, currently_computing)
            }
            Item::FromType { base, .. } => {
                let base = *base;
                self.compute_type(base, currently_computing)
            }
            Item::GodType { .. } => MOk(self.god_type()),
            Item::InductiveValue { typee, records, .. } => {
                let (typee, records) = (*typee, records.clone());
                self.type_of_inductive_value(typee, records, defined_in, currently_computing)
            }
            Item::IsSameVariant { base, other } => {
                // The type is a boolean dependent on the variables of the two expressions.
                let (base, other) = (*base, *other);
                self.type_of_is_same_variant(base, other, defined_in, currently_computing)
            }
            Item::Pick {
                initial_clause,
                elif_clauses,
                else_clause,
            } => {
                let initial_clause = *initial_clause;
                let elif_clauses = elif_clauses.clone();
                let else_clause = *else_clause;
                self.type_of_pick(
                    initial_clause,
                    elif_clauses,
                    else_clause,
                    defined_in,
                    currently_computing,
                )
            }
            Item::PrimitiveOperation(op) => {
                let op = op.clone();
                self.type_of_primitive_operation(op, defined_in)
            }
            Item::PrimitiveType(..) => MOk(self.god_type()),
            Item::PrimitiveValue(pv) => self.type_of_primitive_value(pv),
            Item::Replacing {
                base, replacements, ..
            } => {
                let (base, replacements) = (*base, replacements.clone());
                self.type_of_replacing(of, base, replacements, currently_computing)
            }
            Item::TypeIs { exact, typee, base } => {
                let (exact, typee, base) = (*exact, *typee, *base);
                self.type_of_type_is(exact, typee, base, currently_computing)
            }
            Item::Variable { typee, selff } => {
                let (typee, selff) = (*typee, *selff);
                self.type_of_variable(typee, selff, currently_computing)
            }
        }
    }
}
