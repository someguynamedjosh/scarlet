use super::{sub_expr::OwnedSubExpr, Environment, UnresolvedConstructError};
use crate::{
    constructs::ConstructId,
    environment::sub_expr::SubExpr,
    shared::TripleBool,
    util::{IsomorphicKeyIndexable, Isomorphism},
};

pub type DefEqualResult = Result<TripleBool, UnresolvedConstructError>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DefEqualQuery {
    left: OwnedSubExpr,
    right: OwnedSubExpr,
    limit: u32,
}

impl<'x> Isomorphism<DefEqualQuery> for (SubExpr<'x>, SubExpr<'x>, u32) {
    fn convert(self) -> DefEqualQuery {
        DefEqualQuery {
            left: self.0.to_owned(),
            right: self.1.to_owned(),
            limit: self.2,
        }
    }

    fn equals(&self, other: &DefEqualQuery) -> bool {
        self.0.equals(&other.left) && self.1.equals(&other.right) && self.2 == other.limit
    }
}

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
        // For now this produces no noticable performance improvements.
        // if let Some((_, result)) = self.def_equal_memo_table.iso_get(&(left, right, limit)) {
        //     return result.clone();
        // }
        let result = (|| {
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
        })();
        // self.def_equal_memo_table
        //     .insert((left, right, limit).convert(), result.clone());
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
