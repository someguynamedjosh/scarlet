use super::structure::{Environment, ItemId, Variable, VariableId};
use crate::{
    shared::{OrderedMap, OrderedSet},
    stage2::structure::Definition,
};

#[derive(Debug)]
struct DepQueryResult<'x> {
    vars: OrderedSet<VariableId<'x>>,
    partial_over: OrderedSet<ItemId<'x>>,
}

impl<'x> DepQueryResult<'x> {
    pub fn new() -> Self {
        Self::empty(OrderedSet::new())
    }

    pub fn empty(partial_over: OrderedSet<ItemId<'x>>) -> Self {
        Self {
            vars: Default::default(),
            partial_over,
        }
    }

    pub fn full(vars: OrderedSet<VariableId<'x>>) -> Self {
        Self {
            vars,
            partial_over: OrderedSet::new(),
        }
    }

    pub fn append(&mut self, other: Self) {
        let sv = std::mem::take(&mut self.vars);
        self.vars = sv.union(other.vars);
        let spo = std::mem::take(&mut self.partial_over);
        self.partial_over = spo.union(other.partial_over);
    }

    pub fn remove_partial(&mut self, over: ItemId<'x>) {
        self.partial_over.remove(&over);
    }
}

impl<'x> Environment<'x> {
    fn get_deps_from_def(&mut self, of: ItemId<'x>) -> DepQueryResult<'x> {
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
                    deps.vars = deps.vars.union(self.items[condition.pattern].after.clone());
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
                let mut base: OrderedSet<VariableId<'x>> = self.items[of].after.clone();
                base.insert_or_replace(var, ());
                DepQueryResult::full(base)
            }
        }
    }

    fn dep_query(&mut self, of: ItemId<'x>) -> DepQueryResult<'x> {
        if self.items[of].dependencies.is_none() {
            if self.query_stack_contains(of) {
                return DepQueryResult::empty(vec![(of, ())].into_iter().collect());
            } else {
                self.with_query_stack_frame(of, |this| {
                    let mut deps = this.get_deps_from_def(of);
                    deps.remove_partial(of);
                    if deps.partial_over.is_empty() {
                        this.items[of].dependencies = Some(deps.vars.clone());
                    }
                    deps
                })
            }
        } else {
            let deps = self.items[of].dependencies.as_ref().unwrap().clone();
            DepQueryResult::full(deps)
        }
    }

    pub fn get_deps(&mut self, of: ItemId<'x>) -> OrderedSet<VariableId<'x>> {
        let result = self.with_fresh_query_stack(|this| this.dep_query(of));
        assert!(result.partial_over.is_empty());
        result.vars
    }

    pub fn find_all_dependencies(&mut self) {
        let id = if let Some((id, _)) = self.items.iter().next() {
            id
        } else {
            return;
        };
        self.get_deps(id);
        let mut id = id;
        while let Some(next) = self.items.next(id) {
            id = next;
            self.get_deps(id);
        }
    }
}
