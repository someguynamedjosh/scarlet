use std::cell::Ref;

use owning_ref::OwningRef;

use super::{Equal, EqualResult};
use crate::item::{
    definitions::{
        other::DOther,
        substitution::{DSubstitution, Substitutions},
    },
    Item, ItemPtr,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EqualityTestSide {
    Left,
    Right,
}

pub struct EqualityCalculationContext {
    lhs: ItemPtr,
    rhs: ItemPtr,
    self_side: EqualityTestSide,
}

pub type Ecc = EqualityCalculationContext;

/// Using this in a function signature guarantees that only
/// EqualityCalculationContext can call that function. If you are reusing this
/// inside the function that is being called, you are doing something wrong.
pub struct OnlyCalledByEcc(pub(super) ());

impl EqualityCalculationContext {
    fn new(lhs: ItemPtr, rhs: ItemPtr) -> Self {
        Self {
            lhs,
            rhs,
            self_side: EqualityTestSide::Left,
        }
    }

    pub fn primary(&self) -> &ItemPtr {
        match self.self_side {
            EqualityTestSide::Left => self.lhs(),
            EqualityTestSide::Right => self.rhs(),
        }
    }

    pub fn other(&self) -> &ItemPtr {
        match self.self_side {
            EqualityTestSide::Left => self.rhs(),
            EqualityTestSide::Right => self.lhs(),
        }
    }

    pub fn lhs(&self) -> &ItemPtr {
        &self.lhs
    }

    pub fn rhs(&self) -> &ItemPtr {
        &self.rhs
    }

    pub fn with_primary(&self, new_primary: ItemPtr) -> Self {
        self.with_primary_and_other(new_primary, self.other().ptr_clone())
    }

    pub fn with_primary_and_other(&self, new_primary: ItemPtr, new_other: ItemPtr) -> Self {
        let (lhs, rhs) = match self.self_side {
            EqualityTestSide::Left => (new_primary, new_other),
            EqualityTestSide::Right => (new_other, new_primary),
        };
        Self::new(lhs, rhs)
    }

    /// Computes equality by querying the left element whether it is equal to
    /// the right element. Tries get_equality_right as a backup if that does not
    /// produce a conclusive answer.
    pub fn get_equality_left(&mut self) -> EqualResult {
        self.self_side = EqualityTestSide::Left;
        let lhs = self.lhs.ptr_clone();
        let lhs = lhs.borrow();
        let result = lhs
            .definition
            .get_equality_using_context(self, OnlyCalledByEcc(()))?;
        if result == Equal::Unknown {
            return self.get_equality_right();
        }
        Ok(result)
    }

    /// Computes equality by querying the right element whether it is equal to
    /// the left element.
    pub fn get_equality_right(&mut self) -> EqualResult {
        self.self_side = EqualityTestSide::Right;
        let rhs = self.rhs.ptr_clone();
        let rhs = rhs.borrow();
        rhs.definition
            .get_equality_using_context(self, OnlyCalledByEcc(()))
    }
}

impl ItemPtr {
    pub fn get_equality_left(&self, other: &Self) -> EqualResult {
        Ecc::new(self.ptr_clone(), other.ptr_clone()).get_equality_left()
    }

    pub fn get_equality_right(&self, other: &Self) -> EqualResult {
        Ecc::new(self.ptr_clone(), other.ptr_clone()).get_equality_right()
    }

    /// Gets equality and trims away redundant substitutions like x -> x,
    /// y -> x(x IS y), fx -> fx(x IS y)(y IS x) and so on.
    pub fn get_trimmed_equality(&self, other: &Self) -> EqualResult {
        let result = self.get_equality_left(other)?;
        todo!()
    }
}
