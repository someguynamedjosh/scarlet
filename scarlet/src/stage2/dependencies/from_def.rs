use super::structures::DepQueryResult;
use crate::stage2::structure::{Definition, Environment, ItemId, VarType, VariableInfo};

impl<'x> Environment<'x> {
    pub(super) fn get_deps_from_def(
        &mut self,
        of: ItemId<'x>,
        num_struct_unwraps: u32,
    ) -> DepQueryResult<'x> {
        match self.get_definition(of).clone() {
            Definition::BuiltinOperation(_, args) => {
                let mut result = DepQueryResult::new();
                for arg in args {
                    result.append(self.dep_query(arg, num_struct_unwraps));
                }
                result
            }
            Definition::BuiltinValue(_) => DepQueryResult::new(),
            Definition::Match {
                base,
                conditions,
                else_value,
            } => {
                let mut result = self.dep_query(base, num_struct_unwraps);
                for condition in conditions {
                    result.append(
                        self.dep_query(condition.pattern, num_struct_unwraps)
                            .discarding_shy(),
                    );
                    result.append(self.dep_query(condition.value, num_struct_unwraps));
                }
                result.append(self.dep_query(else_value, num_struct_unwraps));
                result
            }
            Definition::Member(base, _) => self.dep_query(base, num_struct_unwraps + 1),
            Definition::Other(item) => self.dep_query(item, num_struct_unwraps),
            Definition::SetEager { base, vals, eager } => {
                let mut deps_to_set = DepQueryResult::new();
                for val in vals {
                    deps_to_set.append(self.dep_query(val, num_struct_unwraps));
                }
                let mut result = self.dep_query(base, num_struct_unwraps);
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
                    if num_struct_unwraps == 0 {
                        query.append(self.dep_query(field.value, 0).discarding_shy());
                    } else {
                        query.append(self.dep_query(field.value, num_struct_unwraps - 1));
                    }
                }
                query
            }
            Definition::UnresolvedSubstitute(..) => {
                self.resolve_substitution(of);
                self.get_deps_from_def(of, num_struct_unwraps)
            }
            Definition::ResolvedSubstitute(base, subs) => {
                let base_deps = self.dep_query(base, num_struct_unwraps);
                let mut final_deps = DepQueryResult::empty(base_deps.partial_over.clone());
                for (dep, _) in base_deps.deps {
                    if let Some(&value) = subs.get(&dep.var) {
                        let value_deps = self.dep_query(value, num_struct_unwraps);
                        if dep.eager {
                            final_deps.append(value_deps.all_eager());
                        } else {
                            final_deps.append(value_deps);
                        }
                    } else {
                        final_deps.deps.insert_or_replace(dep, ());
                    }
                }
                final_deps
            }
            Definition::Variable { var, typee } => {
                let mut result = self
                    .deps_of_var_typ(typee, num_struct_unwraps)
                    .discarding_shy();
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

    fn deps_of_var_typ(
        &mut self,
        typee: VarType<'x>,
        num_struct_unwraps: u32,
    ) -> DepQueryResult<'x> {
        match typee {
            VarType::God | VarType::_32U | VarType::Bool => DepQueryResult::new(),
            VarType::Just(other) => self.dep_query(other, num_struct_unwraps),
            VarType::And(left, right) | VarType::Or(left, right) => {
                let mut result = self.dep_query(left, num_struct_unwraps);
                result.append(self.dep_query(right, num_struct_unwraps));
                result
            }
        }
    }
}
