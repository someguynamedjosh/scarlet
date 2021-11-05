mod bounding_pattern;
mod match_impl;
mod result;

pub use crate::stage2::matchh::result::MatchResult;
use crate::stage2::structure::{
    BuiltinOperation, BuiltinValue, Definition, Environment, ItemId, Substitutions, VarType,
    VariableId,
};

impl<'x> Environment<'x> {
    pub(super) fn matches(
        &mut self,
        item: ItemId<'x>,
        match_against: ItemId<'x>,
    ) -> MatchResult<'x> {
        let item_bp = self.find_bounding_pattern(item);
        let match_against_bp = self.find_bounding_pattern(match_against);
        self.matches_impl(item, item_bp, match_against_bp, &[])
    }
}
