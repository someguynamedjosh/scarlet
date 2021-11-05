use crate::stage2::{
    matchh::result::MatchResult,
    structure::{Environment, ItemId, VariableId},
};

impl<'x> Environment<'x> {
    pub(super) fn on_right_and(
        &mut self,
        original_value: ItemId<'x>,
        value: ItemId<'x>,
        left: ItemId<'x>,
        eager_vars: &[VariableId<'x>],
        right: ItemId<'x>,
        var: VariableId<'x>,
    ) -> MatchResult<'x> {
        let left = self.matches_impl(original_value, value, left, eager_vars);
        let right = self.matches_impl(original_value, value, right, eager_vars);
        MatchResult::and(vec![left, right]).with_sub_if_match(var, original_value)
    }

    pub(super) fn on_left_or(
        &mut self,
        original_value: ItemId<'x>,
        left: ItemId<'x>,
        pattern: ItemId<'x>,
        eager_vars: &[VariableId<'x>],
        right: ItemId<'x>,
        var: VariableId<'x>,
    ) -> MatchResult<'x> {
        let left = self.matches_impl(original_value, left, pattern, eager_vars);
        let right = self.matches_impl(original_value, right, pattern, eager_vars);
        MatchResult::and(vec![left, right]).with_sub_if_match(var, original_value)
    }

    pub(super) fn on_right_or(
        &mut self,
        original_value: ItemId<'x>,
        value: ItemId<'x>,
        left: ItemId<'x>,
        eager_vars: &[VariableId<'x>],
        right: ItemId<'x>,
        var: VariableId<'x>,
    ) -> MatchResult<'x> {
        let left = self.matches_impl(original_value, value, left, eager_vars);
        let right = self.matches_impl(original_value, value, right, eager_vars);
        MatchResult::or(vec![left, right]).with_sub_if_match(var, original_value)
    }

    pub(super) fn on_left_and(
        &mut self,
        original_value: ItemId<'x>,
        left: ItemId<'x>,
        pattern: ItemId<'x>,
        eager_vars: &[VariableId<'x>],
        right: ItemId<'x>,
        var: VariableId<'x>,
    ) -> MatchResult<'x> {
        let left = self.matches_impl(original_value, left, pattern, eager_vars);
        let right = self.matches_impl(original_value, right, pattern, eager_vars);
        MatchResult::or(vec![left, right]).with_sub_if_match(var, original_value)
    }
}
