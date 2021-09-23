use std::collections::HashMap;

use crate::{
    shared::{
        Definitions, IntegerMathOperation, Item, ItemId, PrimitiveOperation, PrimitiveValue,
        Replacements,
    },
    stage4::structure::Environment,
};

mod reduce_basics;
mod reduce_inductive_value;
mod reduce_is_same_variant;
mod reduce_pick;
mod reduce_primitive_operation;
mod reduce_replacing;
mod replacements;

pub type Reps = HashMap<ItemId, ItemId>;

#[derive(Clone, Copy)]
pub struct ReduceOptions<'a> {
    item: ItemId,
    reps: &'a Reps,
    reduce_defs: bool,
}

impl<'a> ReduceOptions<'a> {
    fn with_item(mut self, item: ItemId) -> Self {
        self.item = item;
        self
    }
}

pub fn reduce(env: &mut Environment) {
    let mut replacements = HashMap::new();
    let mut id = ItemId(0);
    while id.0 < env.items.len() {
        env.compute_type(id).unwrap();
        let opts = ReduceOptions {
            item: id,
            reduce_defs: false,
            reps: &HashMap::new(),
        };
        let new = env.reduce(opts);
        if new != id {
            replacements.insert(id, new);
        }
        id.0 += 1
    }
    let mut id = ItemId(0);
    while id.0 < env.items.len() {
        env.apply_replacements(id, &replacements);
        id.0 += 1
    }
}

impl Environment {
    /// Returns true if the provided type does not indicate that corresponding
    /// values depend on the values of other variables.
    fn type_is_not_from(&self, typee: ItemId) -> bool {
        match &self.items[typee.0].base {
            Item::Defining { base, .. } => self.type_is_not_from(*base),
            Item::FromType { base, vars, .. } => vars.is_empty() && self.type_is_not_from(*base),
            Item::Replacing { base, .. } => self.type_is_not_from(*base),
            _ => true,
        }
    }

    fn reduce(&mut self, opts: ReduceOptions) -> ItemId {
        if let Some(rep) = opts.reps.get(&opts.item) {
            return *rep;
        }
        match &self.items[opts.item.0].base {
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
