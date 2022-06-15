use std::cell::Ref;

use owning_ref::OwningRef;

use super::{trim::trim_result, Equal, EqualResult, EqualSuccess};
use crate::item::{
    definitions::{
        other::DOther,
        substitution::{DSubstitution, Substitutions},
    },
    resolvable::UnresolvedItemError,
    Item, ItemPtr,
};

const TRACE: bool = true;

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

    pub fn currently_computing_equality_for_lhs(&self) -> bool {
        self.self_side == EqualityTestSide::Left
    }

    pub fn currently_computing_equality_for_rhs(&self) -> bool {
        self.self_side == EqualityTestSide::Right
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
    pub fn get_equality_left(&mut self) -> Result<Equal, UnresolvedItemError> {
        if TRACE {
            println!("{:#?} =<= {:#?}", self.lhs, self.rhs);
        }
        self.self_side = EqualityTestSide::Left;
        let lhs = self.lhs.ptr_clone();
        let lhs_borrow = lhs.borrow();
        let lhs = lhs_borrow.definition.clone_into_box();
        drop(lhs_borrow);
        let result = lhs.get_equality_using_context(self, OnlyCalledByEcc(()))?;
        let result = if let Equal::Yes(cases) = result.equal {
            let equal = if result.unique {
                Equal::Yes(cases)
            } else if self.rhs.downcast_definition::<DSubstitution>().is_some() {
                if let Ok(Equal::Yes(other_cases)) = self.get_equality_right() {
                    Equal::Yes([cases, other_cases].concat())
                } else {
                    Equal::Yes(cases)
                }
            } else {
                Equal::Yes(cases)
            };
            Ok(equal)
        } else if result.equal == Equal::Unknown {
            self.get_equality_right()
        } else {
            Ok(result.equal)
        };
        if TRACE {
            println!("{:#?}", result);
        }
        result
    }

    /// Computes equality by querying the right element whether it is equal to
    /// the left element.
    pub fn get_equality_right(&mut self) -> Result<Equal, UnresolvedItemError> {
        self.self_side = EqualityTestSide::Right;
        let rhs = self.rhs.ptr_clone();
        let rhs = rhs.borrow();
        Ok(rhs
            .definition
            .get_equality_using_context(self, OnlyCalledByEcc(()))?
            .equal)
    }
}

impl ItemPtr {
    pub fn get_equality_left(&self, other: &Self) -> Result<Equal, UnresolvedItemError> {
        Ecc::new(self.ptr_clone(), other.ptr_clone()).get_equality_left()
    }

    pub fn get_equality_right(&self, other: &Self) -> Result<Equal, UnresolvedItemError> {
        Ecc::new(self.ptr_clone(), other.ptr_clone()).get_equality_right()
    }

    /// Gets equality and trims away redundant substitutions like x -> x,
    /// y -> x(x IS y), fx -> fx(x IS y)(y IS x) and so on.
    pub fn get_trimmed_equality(&self, other: &Self) -> Result<Equal, UnresolvedItemError> {
        let mut result = self.get_equality_left(other)?;
        trim_result(&mut result)?;
        Ok(result)
    }
}
