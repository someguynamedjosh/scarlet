use crate::stage2::{
    dependencies::structures::DepQueryResult,
    structure::{Condition, Definition, Environment, ItemId, Substitutions, VariableInfo},
};

impl<'x> Environment<'x> {
    pub(super) fn deps_of_resolved_substitution(
        &mut self,
        base: ItemId<'x>,
        num_struct_unwraps: u32,
        subs: Substitutions<'x>,
    ) -> DepQueryResult<'x> {
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
                let var_item = self.substitute(dep.var_item, &subs).unwrap();
                if let &Definition::Variable { typee, var } = self.get_definition(var_item) {
                    let subbed_dep = VariableInfo {
                        eager: dep.eager,
                        typee,
                        var,
                        var_item: dep.var_item,
                    };
                    final_deps.deps.insert_or_replace(subbed_dep, ());
                } else {
                    unreachable!()
                }
            }
        }
        final_deps
    }

    pub(super) fn deps_of_set_eager(
        &mut self,
        vals: Vec<ItemId<'x>>,
        num_struct_unwraps: u32,
        base: ItemId<'x>,
        of: ItemId<'x>,
        all: bool,
        eager: bool,
    ) -> DepQueryResult<'x> {
        let mut deps_to_set = DepQueryResult::new();
        for val in vals {
            deps_to_set.append(self.dep_query(val, num_struct_unwraps));
        }
        if all {
            deps_to_set.append(self.dep_query(base, num_struct_unwraps));
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

    pub(super) fn deps_of_builtin_op(
        &mut self,
        args: Vec<ItemId<'x>>,
        num_struct_unwraps: u32,
    ) -> DepQueryResult<'x> {
        let mut result = DepQueryResult::new();
        for arg in args {
            result.append(self.dep_query(arg, num_struct_unwraps));
        }
        result
    }

    pub(super) fn deps_of_match(
        &mut self,
        base: ItemId<'x>,
        num_struct_unwraps: u32,
        conditions: Vec<Condition<'x>>,
        else_value: ItemId<'x>,
    ) -> DepQueryResult<'x> {
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
}
