mod equal;
mod tests;

use std::{collections::HashSet, ops::ControlFlow};

pub use equal::Equal;
use itertools::Itertools;
use typed_arena::Arena;

use super::{
    dependencies::{DepResult, Dependencies},
    Environment, UnresolvedItemError,
};
use crate::{
    constructs::{
        substitution::{CSubstitution, Substitutions},
        variable::{CVariable, Variable, VariableId},
        Construct, ItemId,
    },
    scope::LookupInvariantResult,
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

pub type DeqResult = Result<Equal, UnresolvedItemError>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DiscoverEqualQuery {
    left: ItemId,
    right: ItemId,
}

impl<'x> Environment<'x> {
    pub fn are_same_item(
        &mut self,
        left: ItemId,
        right: ItemId,
    ) -> Result<bool, UnresolvedItemError> {
        let left = self.dereference(left)?;
        let right = self.dereference(right)?;
        Ok(left == right)
    }

    pub fn discover_equal(&mut self, left: ItemId, right: ItemId, limit: u32) -> DeqResult {
        self.discover_equal_with_subs(left, vec![], right, vec![], limit)
    }

    fn compute_dependencies_with_subs(
        &mut self,
        base: ItemId,
        subs: &[&Substitutions],
    ) -> DepResult {
        let mut deps = self.get_dependencies(base);
        for subs in subs {
            deps = CSubstitution::sub_deps(deps, subs, &HashSet::new(), self);
        }
        deps
    }

    fn filter_subs(
        &mut self,
        base: ItemId,
        subs: &Substitutions,
    ) -> Result<Substitutions, UnresolvedItemError> {
        let deps = self.get_dependencies(base);
        if let Some(err) = deps.error() {
            return Err(err);
        }
        let subs = subs
            .into_iter()
            .filter(|(var, _)| deps.contains_var(*var))
            .copied()
            .collect::<Substitutions>();
        Ok(subs)
    }

    fn filter_subs_list(
        &mut self,
        base: ItemId,
        subs: Vec<&Substitutions>,
    ) -> Result<Vec<Substitutions>, UnresolvedItemError> {
        let mut result = Vec::new();
        let mut base = base;
        for subs in subs {
            let subs = self.filter_subs(base, subs)?;
            base = self.substitute_unchecked(base, &subs);
            result.push(subs);
        }
        Ok(result)
    }

    pub(crate) fn discover_equal_with_subs(
        &mut self,
        left: ItemId,
        left_subs: Vec<&Substitutions>,
        right: ItemId,
        right_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> DeqResult {
        let extra_sub_holder = Arena::new();
        let mut left = self.dereference(left)?;
        let mut right = self.dereference(right)?;
        let mut left_subs = left_subs.into_iter().map(|x| &*x).collect_vec();
        let mut right_subs = right_subs.into_iter().map(|x| &*x).collect_vec();
        let trace = false;
        if trace {
            println!();
            println!("{:?} {:?} = {:?} {:?}?", left, left_subs, right, right_subs);
        };
        if left == right {
            if left_subs.len() > 0 || right_subs.len() > 0 {
                // todo!();
            } else {
                if trace {
                    println!("Ok({:?})", Equal::yes());
                }
                return Ok(Equal::yes());
            }
        }
        if limit == 0 {
            if trace {
                println!("Ok({:?})", Equal::NeedsHigherLimit);
            }
            return Ok(Equal::NeedsHigherLimit);
        }
        while let Ok(con) = self.get_item_as_construct(left) {
            if let Some((base, extra_subs, _)) = con.dyn_clone().dereference(self) {
                left = base;
                if let Some(extra_subs) = extra_subs {
                    let extra_subs = extra_sub_holder.alloc(extra_subs.clone());
                    left_subs.insert(0, extra_subs);
                }
            } else {
                break;
            }
        }
        while let Ok(con) = self.get_item_as_construct(right) {
            if let Some((base, extra_subs, _)) = con.dyn_clone().dereference(self) {
                right = base;
                if let Some(extra_subs) = extra_subs {
                    let extra_subs = extra_sub_holder.alloc(extra_subs.clone());
                    right_subs.insert(0, extra_subs);
                }
            } else {
                break;
            }
        }
        let left_subs_src = self.filter_subs_list(left, left_subs)?;
        let right_subs_src = self.filter_subs_list(right, right_subs)?;
        let left_subs = left_subs_src.iter().collect_vec();
        let mut right_subs = right_subs_src.iter().collect_vec();
        if left == right {
            if left_subs.len() > 0 || right_subs.len() > 0 {
                // todo!();
            } else {
                if trace {
                    println!("Ok({:?})", Equal::yes());
                }
                return Ok(Equal::yes());
            }
        }
        let rvar_id =
            if let Some(rvar) = self.get_and_downcast_construct_definition::<CVariable>(right)? {
                for (index, subs) in right_subs.iter().enumerate() {
                    if let Some(sub) = subs.get(&rvar.get_id()) {
                        let mut without_this_sub = right_subs[index].clone();
                        without_this_sub.remove(&rvar.get_id());
                        right_subs[index] = extra_sub_holder.alloc(without_this_sub);
                        return self
                            .discover_equal_with_subs(left, left_subs, *sub, right_subs, limit)
                            .map(|x| x.sort(self));
                    }
                }
                Some(rvar.get_id())
            } else {
                None
            };
        if let Some(lvar) = self.get_and_downcast_construct_definition::<CVariable>(left)? {
            let lvar = lvar.clone();
            return self
                .handle_lhs_variable(
                    left_subs,
                    lvar,
                    &extra_sub_holder,
                    right,
                    right_subs,
                    limit,
                    trace,
                    rvar_id,
                )
                .map(|x| x.sort(self));
        }
        // For now this produces no noticable performance improvements.
        // if let Some((_, result)) = self.def_equal_memo_table.iso_get(&(left, right,
        // limit)) {     return result.clone();
        // }
        let left_def = self.get_item_as_construct(left)?.dyn_clone();
        let right_def = self.get_item_as_construct(right)?.dyn_clone();
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
        result.map(|x| x.sort(self))
    }

    fn handle_lhs_variable<'a>(
        &mut self,
        left_subs: Vec<&'a Substitutions>,
        lvar: CVariable,
        extra_sub_holder: &'a Arena<Substitutions>,
        right: ItemId,
        right_subs: Vec<&'a Substitutions>,
        limit: u32,
        trace: bool,
        rvar_id: Option<VariableId>,
    ) -> DeqResult {
        let (left_subs, right_subs) = match self.handle_lhs_substitution(
            left_subs,
            &lvar,
            extra_sub_holder,
            right,
            right_subs,
            limit,
        ) {
            ControlFlow::Break(value) => return value,
            ControlFlow::Continue((l, r)) => (l, r),
        };
        if limit == 0 {
            if trace {
                println!("Ok({:?})", Equal::NeedsHigherLimit);
            }
            return Ok(Equal::NeedsHigherLimit);
        }
        let lvar = lvar.clone();
        let lvar = self.get_variable(lvar.get_id()).clone();
        let ldeps = lvar.get_dependencies();
        let ldeps = ldeps
            .iter()
            .flat_map(|&dep| self.get_dependencies(dep).into_variables())
            .collect_vec();
        if Some(lvar.id.unwrap()) == rvar_id {
            // We can only get here if the right variable isn't substituted.
            return self.check_var_is_same(&lvar, trace, left_subs, right_subs, limit);
        }
        let mut limit_reached = false;
        for base_index in (0..=right_subs.len()).rev() {
            let rdeps = self.compute_dependencies_with_subs(right, &right_subs[..base_index]);
            if ldeps.len() > rdeps.num_variables() {
                continue;
            }
            let mut equal = Equal::yes();
            for (ldep, rdep) in ldeps.iter().zip(rdeps.as_variables()) {
                let ldep = self.get_variable(ldep.id).item.unwrap();
                let rdep = self.get_variable(rdep.id).item.unwrap();
                let deps_equal = self.discover_equal_with_subs(
                    ldep,
                    left_subs.clone(),
                    rdep,
                    Vec::from(&right_subs[base_index..]),
                    limit - 1,
                )?;
                equal = Equal::and(vec![equal, deps_equal]);
            }
            if let Equal::Yes(mut subs) = equal {
                let mut right = right;
                for sub in &right_subs[..base_index] {
                    let rdeps = self.get_dependencies(right);
                    let mut filtered_sub = Substitutions::new();
                    for dep in rdeps.into_variables() {
                        if let Some(&value) = sub.get(&dep.id) {
                            filtered_sub.insert_or_replace(dep.id, value);
                        }
                    }
                    right = self.substitute_unchecked(right, &filtered_sub);
                }
                let mut dep_subs = Substitutions::new();
                for (ldep, rdep) in ldeps.iter().zip(rdeps.as_variables()) {
                    if ldep.id == rdep.id {
                        continue;
                    }
                    let ldep = self.get_variable(ldep.id).item.unwrap();
                    dep_subs.insert_no_replace(rdep.id, ldep);
                }
                let right = self.substitute_unchecked(right, &dep_subs);
                subs.insert_no_replace(lvar.id.unwrap(), right);
                if trace {
                    println!("Ok(Equal::Yes({:?}))", subs);
                }
                return Ok(Equal::Yes(subs));
            } else if let Equal::NeedsHigherLimit = equal {
                limit_reached = true;
            }
        }
        let result = Ok(if limit_reached {
            Equal::NeedsHigherLimit
        } else {
            Equal::Unknown
        });
        if trace {
            println!("{:?}", result);
        }
        return result;
    }

    fn handle_lhs_substitution<'a>(
        &mut self,
        mut left_subs: Vec<&'a Substitutions>,
        lvar: &CVariable,
        extra_sub_holder: &'a Arena<Substitutions>,
        right: ItemId,
        right_subs: Vec<&'a Substitutions>,
        limit: u32,
    ) -> ControlFlow<
        Result<Equal, UnresolvedItemError>,
        (Vec<&'a Substitutions>, Vec<&'a Substitutions>),
    > {
        for (index, subs) in left_subs.iter().enumerate() {
            if let Some(sub) = subs.get(&lvar.get_id()) {
                let mut without_this_sub = left_subs[index].clone();
                without_this_sub.remove(&lvar.get_id());
                left_subs[index] = extra_sub_holder.alloc(without_this_sub);
                return ControlFlow::Break(self.discover_equal_with_subs(
                    *sub,
                    left_subs.clone(),
                    right,
                    right_subs,
                    limit,
                ));
            }
        }
        ControlFlow::Continue((left_subs, right_subs))
    }

    fn check_var_is_same(
        &mut self,
        lvar: &Variable,
        trace: bool,
        left_subs: Vec<&Substitutions>,
        right_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> DeqResult {
        let parts = lvar
            .get_dependencies()
            .iter()
            .map(|&dep| {
                self.discover_equal_with_subs(
                    dep,
                    left_subs.clone(),
                    dep,
                    right_subs.clone(),
                    limit - 1,
                )
            })
            .collect::<Result<_, _>>()?;
        let result = Ok(Equal::and(parts));
        if trace {
            println!("{:?}", result);
        }
        result
    }
}
