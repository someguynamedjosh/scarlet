mod bounding_pattern;
mod match_impl;
mod result;

pub use crate::stage2::matchh::result::MatchResult;
use crate::stage2::structure::{Environment, ItemId};

impl<'x> Environment<'x> {
    pub(super) fn matches(
        &mut self,
        item: ItemId<'x>,
        match_against: ItemId<'x>,
    ) -> MatchResult<'x> {
        let item = self.reduce(item);
        let match_against = self.reduce(match_against);
        self.matches_impl(item, item, match_against, &[])
    }
}
