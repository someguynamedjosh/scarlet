use crate::{
    stage2::structure::{ItemId, PrimitiveOperation, PrimitiveValue, Replacements},
    stage3::structure::{self as stage3, Item},
    stage4::structure::Environment,
};
use std::collections::HashMap;

pub fn ingest(from: stage3::Environment) -> Result<Environment, String> {
    let mut env = Environment::new(from);
    let mut next_item = ItemId(0);
    while next_item.0 < env.items.len() {
        env.compute_type(next_item)?;
        next_item.0 += 1;
    }
    Ok(env)
}

struct VarList(Vec<ItemId>);

impl VarList {
    pub fn new() -> VarList {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, item: ItemId) {
        if !self.0.contains(&item) {
            self.0.push(item)
        }
    }

    pub fn append(&mut self, items: &[ItemId]) {
        for item in items {
            self.push(*item);
        }
    }

    pub fn into_vec(self) -> Vec<ItemId> {
        self.0
    }
}

impl Environment {
    /// Returns the type of an item after applying the given replacements.
    /// E.G. a + b with replacements a: c should yield Int From{b c}
    fn compute_type_after_replacing(
        &mut self,
        base: ItemId,
        replacements: Replacements,
    ) -> Result<ItemId, String> {
        let unreplaced_type = self.compute_type(base)?;
        // A hashmap of variables to replace and what variables the replaced values depend on.
        let mut replacement_data = HashMap::<ItemId, VarList>::new();
        for (target, value) in replacements {
            let valtype = self.compute_type(value)?;
            let valtype_vars = self.get_from_variables(valtype)?;
            replacement_data.insert(target, valtype_vars);
        }
        // TODO: This doesn't work when replacing a variable with more variables. I think?
        let def = &self.items[unreplaced_type.0].base;
        let res = match def {
            Item::FromType { base, vars } => {
                let mut vars_after_reps = VarList::new();
                for var in vars {
                    if let Some(replaced_value_vars) = replacement_data.get(var) {
                        // $var is being replaced with a value that depends on replaced_value_vars.
                        vars_after_reps.append(&replaced_value_vars.0[..])
                    } else {
                        // $var is not being replaced so the expression still depends on it.
                        vars_after_reps.push(*var);
                    }
                }
                if vars_after_reps.len() == 0 {
                    *base
                } else if &vars_after_reps.0 == vars {
                    unreplaced_type
                } else {
                    let base = *base;
                    self.insert(Item::FromType {
                        base,
                        vars: vars_after_reps.into_vec(),
                    })
                }
            }
            _ => unreplaced_type,
        };
        Ok(res)
    }

    // Collects all variables specified by From items pointed to by the provided ID.
    fn get_from_variables(&mut self, typee: ItemId) -> Result<VarList, String> {
        Ok(match &self.items[typee.0].base {
            Item::Defining { base: id, .. } => {
                let id = *id;
                self.get_from_variables(id)?
            }
            Item::FromType { base, vars } => {
                let base = *base;
                let vars = vars.clone();
                let mut result = self.get_from_variables(base)?;
                result.append(&vars[..]);
                result
            }
            Item::Replacing { .. } => todo!(),
            _ => VarList::new(),
        })
    }

    fn with_from_vars(&mut self, base: ItemId, from_vars: VarList) -> ItemId {
        if from_vars.len() > 0 {
            self.insert(Item::FromType {
                base,
                vars: from_vars.into_vec(),
            })
        } else {
            base
        }
    }

    pub(super) fn op_type(&self, op: &PrimitiveOperation) -> ItemId {
        match op {
            PrimitiveOperation::I32Math(..) => self.i32_type(),
        }
    }

    pub(super) fn compute_type(&mut self, of: ItemId) -> Result<ItemId, String> {
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
            Item::Pick { initial_clause, elif_clauses, else_clause } => {
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
                            let target = remaining_variables_after_reps.0.remove(0);
                            replacements.push((target, unlabeled_replacement))
                        }
                        let replacements = replacements.clone();
                        self.compute_type_after_replacing(base, replacements)?
                    }
                    _ => unreachable!(),
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
