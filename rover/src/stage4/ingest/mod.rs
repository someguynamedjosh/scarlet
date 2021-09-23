use std::collections::HashMap;

use var_list::VarList;

use crate::{
    shared::{Item, ItemId, PrimitiveOperation, PrimitiveValue, Replacements},
    stage3::structure::{self as stage3},
    stage4::structure::Environment,
};

mod after_replacing;
mod helpers;
mod var_list;

pub fn ingest(from: stage3::Environment) -> Result<Environment, String> {
    let mut env = Environment::new(from);
    let mut next_item = ItemId(0);
    while next_item.0 < env.items.len() {
        env.compute_type(next_item)?;
        next_item.0 += 1;
    }
    Ok(env)
}

impl Environment {
    pub fn op_type(&self, op: &PrimitiveOperation) -> ItemId {
        match op {
            PrimitiveOperation::I32Math(..) => self.i32_type(),
        }
    }

    pub fn compute_type(&mut self, of: ItemId) -> Result<ItemId, String> {
        assert!(of.0 < self.items.len());
        let item = &self.items[of.0];
        if let Some(typee) = item.typee {
            return Ok(typee);
        }
        let typee = match &item.base {
            Item::Defining { base, .. } => {
                let base = *base;
                self.compute_type(base)?
            }
            Item::FromType { base, .. } => {
                let base = *base;
                self.compute_type(base)?
            }
            Item::GodType { .. } => self.god_type(),
            // TODO: This is not always correct. Need to finalize how inductive
            // types can depend on vars.
            Item::InductiveType(..) => self.god_type(),
            Item::InductiveValue { typee, records, .. } => {
                let mut from_vars = VarList::new();
                let typee = *typee;
                for recorded in records.clone() {
                    let typee = self.compute_type(recorded)?;
                    let recorded_vars = self.get_from_variables(typee)?;
                    from_vars.append(&recorded_vars.into_vec()[..]);
                }
                self.with_from_vars(typee, from_vars)
            }
            Item::IsSameVariant { base, other } => {
                // The type is a boolean dependent on the variables of the two expressions.
                let base = *base;
                let other = *other;
                let btype = self.compute_type(base)?;
                let otype = self.compute_type(other)?;
                let mut from_vars = VarList::new();
                from_vars.append(&self.get_from_variables(btype)?.into_vec());
                from_vars.append(&self.get_from_variables(otype)?.into_vec());
                self.with_from_vars(self.bool_type(), from_vars)
            }
            // Type check will ensure this is identical to the other types.
            Item::Pick {
                initial_clause,
                elif_clauses,
                else_clause,
            } => {
                let initial_clause = *initial_clause;
                let elif_clauses = elif_clauses.clone();
                let else_clause = *else_clause;

                let id = initial_clause.1;
                let initial_value_type = self.compute_type(id)?;
                // What type it is after all variables are replaced.
                let base_value_type = self.after_from(initial_value_type);

                let mut vars = VarList::new();
                {
                    let typ = self.compute_type(initial_clause.0)?;
                    vars.append(&self.get_from_variables(typ)?.into_vec());
                    vars.append(&self.get_from_variables(initial_value_type)?.into_vec());
                }
                for (cond, val) in elif_clauses {
                    let typ = self.compute_type(cond)?;
                    vars.append(&self.get_from_variables(typ)?.into_vec());
                    let typ = self.compute_type(val)?;
                    vars.append(&self.get_from_variables(typ)?.into_vec());
                }
                {
                    let typ = self.compute_type(else_clause)?;
                    vars.append(&self.get_from_variables(typ)?.into_vec());
                }

                self.with_from_vars(base_value_type, vars)
            }
            Item::PrimitiveOperation(op) => {
                let mut from_vars = VarList::new();
                let typee = self.op_type(op);
                for input in op.inputs() {
                    let input_type = self.items[input.0].typee.unwrap();
                    let input_vars = self.get_from_variables(input_type)?;
                    from_vars.append(&input_vars.into_vec()[..]);
                }
                self.with_from_vars(typee, from_vars)
            }
            Item::PrimitiveType(..) => self.god_type(),
            Item::PrimitiveValue(pv) => match pv {
                PrimitiveValue::Bool(..) => self.bool_type(),
                PrimitiveValue::I32(..) => self.i32_type(),
            },
            Item::Replacing {
                base, replacements, ..
            } => {
                let base = *base;
                let replacements = replacements.clone();
                let after_reps = self.compute_type_after_replacing(base, replacements)?;
                // These are the variables that unlabeled replacements might refer to.
                let mut remaining_variables_after_reps = self.get_from_variables(after_reps)?;
                // The same as above, but a mutable reference.
                match &mut self.items[of.0].base {
                    Item::Replacing {
                        replacements,
                        unlabeled_replacements,
                        ..
                    } => {
                        for unlabeled_replacement in unlabeled_replacements.drain(..) {
                            if remaining_variables_after_reps.len() == 0 {
                                todo!("Nice error, no more variables to replace.");
                            }
                            let target = remaining_variables_after_reps.pop_front().unwrap();
                            replacements.push((target, unlabeled_replacement))
                        }
                        let replacements = replacements.clone();
                        self.compute_type_after_replacing(base, replacements)?
                    }
                    _ => unreachable!(),
                }
            }
            Item::TypeIs { exact, typee, base } => {
                if *exact {
                    *typee
                } else {
                    let base = *base;
                    self.compute_type(base)?
                }
            }
            Item::Variable { typee, selff } => {
                let base = *typee;
                let vars = vec![*selff];
                self.insert(Item::FromType { base, vars })
            }
        };
        self.set_type(of, typee);
        Ok(typee)
    }
}
