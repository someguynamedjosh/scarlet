use std::collections::HashMap;

use crate::{
    stage2::structure::{
        Definitions, IntegerMathOperation, ItemId, PrimitiveOperation, Replacements,
    },
    stage3::structure::Item,
    stage4::structure::Environment,
};

pub fn reduce(env: &mut Environment) {
    let mut replacements = HashMap::new();
    let mut id = ItemId(0);
    while id.0 < env.items.len() {
        env.compute_type(id).unwrap();
        let new = env.reduce(id, &HashMap::new(), false);
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
            Imo::Add(l, r) => {
                Self::apply_replacements_to(l, reps);
                Self::apply_replacements_to(r, reps);
            }
            Imo::Subtract(l, r) => {
                Self::apply_replacements_to(l, reps);
                Self::apply_replacements_to(r, reps);
            }
        }
    }

    /// This is used to replace references to items with references to equal
    /// items which have been reduced more than the original.
    fn apply_replacements(&mut self, item: ItemId, reps: &HashMap<ItemId, ItemId>) {
        let item = &mut self.items[item.0];
        if let Some(typee) = &mut item.typee {
            Self::apply_replacements_to(typee, reps);
        }
        match &mut item.base {
            Item::Defining { base, definitions } => {
                Self::apply_replacements_to(base, reps);
                Self::apply_replacements_to_defs(definitions, reps);
            }
            Item::FromType { base, vars } => {
                Self::apply_replacements_to(base, reps);
                Self::apply_replacements_to_ids(vars, reps);
            }
            Item::GodType | Item::InductiveType(..) => (),
            Item::InductiveValue { typee, records, .. } => {
                Self::apply_replacements_to(typee, reps);
                Self::apply_replacements_to_ids(records, reps);
            }
            Item::PrimitiveOperation(op) => match op {
                PrimitiveOperation::I32Math(op) => Self::apply_replacements_to_int_op(op, reps),
            },
            Item::PrimitiveType(..) | Item::PrimitiveValue(..) => (),
            Item::Replacing { base, replacements, unlabeled_replacements } => {
                Self::apply_replacements_to(base, reps);
                Self::apply_replacements_to_reps(replacements, reps);
                assert_eq!(unlabeled_replacements.len(), 0);
            }
            Item::Variable { selff, typee } => {
                Self::apply_replacements_to(selff, reps);
                Self::apply_replacements_to(typee, reps);
            }
        }
    }

    /// Returns true if the provided type does not indicate that corresponding
    /// values depend on the values of other variables.
    fn type_is_not_from(&self, typee: ItemId) -> bool {
        match &self.items[typee.0].base {
            Item::Defining { base, .. } => self.type_is_not_from(*base),
            Item::FromType { base, vars, .. } => vars.len() == 0 && self.type_is_not_from(*base),
            Item::Replacing { base, .. } => self.type_is_not_from(*base),
            _ => true,
        }
    }

    fn reduce(
        &mut self,
        item: ItemId,
        reps: &HashMap<ItemId, ItemId>,
        reduce_defs: bool,
    ) -> ItemId {
        if let Some(rep) = reps.get(&item) {
            return *rep;
        }
        match &self.items[item.0].base {
            Item::Defining { base, .. } => {
                if reduce_defs {
                    let base = *base;
                    self.reduce(base, reps, true)
                } else {
                    item
                }
            }
            Item::FromType { base, vars } => {
                let base = *base;
                let vars = vars.clone();
                let rbase = self.reduce(base, reps, reduce_defs);
                if rbase == base {
                    item
                } else {
                    let item = Item::FromType { base: rbase, vars };
                    let id = self.insert(item);
                    self.compute_type(id).unwrap();
                    id
                }
            }
            Item::GodType | Item::InductiveType(..) => item,
            Item::InductiveValue {
                typee,
                records,
                variant_name,
            } => {
                let typee = *typee;
                let records = records.clone();
                let variant_name = variant_name.clone();
                let mut new_records = Vec::new();
                for record in &records {
                    new_records.push(self.reduce(*record, reps, true));
                }
                if new_records == records {
                    item
                } else {
                    let item = Item::InductiveValue {
                        typee,
                        records: new_records,
                        variant_name,
                    };
                    let id = self.insert(item);
                    self.compute_type(id).unwrap();
                    id
                }
            }
            Item::PrimitiveOperation(op) => {
                let op = op.clone();
                let inputs = op.inputs();
                let mut reduced_inputs = Vec::new();
                let mut input_values = Vec::new();
                for input in &inputs {
                    let reduced = self.reduce(*input, reps, true);
                    reduced_inputs.push(reduced);
                    if let Item::PrimitiveValue(val) = &self.items[reduced.0].base {
                        input_values.push(*val);
                    }
                }
                if input_values.len() == reduced_inputs.len() {
                    let computed = op.compute(input_values);
                    self.insert_with_type(Item::PrimitiveValue(computed), self.op_type(&op))
                } else if reduced_inputs == inputs {
                    item
                } else {
                    let op = op.with_inputs(reduced_inputs);
                    let id = self.insert(Item::PrimitiveOperation(op));
                    self.compute_type(id).unwrap();
                    id
                }
            }
            Item::PrimitiveType(..) | Item::PrimitiveValue(..) => item,
            Item::Replacing {
                base,
                replacements,
                unlabeled_replacements,
            } => {
                // This should be taken care of by stage4::ingest.
                assert_eq!(unlabeled_replacements.len(), 0);
                // Do not replace anything this new replacement statement
                // replaces, because this statement is replacing those with
                // potentially different values. Only replace ones it does not
                // mention.
                let base = *base;
                let mut replacements_after = reps.clone();
                let replacements_here = replacements.clone();
                let mut remaining_replacements = Vec::new();
                for (target, value) in &replacements_here {
                    let value = self.reduce(*value, reps, true);
                    let typee = self.items[value.0].typee.unwrap();
                    if self.type_is_not_from(typee) {
                        // If the value to replace with does not depend on other variables, we should try to plug it in.
                        replacements_after.insert(*target, value);
                    } else {
                        // Otherwise, leave it be.
                        remaining_replacements.push((*target, value));
                        replacements_after.remove(target);
                    }
                }
                let rbase = self.reduce(base, &replacements_after, true);
                if remaining_replacements.len() == 0 {
                    rbase
                } else {
                    let item = Item::Replacing {
                        base: rbase,
                        replacements: remaining_replacements,
                        unlabeled_replacements: Vec::new(),
                    };
                    let id = self.insert(item);
                    self.compute_type(id).unwrap();
                    id
                }
            }
            Item::Variable { .. } => item,
        }
    }
}
