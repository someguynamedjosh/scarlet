use super::structures::DepQueryResult;
use crate::{
    shared::OrderedSet,
    stage2::structure::{Environment, ConstructId, VariableInfo},
};

impl<'x> Environment<'x> {
    pub fn dep_query(
        &mut self,
        of: ConstructId<'x>,
        num_struct_unwraps: u32,
    ) -> DepQueryResult<'x> {
        if self.items[of].dependencies.is_none() || num_struct_unwraps != 0 {
            if self.query_stack_contains(of) {
                return DepQueryResult::empty(vec![(of, ())].into_iter().collect());
            } else {
                self.with_query_stack_frame(of, |this| {
                    this.compute_deps_from_scratch(of, num_struct_unwraps)
                })
            }
        } else {
            let deps = self.items[of].dependencies.as_ref().unwrap().clone();
            DepQueryResult::full(deps)
        }
    }

    pub fn get_deps(&mut self, of: ConstructId<'x>) -> OrderedSet<VariableInfo<'x>> {
        let result = self.with_fresh_query_stack(|this| this.dep_query(of, 0));
        assert!(result.partial_over.is_empty());
        result.deps
    }

    fn compute_deps_from_scratch(
        &mut self,
        of: ConstructId<'x>,
        num_struct_unwraps: u32,
    ) -> DepQueryResult<'x> {
        let mut deps = self.get_deps_from_def(of, num_struct_unwraps);
        deps.remove_partial(of);
        if deps.partial_over.is_empty() && num_struct_unwraps == 0 {
            self.items[of].dependencies = Some(deps.deps.clone());
        }
        deps
    }
}
