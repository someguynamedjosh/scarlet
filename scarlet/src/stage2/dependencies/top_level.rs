use super::structures::DepQueryResult;
use crate::{
    shared::OrderedSet,
    stage2::structure::{Environment, ItemId, VariableInfo},
};

impl<'x> Environment<'x> {
    pub(super) fn dep_query(&mut self, of: ItemId<'x>) -> DepQueryResult<'x> {
        if self.items[of].dependencies.is_none() {
            if self.query_stack_contains(of) {
                return DepQueryResult::empty(vec![(of, ())].into_iter().collect());
            } else {
                self.with_query_stack_frame(of, |this| this.compute_deps_from_scratch(of))
            }
        } else {
            let deps = self.items[of].dependencies.as_ref().unwrap().clone();
            DepQueryResult::full(deps)
        }
    }

    pub fn get_deps(&mut self, of: ItemId<'x>) -> OrderedSet<VariableInfo<'x>> {
        let result = self.with_fresh_query_stack(|this| this.dep_query(of));
        assert!(result.partial_over.is_empty());
        result.deps
    }

    fn compute_deps_from_scratch(&mut self, of: ItemId<'x>) -> DepQueryResult<'x> {
        let mut deps = self.get_deps_from_def(of);
        deps.remove_partial(of);
        if deps.partial_over.is_empty() {
            self.items[of].dependencies = Some(deps.deps.clone());
        }
        deps
    }
}