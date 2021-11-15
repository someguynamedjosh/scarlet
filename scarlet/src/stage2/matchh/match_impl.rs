mod pattern_connectives;
mod values_and_variables;

use MatchResult::*;

use crate::stage2::{
    matchh::result::MatchResult,
    structure::{Environment, ConstructId, VarType, VariableId},
};

impl<'x> Environment<'x> {
    pub(super) fn matches_impl(
        &mut self,
        original_value: ConstructId<'x>,
        value: ConstructId<'x>,
        pattern: ConstructId<'x>,
        eager_vars: &[VariableId<'x>],
    ) -> MatchResult<'x> {
        let value_def = self.get_resolved_definition(value).clone();
        let pattern_def = self.get_resolved_definition(pattern).clone();

        todo!()
    }
}
