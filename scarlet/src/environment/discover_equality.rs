mod equal;
mod tests;

pub use equal::Equal;
use itertools::Itertools;
use typed_arena::Arena;

use super::{Environment, UnresolvedConstructError};
use crate::constructs::{
    substitution::{CSubstitution, Substitutions},
    variable::CVariable,
    ConstructId,
};

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
        self.discover_equal_with_subs(left, vec![], right, vec![], limit)
    }

    pub(crate) fn discover_equal_with_subs(
        &mut self,
        left: ConstructId,
        left_subs: Vec<&Substitutions>,
        right: ConstructId,
        right_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> DeqResult {
        let extra_sub_holder = Arena::new();
        let mut left = self.dereference(left)?;
        let mut right = self.dereference(right)?;
        let mut left_subs = left_subs.into_iter().map(|r| &*r).collect_vec();
        let mut right_subs = right_subs.into_iter().map(|r| &*r).collect_vec();
        let trace = false;
        if trace {
            println!();
            println!("{:?} = {:?}?", left, right);
        };
        if left == right {
            if trace {
                println!("Ok({:?})", Equal::yes());
            }
            return Ok(Equal::yes());
        }
        if limit == 0 {
            if trace {
                println!("Ok({:?})", Equal::NeedsHigherLimit);
            }
            return Ok(Equal::NeedsHigherLimit);
        }
        while let Some(lsub) = self.get_and_downcast_construct_definition::<CSubstitution>(left)? {
            left = lsub.base();
            let subs = extra_sub_holder.alloc(lsub.substitutions().clone());
            left_subs.insert(0, subs);
        }
        while let Some(rsub) = self.get_and_downcast_construct_definition::<CSubstitution>(right)? {
            right = rsub.base();
            let subs = extra_sub_holder.alloc(rsub.substitutions().clone());
            right_subs.insert(0, subs);
        }
        if let Some(lvar) = self.get_and_downcast_construct_definition::<CVariable>(left)? {
            while left_subs.len() > 0 {
                let subs = left_subs.remove(0);
                if let Some(sub) = subs.get(&lvar.get_id()) {
                    return self.discover_equal_with_subs(*sub, left_subs, right, right_subs, limit);
                }
            }
        }
        if let Some(rvar) = self.get_and_downcast_construct_definition::<CVariable>(right)? {
            while right_subs.len() > 0 {
                let subs = right_subs.remove(0);
                if let Some(sub) = subs.get(&rvar.get_id()) {
                    return self.discover_equal_with_subs(left, left_subs, *sub, right_subs, limit);
                }
            }
        }
        // For now this produces no noticable performance improvements.
        // if let Some((_, result)) = self.def_equal_memo_table.iso_get(&(left, right,
        // limit)) {     return result.clone();
        // }
        let left_def = self.get_construct_definition(left)?.dyn_clone();
        let right_def = self.get_construct_definition(right)?.dyn_clone();
        if trace {
            println!("{:#?} = {:#?}", left_def, right_def);
        }
        let limit = limit - 1;
        let result =
            left_def.discover_equality(self, left_subs, right, &*right_def, right_subs, limit);
        if trace {
            println!("{:?}", result);
        }
        // self.def_equal_memo_table
        //     .insert((left, right, limit).convert(), result.clone());
        result
    }
}
