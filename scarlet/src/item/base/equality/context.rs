use std::collections::HashSet;

use super::{trim::trim_result, Equal};
use crate::{
    item::{
        definitions::{
            substitution::{DSubstitution, Substitutions},
            variable::VariablePtr,
        },
        dependencies::Dependencies,
        resolvable::UnresolvedItemError,
        util::unchecked_substitution,
        ItemPtr,
    },
    util::PtrExtension,
};

const TRACE: bool = false;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EqualityTestSide {
    Left,
    Right,
}

#[derive(Debug)]
pub struct EqualityCalculationContext {
    lhs: ItemPtr,
    lhs_subs: Vec<Substitutions>,
    rhs: ItemPtr,
    rhs_subs: Vec<Substitutions>,
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
            lhs: lhs.dereference(),
            lhs_subs: Vec::new(),
            rhs: rhs.dereference(),
            rhs_subs: Vec::new(),
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

    pub fn prepend_substitutions_for_primary(&mut self, subs: Substitutions) {
        let sub_list = match self.self_side {
            EqualityTestSide::Left => &mut self.lhs_subs,
            EqualityTestSide::Right => &mut self.rhs_subs,
        };
        sub_list.insert(0, subs);
    }

    pub fn try_select_value_substituted_for_var_in_primary(
        &self,
        target_to_look_for: &VariablePtr,
    ) -> Result<Option<Self>, UnresolvedItemError> {
        let sub_list = match &self.self_side {
            EqualityTestSide::Left => &self.lhs_subs,
            EqualityTestSide::Right => &self.rhs_subs,
        };
        let mut dependencies = Dependencies::new();
        for dep in target_to_look_for.borrow().dependencies() {
            dependencies.append(dep.get_dependencies());
        }
        let mut new_sub_list = Vec::new();
        let mut new_value = None;
        'sub_list_loop: for (sub_index, sub) in sub_list.iter().enumerate() {
            let mut new_sub = Substitutions::new();
            let mut new_dependencies = Dependencies::new();
            for (target, value) in sub {
                if target.is_same_instance_as(target_to_look_for) {
                    let mut sub_without_target = sub.clone();
                    sub_without_target.remove(target).unwrap();
                    new_sub_list.push(sub_without_target);
                    new_sub_list.extend(sub_list[sub_index + 1..].iter().cloned());
                    new_value = Some(value.ptr_clone());
                    break 'sub_list_loop;
                } else if dependencies.contains_var(target) {
                    dependencies.remove(target);
                    new_dependencies.append(value.get_dependencies());
                    new_sub.insert_no_replace(target.ptr_clone(), value.ptr_clone());
                }
            }
            dependencies.append(new_dependencies);
            if new_sub.len() > 0 {
                new_sub_list.push(new_sub);
            }
        }
        if let Some(err) = dependencies.error() {
            Err(err.clone())
        } else if let Some(value) = new_value {
            Ok(Some(match self.self_side {
                EqualityTestSide::Left => Self {
                    lhs: value,
                    lhs_subs: new_sub_list,
                    rhs: self.rhs.ptr_clone(),
                    rhs_subs: self.rhs_subs.clone(),
                    self_side: self.self_side,
                },
                EqualityTestSide::Right => Self {
                    lhs: self.lhs.ptr_clone(),
                    lhs_subs: self.lhs_subs.clone(),
                    rhs: value,
                    rhs_subs: new_sub_list,
                    self_side: self.self_side,
                },
            }))
        } else {
            Ok(None)
        }
    }

    pub fn with_primary(&self, new_primary: ItemPtr) -> Self {
        self.with_primary_and_other(new_primary, self.other().ptr_clone())
    }

    pub fn with_primary_and_other(&self, new_primary: ItemPtr, new_other: ItemPtr) -> Self {
        let (lhs, rhs) = match self.self_side {
            EqualityTestSide::Left => (new_primary, new_other),
            EqualityTestSide::Right => (new_other, new_primary),
        };
        Self {
            lhs: lhs.dereference(),
            lhs_subs: self.lhs_subs.clone(),
            rhs: rhs.dereference(),
            rhs_subs: self.rhs_subs.clone(),
            self_side: self.self_side,
        }
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
        let result = if let Equal::Yes(lhs, rhs) = result {
            Ok(Equal::Yes(lhs, rhs))
        } else if result == Equal::Unknown {
            self.get_equality_right()
        } else {
            Ok(result)
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
            .get_equality_using_context(self, OnlyCalledByEcc(()))?)
    }

    pub fn other_with_subs(&self) -> ItemPtr {
        let mut other = self.other().ptr_clone();
        let sub_list = match self.self_side {
            EqualityTestSide::Left => &self.rhs_subs,
            EqualityTestSide::Right => &self.lhs_subs,
        };
        for subs in sub_list {
            other = unchecked_substitution(other, subs);
        }
        other
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
