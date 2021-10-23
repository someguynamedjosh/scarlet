use super::structures::DepQueryResult;
use crate::{
    shared::OrderedSet,
    stage2::structure::{BuiltinOperation, Definition, Environment, ItemId, VariableId},
};

impl<'x> Environment<'x> {
    pub(super) fn get_deps_from_def(&mut self, of: ItemId<'x>) -> DepQueryResult<'x> {
        match self.items[of].definition.clone().unwrap() {
            Definition::BuiltinOperation(op, args) => {
                if op == BuiltinOperation::Matches {
                    todo!()
                }
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
                    deps.append(self.after_query(condition.pattern));
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
            Definition::Struct(fields) => {
                let mut base = DepQueryResult::new();
                for field in fields {
                    base.append(self.dep_query(field.value));
                }
                base
            }
            Definition::Substitute(base, subs) => {
                let base_deps = self.dep_query(base);
                let mut final_deps = DepQueryResult::empty(base_deps.partial_over.clone());
                // For each dependency of the base expression...
                'deps: for (base_dep, _) in base_deps.vars.clone() {
                    for sub in &subs {
                        // If there is a substitution targeting that dependency...
                        if base_dep == self.item_as_variable(sub.target.unwrap()) {
                            // Then push all the substituted value's dependencies.
                            final_deps.append(self.dep_query(sub.value));
                            // And don't bother pushing the original dependency.
                            continue 'deps;
                        }
                    }
                    // Otherwise, if it is not replaced, the new expression is
                    // still dependant on it.
                    final_deps.vars.insert_or_replace(base_dep, ());
                }
                final_deps
            }
            Definition::Variable(var) => {
                let pattern = self.vars[var].pattern;
                let mut afters = self.after_query(pattern);
                afters.vars.insert_or_replace(var, ());
                afters
            }
        }
    }
}
