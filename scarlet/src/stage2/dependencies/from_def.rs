use itertools::Itertools;

use super::structures::DepQueryResult;
use crate::{
    shared::OrderedSet,
    stage2::structure::{Definition, Environment, ItemId, VarType, VariableId, VariableInfo},
};

impl<'x> Environment<'x> {
    pub(super) fn get_deps_from_def(&mut self, of: ItemId<'x>) -> DepQueryResult<'x> {
        match self.get_definition(of).clone() {
            Definition::BuiltinOperation(_, args) => {
                let mut result = DepQueryResult::new();
                for arg in args {
                    result.append(self.dep_query(arg));
                }
                result
            }
            Definition::BuiltinValue(_) => DepQueryResult::new(),
            Definition::Match {
                base,
                conditions,
                else_value,
            } => todo!(),
            Definition::Member(_, _) => todo!(),
            Definition::Other(item) => self.dep_query(item),
            Definition::SetEager { base, vals, eager } => {
                let mut deps_to_set = DepQueryResult::new();
                for val in vals {
                    deps_to_set.append(self.dep_query(val));
                }
                let mut result = self.dep_query(base);
                if deps_to_set.partial_over.contains_key(&of) {
                    deps_to_set.append(result.clone());
                }
                result.partial_over = result.partial_over.union(deps_to_set.partial_over);
                for (set_this, _) in deps_to_set.deps {
                    for (target, _) in &mut result.deps {
                        if target.var == set_this.var {
                            target.eager = eager;
                        }
                    }
                }
                result
            }
            Definition::Struct(fields) => {
                let mut query = DepQueryResult::new();
                for field in fields {
                    query.append(self.dep_query(field.value).discarding_shy());
                }
                query
            }
            Definition::UnresolvedSubstitute(..) => {
                self.resolve_substitution(of);
                self.get_deps_from_def(of)
            }
            Definition::ResolvedSubstitute(base, subs) => {
                let base_deps = self.dep_query(base);
                let mut final_deps = DepQueryResult::empty(base_deps.partial_over.clone());
                for (dep, _) in base_deps.deps {
                    if let Some(&value) = subs.get(&dep.var) {
                        final_deps.append(self.dep_query(value));
                    } else {
                        final_deps.deps.insert_or_replace(dep, ());
                    }
                }
                final_deps
            },
            Definition::Variable { var, typee } => {
                let mut result = self.deps_of_var_typ(typee).discarding_shy();
                let this = VariableInfo {
                    var_item: of,
                    var,
                    typee,
                    eager: false,
                };
                result.deps.insert_or_replace(this, ());
                result
            }
        }
    }

    fn deps_of_var_typ(&mut self, typee: VarType<'x>) -> DepQueryResult<'x> {
        match typee {
            VarType::God | VarType::_32U | VarType::Bool => DepQueryResult::new(),
            VarType::Just(other) => self.dep_query(other),
            VarType::And(left, right) => {
                let mut result = self.dep_query(left);
                result.append(self.dep_query(right));
                result
            }
        }
    }
}
