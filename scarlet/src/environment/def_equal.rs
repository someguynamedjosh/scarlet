use super::{Environment, UnresolvedConstructError};
use crate::{
    constructs::{substitution::SubExpr, ConstructId},
    shared::TripleBool,
};

pub type DefEqualResult = Result<TripleBool, UnresolvedConstructError>;

impl<'x> Environment<'x> {
    pub(crate) fn is_def_equal(
        &mut self,
        left: SubExpr,
        right: SubExpr,
        limit: u32,
    ) -> DefEqualResult {
        if left == right {
            return Ok(TripleBool::True);
        }
        if limit == 0 {
            return Ok(TripleBool::Unknown);
        }
        let result = self
            .get_construct_definition(left.0)?
            .dyn_clone()
            .is_def_equal(self, &left.1, right, limit);
        if result.is_ok() && result != Ok(TripleBool::Unknown) {
            return result;
        }
        let result = self
            .get_construct_definition(right.0)?
            .dyn_clone()
            .is_def_equal(self, &right.1, left, limit);
        result
    }

    pub(crate) fn is_def_equal_without_subs(
        &mut self,
        left: ConstructId,
        right: ConstructId,
        limit: u32,
    ) -> DefEqualResult {
        self.is_def_equal(
            SubExpr(left, &Default::default()),
            SubExpr(right, &Default::default()),
            limit,
        )
    }
}
