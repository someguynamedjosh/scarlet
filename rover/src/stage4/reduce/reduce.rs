use super::ReduceOptions;
use crate::{
    shared::{Item, ItemId},
    stage4::structure::Environment,
};

impl Environment {
    pub fn reduce(&mut self, opts: ReduceOptions) -> ItemId {
        if let Some(rep) = opts.reps.get(&opts.item) {
            return *rep;
        }
        match &self.items[opts.item.0].definition {
            Item::Defining { base, .. } => {
                let base = *base;
                self.reduce_def(opts, base)
            }
            Item::FromType { base, vars } => {
                let base = *base;
                let vars = vars.clone();
                self.reduce_from_type(opts, base, vars)
            }
            Item::GodType | Item::InductiveType(..) => opts.item,
            Item::InductiveValue {
                typee,
                records,
                variant_name,
            } => {
                let typee = *typee;
                let records = records.clone();
                let variant_name = variant_name.clone();
                self.reduce_inductive_value(opts, typee, records, variant_name)
            }
            Item::IsSameVariant { base, other } => {
                let base = *base;
                let other = *other;
                self.reduce_is_same_variant(opts, base, other)
            }
            Item::Pick {
                initial_clause,
                elif_clauses,
                else_clause,
            } => {
                let initial_clause = *initial_clause;
                let elif_clauses = elif_clauses.clone();
                let else_clause = *else_clause;
                self.reduce_pick(opts, initial_clause, elif_clauses, else_clause)
            }
            Item::PrimitiveOperation(op) => {
                let op = op.clone();
                self.reduce_primitive_operation(opts, op)
            }
            Item::PrimitiveType(..) | Item::PrimitiveValue(..) => opts.item,
            Item::Replacing {
                base,
                replacements,
                unlabeled_replacements,
            } => {
                // This should be taken care of by stage4::ingest.
                assert_eq!(unlabeled_replacements.len(), 0);
                let base = *base;
                let replacements = replacements.clone();
                self.reduce_replacing(opts, base, replacements)
            }
            Item::TypeIs { base, .. } => {
                let base = *base;
                self.reduce(opts.with_item(base))
            }
            Item::Variable { .. } => opts.item,
        }
    }
}
