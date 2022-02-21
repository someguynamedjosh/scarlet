mod tests;

use super::{sub_expr::OwnedSubExpr, Environment, UnresolvedConstructError};
use crate::{
    constructs::{substitution::Substitutions, variable::VariableId, ConstructId},
    environment::sub_expr::SubExpr,
    shared::TripleBool,
    util::{IsomorphicKeyIndexable, Isomorphism},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Equal {
    Yes(Substitutions, Substitutions),
    NeedsHigherLimit,
    Unknown,
    No,
}

fn combine_substitutions(from: Substitutions, target_subs: &mut Substitutions) -> Result<(), ()> {
    for (target, value) in from {
        if target_subs.contains_key(&target) {
            return Err(());
        } else {
            target_subs.insert_no_replace(target, value);
        }
    }
    Ok(())
}

impl Equal {
    pub fn yes() -> Self {
        Self::Yes(Default::default(), Default::default())
    }

    pub fn swapped(self) -> Self {
        match self {
            Self::Yes(left, right) => Self::Yes(right, left),
            other => other,
        }
    }

    pub fn and(over: Vec<Self>) -> Self {
        let mut default = Self::yes();
        for b in over {
            match b {
                Self::Yes(left, right) => {
                    if let Self::Yes(exleft, exright) = &mut default {
                        let success = (|| -> Result<(), ()> {
                            combine_substitutions(left, exleft)?;
                            combine_substitutions(right, exright)?;
                            Ok(())
                        })();
                        if success.is_err() {
                            default = Self::Unknown
                        }
                    }
                }
                Self::NeedsHigherLimit => {
                    if let Self::Yes(..) = default {
                        default = Self::NeedsHigherLimit
                    }
                }
                Self::Unknown => default = Self::Unknown,
                Self::No => return Self::No,
            }
        }
        default
    }

    pub fn or(over: Vec<Self>) -> Self {
        let mut default = Self::No;
        for b in over {
            match b {
                Self::Yes(..) => return b,
                Self::Unknown => {
                    if let Self::No = default {
                        default = Self::Unknown
                    }
                }
                Self::NeedsHigherLimit => default = Self::NeedsHigherLimit,
                Self::No => return Self::No,
            }
        }
        default
    }

    pub fn without_subs(self) -> Self {
        match self {
            Self::Yes(..) => Self::yes(),
            other => other,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeqSide {
    Left,
    Right,
}

impl Default for DeqSide {
    fn default() -> Self {
        Self::Left
    }
}

impl DeqSide {
    fn swapped(self) -> DeqSide {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

pub type DeqPriority = u8;

pub type DeqResult = Result<Equal, UnresolvedConstructError>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DiscoverEqualQuery {
    left: ConstructId,
    right: ConstructId,
}

impl<'x> Environment<'x> {
    pub fn are_same_construct(
        &mut self,
        left: ConstructId,
        right: ConstructId,
    ) -> Result<bool, UnresolvedConstructError> {
        let left = self.dereference(left)?;
        let right = self.dereference(right)?;
        Ok(left == right)
    }

    pub fn discover_equal(
        &mut self,
        left: ConstructId,
        right: ConstructId,
        limit: u32,
    ) -> DeqResult {
        self.discover_equal_with_tiebreaker(left, right, limit, DeqSide::default())
    }

    pub(crate) fn discover_equal_with_tiebreaker(
        &mut self,
        left: ConstructId,
        right: ConstructId,
        limit: u32,
        tiebreaker: DeqSide,
    ) -> DeqResult {
        let left = self.dereference(left)?;
        let right = self.dereference(right)?;
        println!();
        println!("{:?} = {:?}?", left, right);
        if left == right {
            println!("Ok({:?})", Equal::yes());
            return Ok(Equal::yes());
        }
        if limit == 0 {
            println!("Ok({:?})", Equal::NeedsHigherLimit);
            return Ok(Equal::NeedsHigherLimit);
        }
        // For now this produces no noticable performance improvements.
        // if let Some((_, result)) = self.def_equal_memo_table.iso_get(&(left, right,
        // limit)) {     return result.clone();
        // }
        let result = (|| {
            let left_def = self.get_construct_definition(left)?.dyn_clone();
            let right_def = self.get_construct_definition(right)?.dyn_clone();
            let left_prio = left_def.deq_priority();
            let right_prio = right_def.deq_priority();
            println!("{:#?} = {:#?}", left_def, right_def);
            let preference = if left_prio > right_prio {
                DeqSide::Left
            } else if right_prio > left_prio {
                DeqSide::Right
            } else {
                tiebreaker
            };
            let limit = limit - 1;
            if preference == DeqSide::Left {
                left_def.discover_equality(self, right, &*right_def, limit, tiebreaker)
            } else {
                let tiebreaker = tiebreaker.swapped();
                let res = right_def.discover_equality(self, left, &*left_def, limit, tiebreaker);
                Ok(res?.swapped())
            }
        })();
        println!("{:?}", result);
        // self.def_equal_memo_table
        //     .insert((left, right, limit).convert(), result.clone());
        result
    }
}
