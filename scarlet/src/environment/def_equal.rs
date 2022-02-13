use super::{sub_expr::OwnedSubExpr, Environment, UnresolvedConstructError};
use crate::{
    constructs::ConstructId,
    environment::sub_expr::SubExpr,
    shared::TripleBool,
    util::{IsomorphicKeyIndexable, Isomorphism},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IsDefEqual {
    Yes,
    NeedsHigherLimit,
    Unknowable,
    No,
}

impl IsDefEqual {
    pub fn and(over: Vec<Self>) -> Self {
        let mut default = Self::Yes;
        for b in over {
            match b {
                Self::Yes => (),
                Self::NeedsHigherLimit => {
                    if default == Self::Yes {
                        default = Self::NeedsHigherLimit
                    }
                }
                Self::Unknowable => default = Self::Unknowable,
                Self::No => return Self::No,
            }
        }
        default
    }

    pub fn or(over: Vec<Self>) -> Self {
        let mut default = Self::No;
        for b in over {
            match b {
                Self::Yes => return Self::Yes,
                Self::Unknowable => {
                    if default == Self::Yes {
                        default = Self::Unknowable
                    }
                }
                Self::NeedsHigherLimit => default = Self::NeedsHigherLimit,
                Self::No => return Self::No,
            }
        }
        default
    }
}

pub type DefEqualResult = Result<IsDefEqual, UnresolvedConstructError>;

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
            return Ok(IsDefEqual::Yes);
        }
        if limit == 0 {
            return Ok(IsDefEqual::NeedsHigherLimit);
        }
        // For now this produces no noticable performance improvements.
        // if let Some((_, result)) = self.def_equal_memo_table.iso_get(&(left, right,
        // limit)) {     return result.clone();
        // }
        let result = (|| {
            let result = self
                .get_construct_definition(left.0)?
                .dyn_clone()
                .is_def_equal(self, &left.1, right, limit);
            if result.is_ok()
                && result != Ok(IsDefEqual::NeedsHigherLimit)
                && result != Ok(IsDefEqual::Unknowable)
            {
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
