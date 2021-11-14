mod bounding_pattern;
mod match_impl;
mod result;

pub use crate::stage2::matchh::result::MatchResult;
use crate::stage2::structure::{Environment, ConstructId};

use super::construct::BasicVarType;

impl<'x> Environment<'x> {
    pub(super) fn matches(
        &mut self,
        item: ConstructId<'x>,
        match_against: ConstructId<'x>,
    ) -> MatchResult<'x> {
        // self.matches_impl(item, item, match_against, &[])
        todo!()
    }
}
