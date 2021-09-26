use std::collections::HashMap;

use super::Reps;
use crate::{
    shared::{BuiltinOperation, Definitions, IntegerMathOperation, Item, ItemId, Replacements},
    stage4::structure::Environment,
};

impl Environment {
    /// This is used to replace references to items with references to equal
    /// items which have been reduced more than the original.
    pub fn apply_replacements(&mut self, item_id: ItemId, reps: &Reps) {
        self.apply_replacements_to_info_requested(item_id, reps);
        let item = &mut self.items[item_id.0];
        if let Some(typee) = &mut item.typee {
            Self::apply_replacements_to(typee, reps);
        }
        Self::apply_replacements_to_item(&mut item.definition, reps);
    }

    fn apply_replacements_to_info_requested(&mut self, item_id: ItemId, reps: &Reps) {
        let item = &mut self.items[item_id.0];
        if let Some(scope) = item.info_requested {
            if let Some(new) = reps.get(&item_id) {
                item.info_requested = None;
                self.items[new.0].info_requested = Some(scope);
            }
        }
    }

    fn apply_replacements_to_item(item: &mut Item, reps: &Reps) {
        match item {
            Item::Defining { base, definitions } => {
                Self::apply_replacements_to(base, reps);
                Self::apply_replacements_to_defs(definitions, reps);
            }
            Item::FromType { base, values } => {
                Self::apply_replacements_to(base, reps);
                Self::apply_replacements_to_ids(values, reps);
            }
            Item::GodType => (),
            Item::VariantInstance {
                values,
                typee,
                variant_id,
            } => {
                Self::apply_replacements_to_ids(values, reps);
                Self::apply_replacements_to(typee, reps);
                Self::apply_replacements_to(variant_id, reps);
            }
            Item::Pick {
                initial_clause,
                elif_clauses,
                else_clause,
            } => {
                Self::apply_replacements_to(&mut initial_clause.0, reps);
                Self::apply_replacements_to(&mut initial_clause.1, reps);
                // Replacement coincidentally has the type and behavior we need.
                Self::apply_replacements_to_reps(elif_clauses, reps);
                Self::apply_replacements_to(else_clause, reps);
            }
            Item::BuiltinOperation(op) => match op {
                BuiltinOperation::I32Math(op) => Self::apply_replacements_to_int_op(op, reps),
                BuiltinOperation::AreSameVariant { base, other } => {
                    Self::apply_replacements_to(base, reps);
                    Self::apply_replacements_to(other, reps);
                }
            },
            Item::PrimitiveType(..) | Item::PrimitiveValue(..) => (),
            Item::Replacing {
                base,
                replacements,
                unlabeled_replacements,
            } => {
                Self::apply_replacements_to(base, reps);
                Self::apply_replacements_to_reps(replacements, reps);
                assert_eq!(unlabeled_replacements.len(), 0);
            }
            Item::TypeIs { base, typee, .. } => {
                Self::apply_replacements_to(base, reps);
                Self::apply_replacements_to(typee, reps);
            }
            Item::Variable { selff, typee } => {
                Self::apply_replacements_to(selff, reps);
                Self::apply_replacements_to(typee, reps);
            }
        }
    }

    fn apply_replacements_to(to: &mut ItemId, reps: &HashMap<ItemId, ItemId>) {
        if let Some(rep) = reps.get(&*to) {
            *to = *rep;
        }
    }

    fn apply_replacements_to_ids(to: &mut Vec<ItemId>, reps: &HashMap<ItemId, ItemId>) {
        for id in to {
            Self::apply_replacements_to(id, reps);
        }
    }

    fn apply_replacements_to_defs(to: &mut Definitions, reps: &HashMap<ItemId, ItemId>) {
        for (_, def) in to {
            Self::apply_replacements_to(def, reps);
        }
    }

    fn apply_replacements_to_reps(to: &mut Replacements, reps: &HashMap<ItemId, ItemId>) {
        for (target, val) in to {
            Self::apply_replacements_to(target, reps);
            Self::apply_replacements_to(val, reps);
        }
    }

    fn apply_replacements_to_int_op(to: &mut IntegerMathOperation, reps: &HashMap<ItemId, ItemId>) {
        use IntegerMathOperation as Imo;
        match to {
            Imo::Sum(l, r) => {
                Self::apply_replacements_to(l, reps);
                Self::apply_replacements_to(r, reps);
            }
            Imo::Difference(l, r) => {
                Self::apply_replacements_to(l, reps);
                Self::apply_replacements_to(r, reps);
            }
        }
    }
}
