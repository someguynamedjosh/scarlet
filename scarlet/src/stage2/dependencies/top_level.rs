use super::structures::DepQueryResult;
use crate::{
    shared::OrderedSet,
    stage2::structure::{Environment, ItemId, VariableId},
};

// let after_deps = self.dep_query(after);
// let base_deps = self.dep_query(base);

// if base_deps.partial_over.len() == 0 && after_deps.partial_over.len() == 0 {
//     for (dep, _) in &after_deps.vars {
//         if !base_deps.vars.contains_key(dep) {
//             todo!("Nice error, base {:?} is not dependent on {:?}", base,
// dep);         }
//     }
// }

// let mut result = after_deps;
// result.append(base_deps);
// result

impl<'x> Environment<'x> {
    pub(super) fn dep_query(&mut self, of: ItemId<'x>) -> DepQueryResult<'x> {
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
}
