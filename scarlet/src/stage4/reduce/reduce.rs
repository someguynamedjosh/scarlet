use super::ReduceOptions;
use crate::{
    shared::{Item, ItemId},
    stage4::structure::Environment,
};

impl Environment {
    pub fn reduce(&mut self, opts: ReduceOptions) -> ItemId {
        if let Some(rep) = opts.reps.get(&opts.item) {
            if opts.reps.len() == 1 {
                return *rep;
            } else {
                let mut new_reps = opts.reps.clone();
                new_reps.remove(&opts.item);
                let mut opts = opts;
                opts.item = *rep;
                opts.reps = &new_reps;
                return self.reduce(opts);
            }
        }
        match &self.items[opts.item.0].definition {
            Item::Defining { base, .. } => {
                let base = *base;
                self.reduce_def(opts, base)
            }
            Item::FromType { base, values } => {
                let base = *base;
                let vars = values.clone();
                self.reduce_from_type(opts, base, vars)
            }
            Item::GodType => opts.item,
            Item::VariantInstance {
                typee,
                values,
                variant_id,
            } => {
                let typee = *typee;
                let params = values.clone();
                let variant_id = *variant_id;
                self.reduce_variant_instance(opts, typee, params, variant_id)
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
            Item::BuiltinOperation(op) => {
                let op = op.clone();
                self.reduce_builtin_operation(opts, op)
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
                let base_defined_in = self.items[base.0].defined_in;
                self.reduce_replacing(opts, base, replacements, base_defined_in)
            }
            Item::TypeIs { base, .. } => {
                let base = *base;
                self.reduce(opts.with_item(base))
            }
            Item::Variable { typee, selff } => {
                let (typee, selff) = (*typee, *selff);
                self.reduce_variable(opts, typee, selff)
            }
        }
    }
}
