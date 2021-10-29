use super::structures::DepQueryResult;
use crate::stage2::{
    dependencies::structures::QueryResult,
    structure::{Definition, Environment, ItemId, VarType, VariableItemIds},
};

impl<'x> Environment<'x> {
    pub(super) fn get_deps_from_def(&mut self, of: ItemId<'x>) -> DepQueryResult<'x> {
        match self.items[of].definition.clone().unwrap() {
            Definition::After { base, .. } => {
                todo!()
            }
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
                    deps.append(self.dep_query(condition.pattern).after_consumption());
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
                    let subbed_dep = self.substitute(base_dep.var_item, &subs).unwrap();
                    let def = self.definition_of(subbed_dep);
                    let subbed_dep = if let &Definition::Variable {
                        var,
                        typee,
                        consume: consumable,
                    } = def
                    {
                        VariableItemIds {
                            var_item: subbed_dep,
                            var,
                            typee,
                            consume: consumable,
                        }
                    } else {
                        unreachable!()
                    };
                    // Otherwise, if it is not replaced, the new expression is
                    // still dependant on it.
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
            Definition::UnresolvedSubstitute(..) => unreachable!(),
            Definition::Variable {
                var,
                typee,
                consume: consumable,
            } => {
                let mut afters = QueryResult::new();
                match typee {
                    VarType::God | VarType::_32U | VarType::Bool => (),
                    VarType::Just(other) => {
                        afters.append(self.dep_query(other).after_consumption())
                    }
                    VarType::And(left, right) => {
                        afters.append(self.dep_query(left).after_consumption());
                        afters.append(self.dep_query(right).after_consumption());
                    }
                }
                let typee = self.reduce_var_type(typee);
                let var_item = VariableItemIds {
                    var,
                    var_item: of,
                    typee,
                    consume: consumable,
                };
                afters.deps.insert_or_replace(var_item, ());
                afters
            }
        }
    }
}
