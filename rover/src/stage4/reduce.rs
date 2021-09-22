use std::collections::HashMap;

use crate::{
    stage2::structure::{
        Definitions, IntegerMathOperation, ItemId, PrimitiveOperation, PrimitiveValue, Replacements,
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
            Item::IsSameVariant { base, other } => {
                Self::apply_replacements_to(base, reps);
                Self::apply_replacements_to(other, reps);
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
            Item::PrimitiveOperation(op) => match op {
                PrimitiveOperation::I32Math(op) => Self::apply_replacements_to_int_op(op, reps),
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
            Item::IsSameVariant { base, other } => {
                let base = *base;
                let other = *other;
                let rbase_id = self.reduce(base, reps, reduce_defs);
                let rother_id = self.reduce(other, reps, reduce_defs);
                let rbase = &self.items[rbase_id.0];
                let rother = &self.items[rother_id.0];
                match (&rbase.base, &rother.base) {
                    (
                        Item::InductiveValue {
                            variant_name: base_variant,
                            ..
                        },
                        Item::InductiveValue {
                            variant_name: other_variant,
                            ..
                        },
                    ) => {
                        let result = base_variant == other_variant;
                        self.insert_with_type(
                            Item::PrimitiveValue(PrimitiveValue::Bool(result)),
                            self.bool_type(),
                        )
                    }
                    (Item::PrimitiveValue(base_value), Item::PrimitiveValue(other_value)) => {
                        let result = base_value == other_value;
                        self.insert_with_type(
                            Item::PrimitiveValue(PrimitiveValue::Bool(result)),
                            self.bool_type(),
                        )
                    }
                    _ => {
                        if base == rbase_id || other == rother_id {
                            item
                        } else {
                            let item = Item::IsSameVariant {
                                base: rbase_id,
                                other: rother_id,
                            };
                            let id = self.insert(item);
                            self.compute_type(id).unwrap();
                            id
                        }
                    }
                }
            }
            Item::Pick {
                initial_clause,
                elif_clauses,
                else_clause,
            } => {
                let initial_clause = *initial_clause;
                let elif_clauses = elif_clauses.clone();
                let else_clause = *else_clause;
                self.reduce_pick(reps, reduce_defs, initial_clause, elif_clauses, else_clause)
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
                        // If the value to replace with does not depend on other
                        // variables, we should try to plug it in.
                        replacements_after.insert(*target, value);
                    } else {
                        // Otherwise, leave it be.
                        remaining_replacements.push((*target, value));
                        replacements_after.remove(target);
                    }
                }
                if !remaining_replacements.is_empty() {
                    return item;
                }
                let rbase = self.reduce(base, &replacements_after, true);
                if remaining_replacements.is_empty() {
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
            Item::TypeIs { base, .. } => {
                let base = *base;
                self.reduce(base, reps, reduce_defs)
            }
            Item::Variable { .. } => item,
        }
    }

    fn reduce_condition(
        &mut self,
        cond: ItemId,
        reps: &HashMap<ItemId, ItemId>,
        reduce_defs: bool,
    ) -> Result<bool, ItemId> {
        let rcond = self.reduce(cond, reps, reduce_defs);
        
        match &self.items[rcond.0].base {
            Item::PrimitiveValue(val) => Ok(val.expect_bool()),
            _ => Err(rcond),
        }
    }

    fn reduce_pick(
        &mut self,
        reps: &HashMap<ItemId, ItemId>,
        reduce_defs: bool,
        initial_clause: (ItemId, ItemId),
        elif_clauses: Vec<(ItemId, ItemId)>,
        else_clause: ItemId,
    ) -> ItemId {
        // Stores clauses where we don't know if they are true or false yet.
        let mut unknown_clauses = Vec::new();

        match self.reduce_condition(initial_clause.0, reps, reduce_defs) {
            Ok(true) => {
                debug_assert_eq!(unknown_clauses.len(), 0);
                return self.reduce(initial_clause.1, reps, reduce_defs);
            }
            Ok(false) => (),
            Err(reduced) => {
                unknown_clauses.push((reduced, self.reduce(initial_clause.1, reps, reduce_defs)))
            }
        }

        for (cond, val) in elif_clauses {
            match self.reduce_condition(cond, reps, reduce_defs) {
                Ok(true) => {
                    let val = self.reduce(val, reps, reduce_defs);
                    if unknown_clauses.is_empty() {
                        // Only return if we know for sure no previous clauses will be used.
                        return val;
                    }
                }
                Ok(false) => (),
                Err(reduced) => {
                    unknown_clauses.push((reduced, self.reduce(val, reps, reduce_defs)))
                }
            }
        }

        let else_value = self.reduce(else_clause, reps, reduce_defs);
        if unknown_clauses.is_empty() {
            return else_value;
        }

        let item = Item::Pick {
            initial_clause: unknown_clauses[0],
            elif_clauses: unknown_clauses.into_iter().skip(1).collect(),
            else_clause: else_value,
        };
        let typ = self.items[else_value.0].typee.unwrap();
        self.insert_with_type(item, typ)
    }
}
