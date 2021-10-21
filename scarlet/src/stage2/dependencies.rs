use super::structure::{Environment, ItemId, VariableId};
use crate::{
    shared::{OrderedMap, OrderedSet},
    stage2::structure::Definition,
};

impl<'x> Environment<'x> {
    fn get_deps_from_def(&mut self, of: ItemId<'x>) -> OrderedSet<VariableId<'x>> {
        match self.items[of].definition.clone().unwrap() {
            Definition::BuiltinOperation(_, args) => {
                let mut base = OrderedSet::new();
                for arg in args {
                    base = base.union(self.get_deps(arg).clone());
                }
                base
            }
            Definition::BuiltinValue(..) => OrderedSet::new(),
            Definition::Match {
                base,
                conditions,
                else_value,
            } => todo!(),
            Definition::Member(base, _) => {
                // TODO: Do better than this
                self.get_deps(base).clone()
            }
            Definition::Other(..) => unreachable!(),
            Definition::Struct(fields) => {
                let mut base = OrderedSet::new();
                for field in fields {
                    base = base.union(self.get_deps(field.value).clone());
                }
                base
            }
            Definition::Substitute(base, subs) => {
                let base_deps = self.get_deps(base);
                let mut final_deps = OrderedSet::new();
                // For each dependency of the base expression...
                'deps: for (dep, _) in base_deps.clone() {
                    for sub in &subs {
                        // If there is a substitution targeting that dependency...
                        if dep == self.item_as_variable(sub.target.unwrap()) {
                            // Then push all the substituted value's dependencies.
                            final_deps = final_deps.union(self.get_deps(sub.value).clone());
                            // And don't bother pushing the original dependency.
                            continue 'deps
                        }
                    }
                    // Otherwise, if it is not replaced, the new expression is
                    // still dependant on it.
                    final_deps.insert_or_replace(dep, ());
                }
                final_deps
            },
            Definition::Variable(var) => {
                let mut base: OrderedSet<VariableId<'x>> = self.items[of]
                    .after
                    .iter()
                    .cloned()
                    .map(|x| (x, ()))
                    .collect();
                base.insert_or_replace(var, ());
                base
            }
        }
    }

    pub fn get_deps(&mut self, of: ItemId<'x>) -> &OrderedSet<VariableId<'x>> {
        if self.items[of].dependencies.is_none() {
            let deps = self.get_deps_from_def(of);
            self.items[of].dependencies = Some(deps);
        }
        self.items[of].dependencies.as_ref().unwrap()
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
