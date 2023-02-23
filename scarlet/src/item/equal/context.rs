use std::collections::HashSet;

use super::{equal::Equal, trim::trim_result};
use crate::{
    definitions::parameter::Parameter,
    item::{
        definitions::{
            substitution::{DSubstitution, Substitutions},
            variable::VariablePtr,
        },
        parameters::Parameters,
        ItemId,
    },
    util::PtrExtension,
};

const TRACE: bool = false;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EqualityTestSide {
    Left,
    Right,
}

impl EqualityTestSide {
    fn opposite(&self) -> EqualityTestSide {
        match self {
            EqualityTestSide::Left => EqualityTestSide::Right,
            EqualityTestSide::Right => EqualityTestSide::Left,
        }
    }
}

#[derive(Debug)]
pub struct EqualityContext {
    lhs: ItemId,
    lhs_subs: Vec<Substitutions>,
    rhs: ItemId,
    rhs_subs: Vec<Substitutions>,
    self_side: EqualityTestSide,
}

/// Using this in a function signature guarantees that only
/// EqualityContext can call that function. If you are reusing this
/// inside the function that is being called, you are doing something wrong.
pub struct OnlyCalledByEqualityContext(pub(super) ());

impl EqualityContext {
    fn new(lhs: ItemId, rhs: ItemId) -> Self {
        Self {
            lhs: lhs.dereference(),
            lhs_subs: Vec::new(),
            rhs: rhs.dereference(),
            rhs_subs: Vec::new(),
            self_side: EqualityTestSide::Left,
        }
    }

    pub fn primary(&self) -> &ItemId {
        match self.self_side {
            EqualityTestSide::Left => self.lhs(),
            EqualityTestSide::Right => self.rhs(),
        }
    }

    pub fn other(&self) -> &ItemId {
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

    pub fn lhs(&self) -> &ItemId {
        &self.lhs
    }

    pub fn rhs(&self) -> &ItemId {
        &self.rhs
    }

    pub fn prepend_substitutions_for_primary(&mut self, subs: Substitutions) {
        let sub_list = match self.self_side {
            EqualityTestSide::Left => &mut self.lhs_subs,
            EqualityTestSide::Right => &mut self.rhs_subs,
        };
        sub_list.insert(0, subs);
    }

    fn try_select_value_substituted_for_var(
        &self,
        target_to_look_for: &VariablePtr,
        side: EqualityTestSide,
    ) -> Option<Self> {
        let sub_list = match side {
            EqualityTestSide::Left => &self.lhs_subs,
            EqualityTestSide::Right => &self.rhs_subs,
        };
        let mut parameters = Parameters::new_empty();
        for dep in target_to_look_for.borrow().dependencies() {
            parameters.append(dep.get_dependencies());
        }
        let mut new_sub_list = Vec::new();
        let mut new_value = None;
        for sub in sub_list {
            let mut new_sub = Substitutions::new();
            let mut new_parameters = Parameters::new_empty();
            for (target, value) in sub {
                if target.is_same_instance_as(target_to_look_for) && new_value.is_none() {
                    new_value = Some(value.ptr_clone());
                    let mut sub_without_target = sub.clone();
                    sub_without_target.remove(target).unwrap();
                    new_parameters.append(value.get_dependencies());
                } else if parameters.contains(target) {
                    parameters.remove(target);
                    new_parameters.append(value.get_dependencies());
                    new_sub.insert_no_replace(target.ptr_clone(), value.ptr_clone());
                }
            }
            parameters.append(new_parameters);
            if new_sub.len() > 0 {
                new_sub_list.push(new_sub);
            }
        }
        if let Some(value) = new_value {
            Some(match side {
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
            })
        } else {
            None
        }
    }

    pub fn try_select_value_substituted_for_var_in_primary(
        &self,
        target_to_look_for: &VariablePtr,
    ) -> Option<Self> {
        self.try_select_value_substituted_for_var(target_to_look_for, self.self_side)
    }

    pub fn try_select_value_substituted_for_var_in_other(
        &self,
        target_to_look_for: &VariablePtr,
    ) -> Option<Self> {
        self.try_select_value_substituted_for_var(target_to_look_for, self.self_side.opposite())
    }

    pub fn with_primary(&self, new_primary: ItemId) -> Self {
        self.with_primary_and_other(new_primary, self.other().ptr_clone())
    }

    pub fn with_primary_and_other(&self, new_primary: ItemId, new_other: ItemId) -> Self {
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
    pub fn get_equality_left(&mut self) -> Equal {
        if TRACE {
            println!("{:#?} =<= {:#?}", self.lhs, self.rhs);
            println!("LHS Subs: {:#?}", self.lhs_subs);
            println!("RHS Subs: {:#?}", self.rhs_subs);
        }
        self.self_side = EqualityTestSide::Left;
        let lhs = self.lhs.ptr_clone();
        let lhs = lhs.clone_definition();
        let result = lhs.get_equality_using_context(self, OnlyCalledByEqualityContext(()))?;
        let result = if let Equal::Yes(lhs, rhs) = result {
            if rhs.len() > 0 {
                if let Ok(Equal::Yes(lhs, rhs)) = self.get_equality_right() {
                    Ok(Equal::Yes(lhs, rhs))
                } else {
                    Ok(Equal::Yes(lhs, rhs))
                }
            } else {
                Ok(Equal::Yes(lhs, rhs))
            }
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
    pub fn get_equality_right(&mut self) -> Equal {
        self.self_side = EqualityTestSide::Right;
        let rhs = self.rhs.ptr_clone();
        let rhs = rhs.clone_definition();
        rhs.get_equality_using_context(self, OnlyCalledByEqualityContext(()))?
    }

    pub fn other_with_subs(&self) -> ItemId {
        let mut other = self.other().ptr_clone();
        let sub_list = match self.self_side {
            EqualityTestSide::Left => &self.rhs_subs,
            EqualityTestSide::Right => &self.lhs_subs,
        };
        for subs in sub_list {
            other = unchecked_substitution(other, subs.clone()).unwrap();
        }
        other
    }

    /// No, I don't want
    pub fn no_subs(&self) -> bool {
        for subs in &self.lhs_subs {
            if subs.len() > 0 {
                return false;
            }
        }
        for subs in &self.rhs_subs {
            if subs.len() > 0 {
                return false;
            }
        }
        true
    }

    pub(crate) fn other_subs(&self) -> &[Substitutions] {
        match self.self_side {
            EqualityTestSide::Left => &self.rhs_subs,
            EqualityTestSide::Right => &self.lhs_subs,
        }
    }
}

impl ItemId {
    pub fn get_equality_left(&self, other: &Self) -> Equal {
        EqualityContext::new(self.ptr_clone(), other.ptr_clone()).get_equality_left()
    }

    pub fn get_equality_right(&self, other: &Self) -> Equal {
        EqualityContext::new(self.ptr_clone(), other.ptr_clone()).get_equality_right()
    }

    /// Gets equality and trims away redundant substitutions like x -> x,
    /// y -> x(x IS y), fx -> fx(x IS y)(y IS x) and so on.
    pub fn get_trimmed_equality(&self, other: &Self) -> Equal {
        let mut result = self.get_equality_left(other);
        trim_result(&mut result);
        result
    }
}
