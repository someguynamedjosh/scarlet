use super::structures::DepQueryResult;
use crate::stage2::structure::{Definition, Environment, ItemId, Pattern, VariableInfo};

impl<'x> Environment<'x> {
    pub(super) fn get_deps_from_def(&mut self, of: ItemId<'x>) -> DepQueryResult<'x> {
        match self.items[of].definition.clone().unwrap() {
            Definition::BuiltinOperation(_, args) => {
                let mut base = DepQueryResult::new();
                for arg in args {
                    base.append(self.dep_query(arg));
                }
                base
            }
            Definition::BuiltinValue(..) => DepQueryResult::new(),
            Definition::Match {
                base,
                conditions,
                else_value,
            } => {
                let mut deps = self.dep_query(base);
                for condition in conditions {
                    deps.append(self.dep_query(condition.pattern));
                    deps.append(self.dep_query(condition.value));
                }
                deps.append(self.dep_query(else_value));
                deps
            }
            Definition::Member(base, _) => {
                // TODO: Do better than this
                self.dep_query(base)
            }
            Definition::Other(item) => self.dep_query(item),
            Definition::Pattern(pat) => match pat {
                Pattern::God
                | Pattern::Pattern
                | Pattern::_32U
                | Pattern::Bool
                | Pattern::Capture(..) => DepQueryResult::new(),
                Pattern::And(left, right) => {
                    let mut result = self.dep_query(left);
                    result.append(self.dep_query(right));
                    result
                }
            },
            Definition::ResolvedSubstitute(base, subs) => {
                let base_deps = self.dep_query(base);
                let mut final_deps = DepQueryResult::empty(base_deps.partial_over.clone());
                // For each dependency of the base expression...
                'deps: for (base_dep, _) in base_deps.deps.clone() {
                    for sub in &subs {
                        // If there is a substitution targeting that dependency...
                        if sub.0 == base_dep.var {
                            // Then push all the substituted value's dependencies.
                            final_deps.append(self.dep_query(sub.1));
                            // And don't bother pushing the original dependency.
                            continue 'deps;
                        }
                    }
                    // Otherwise, if it is not replaced, the new expression is
                    // still dependant on it.
                    let subbed_dep = self.substitute(base_dep.var_item, &subs).unwrap();
                    let def = self.definition_of(subbed_dep);
                    let subbed_dep = if let &Definition::Variable {
                        var,
                        pattern: typee,
                    } = def
                    {
                        VariableInfo {
                            var_item: subbed_dep,
                            var,
                            pattern: typee,
                        }
                    } else {
                        unreachable!()
                    };
                    final_deps.deps.insert_or_replace(subbed_dep, ());
                }
                final_deps
            }
            Definition::Struct(fields) => {
                let mut base = DepQueryResult::new();
                for field in fields {
                    base.append(self.dep_query(field.value));
                }
                base
            }
            Definition::UnresolvedSubstitute(..) => {
                self.resolve_targets_in_item(of);
                self.get_deps_from_def(of)
            }
            Definition::Variable { var, pattern } => {
                let pattern = self.reduce(pattern);
                let var_item = VariableInfo {
                    var,
                    var_item: of,
                    pattern,
                };
                let mut result = self.dep_query(pattern);
                result.deps.insert_or_replace(var_item, ());
                result
            }
        }
    }
}
