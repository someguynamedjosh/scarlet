use super::structures::{DepQueryResult, QueryResult};
use crate::{
    shared::OrderedSet,
    stage2::structure::{After, Environment, ItemId, VariableId, VariableItemIds},
};

impl<'x> Environment<'x> {
    fn compute_afters(&mut self, of: ItemId<'x>) -> DepQueryResult<'x> {
        let mut afters = self.get_afters_from_def(of);

        let deps = self.dep_query(of);
        if deps.partial_over.len() == 0 {
            for (var, _) in &afters.deps {
                if !deps.deps.contains_key(var) {
                    todo!("Nice error, {:?} is not dependent on {:?}", of, var);
                }
            }
        }

        afters.remove_partial(of);
        if afters.partial_over.is_empty() {
            self.items[of].after = Some(afters.deps.clone());
        }
        afters
    }

    pub(super) fn after_query(&mut self, of: ItemId<'x>) -> DepQueryResult<'x> {
        match &self.items[of].after {
            None => {
                if self.query_stack_contains(of) {
                    DepQueryResult::empty(vec![(of, ())].into_iter().collect())
                } else {
                    self.with_query_stack_frame(of, |this| this.compute_afters(of))
                }
            }
            Some(vars) => DepQueryResult::full(vars.clone()),
        }
    }

    pub fn get_afters(&mut self, of: ItemId<'x>) -> OrderedSet<VariableItemIds<'x>> {
        let result = self.with_fresh_query_stack(|this| this.after_query(of));
        assert!(result.partial_over.is_empty());
        result.deps
    }

    pub(super) fn dep_query(&mut self, of: ItemId<'x>) -> DepQueryResult<'x> {
        if self.items[of].dependencies.is_none() {
            if self.query_stack_contains(of) {
                return DepQueryResult::empty(vec![(of, ())].into_iter().collect());
            } else {
                self.with_query_stack_frame(of, |this| {
                    let mut deps = this.get_deps_from_def(of);
                    deps.remove_partial(of);
                    if deps.partial_over.is_empty() {
                        this.items[of].dependencies = Some(deps.deps.clone());
                    }
                    deps
                })
            }
        } else {
            let deps = self.items[of].dependencies.as_ref().unwrap().clone();
            DepQueryResult::full(deps)
        }
    }

    pub fn get_deps(&mut self, of: ItemId<'x>) -> OrderedSet<VariableItemIds<'x>> {
        let result = self.with_fresh_query_stack(|this| this.dep_query(of));
        assert!(result.partial_over.is_empty());
        result.deps
    }
}
