use std::thread::current;

use crate::{
    shared::{IntegerMathOperation, Item, ItemId, PrimitiveOperation, Replacements},
    stage4::{ingest::VarList, structure::Environment},
    util::*,
};

impl Environment {
    pub fn compute_dependencies(
        &mut self,
        item: ItemId,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<VarList, String> {
        match &self.items[item.0].definition {
            Item::Defining { base, .. } => {
                let base = *base;
                self.compute_dependencies(base, currently_computing)
            }
            Item::FromType { base, vars } => {
                let base = *base;
                let values = vars.clone();
                let mut res = VarList::new();
                for value in values {
                    let value_deps = self.compute_dependencies(value, currently_computing.clone())?;
                    res.append(value_deps.as_slice());
                }
                let base_deps = self.compute_dependencies(base, currently_computing.clone())?;
                res.append(base_deps.as_slice());
                MOk(res)
            }
            Item::GodType | Item::PrimitiveType(..) | Item::PrimitiveValue(..) => {
                MOk(VarList::new())
            }
            Item::InductiveValue { typee, params, .. } => {
                let typee = *typee;
                let values = params.clone();
                let mut res = self.compute_dependencies(typee, currently_computing.clone())?;
                for value in values {
                    let value_deps = self.compute_dependencies(value, currently_computing.clone())?;
                    res.append(value_deps.as_slice());
                }
                MOk(res)
            }
            Item::IsSameVariant { base, other } => {
                let base = *base;
                let other = *other;
                let mut res = self.compute_dependencies(base, currently_computing.clone())?;
                let other_deps = self.compute_dependencies(other, currently_computing.clone())?;
                res.append(other_deps.as_slice());
                MOk(res)
            }
            Item::Pick {
                initial_clause,
                elif_clauses,
                else_clause,
            } => {
                todo!()
            }
            Item::PrimitiveOperation(op) => match op {
                PrimitiveOperation::I32Math(op) => match op {
                    IntegerMathOperation::Sum(a, b) | IntegerMathOperation::Difference(a, b) => {
                        let a = *a;
                        let b = *b;
                        let mut res = self.compute_dependencies(a, currently_computing.clone())?;
                        let other_deps = self.compute_dependencies(b, currently_computing.clone())?;
                        res.append(other_deps.as_slice());
                        MOk(res)
                    }
                },
            },
            Item::Replacing { base, replacements, unlabeled_replacements } => {
                assert!(unlabeled_replacements.len() == 0, "TODO: Better unlabeled replacements");
                let base = *base;
                let replacements = replacements.clone();
                let base_deps = self.compute_dependencies(base, currently_computing.clone())?;
                let mut result = VarList::new();
                for dep in base_deps.into_vec()  {
                    if let Some(rep) = replacements.iter().find(|r| r.0 == dep && r.0 != r.1) {
                        let replaced_with = rep.1;
                        let replaced_deps = self.compute_dependencies(replaced_with, currently_computing.clone())?;
                        result.append(replaced_deps.as_slice())
                    } else {
                        result.push(dep);
                    }
                }
                MOk(result)
            }
            Item::TypeIs { base,.. } => {
                let base = *base;
                self.compute_dependencies(base, currently_computing)
            }
            Item::Variable { selff, typee } => {
                let selff = *selff;
                let typee = *typee;
                let mut res = self.compute_dependencies(typee, currently_computing)?;
                res.push(selff);
                MOk(res)
            }
        }
    }

    pub fn type_with_dependencies(
        &mut self,
        mut base_type: ItemId,
        mut dependencies: VarList,
        defined_in: Option<ItemId>,
    ) -> ItemId {
        if dependencies.len() > 0 {
            if let Item::FromType {
                base: other_base,
                vars: other_vars,
            } = &self.items[base_type.0].definition
            {
                base_type = *other_base;
                let mut all_vars = VarList::from(other_vars.clone());
                all_vars.append(dependencies.as_slice());
                dependencies = all_vars;
            }
            self.insert(
                Item::FromType {
                    base: base_type,
                    vars: dependencies.into_vec(),
                },
                defined_in,
            )
        } else {
            base_type
        }
    }
}
