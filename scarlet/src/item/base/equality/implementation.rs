use std::{cell::Ref, intrinsics::caller_location};

use itertools::Itertools;
use owning_ref::OwningRef;

use super::{
    item_wrapper::ItemWithSubsAndRecursion, Equal, EqualResult, EqualityCalculationContext,
    OnlyCalledByEcc, PermissionToRefine,
};
use crate::{
    item::{
        definitions::{
            other::DOther,
            substitution::{DSubstitution, Substitutions},
            variable::{DVariable, VariablePtr},
        },
        dependencies::{Dependencies, Dependency},
        resolvable::UnresolvedItemError,
        util::unchecked_substitution,
        Item, ItemPtr,
    },
    util::PtrExtension,
};

const TRACE: bool = true;

enum VariableResult {
    ResultReached(EqualResult),
    ProceedWithVariable(Option<VariablePtr>),
}

impl EqualityCalculationContext {
    pub(super) fn get_equality_impl(&mut self) -> EqualResult {
        match self.get_equality_impl_impl() {
            Ok(Equal::Yes(subs, mut rec_over)) => {
                rec_over.append(&mut self.lhs.recurses_over.clone());
                rec_over.append(&mut self.rhs.recurses_over.clone());
                Ok(Equal::Yes(subs, rec_over))
            }
            other => other,
        }
    }

    fn get_equality_impl_impl(&mut self) -> EqualResult {
        if TRACE {
            println!(
                "{:#?} {:#?} = {:#?} {:#?}",
                self.lhs.item, self.lhs.subs, self.rhs.item, self.rhs.subs
            );
            println!(
                "--------------------------------------------------------------------------------"
            );
        }
        if self.lhs.item.is_same_instance_as(&self.rhs.item) {
            if self.lhs.subs.len() > 0 || self.rhs.subs.len() > 0 {
                // todo!();
            } else {
                return Ok(Equal::yes());
            }
        }
        if self.limit == 0 {
            return Ok(Equal::NeedsHigherLimit);
        }
        // Fully dereference LHS. We do not need any of the intermediate results.
        while self.lhs.dereference_once() {}
        let mut could_use_higher_limit = false;
        let lvar = match self
            .check_and_handle_left_variable_by_dereferencing_rhs(&mut could_use_higher_limit)
        {
            VariableResult::ResultReached(result) => return result,
            VariableResult::ProceedWithVariable(var) => var,
        };
        if let Some(result) = self.after_lhs_substitution(lvar, could_use_higher_limit) {
            return result;
        }
        let lhs = self.lhs.item.ptr_clone();
        let def = &lhs.borrow().definition;
        let result =
            def.get_equality_using_context(self, PermissionToRefine(()), OnlyCalledByEcc(()))?;
        match result {
            Equal::Yes(..) | Equal::No => Ok(result),
            _ => {
                if could_use_higher_limit {
                    Ok(Equal::NeedsHigherLimit)
                } else {
                    Ok(Equal::Unknown)
                }
            }
        }
    }

    fn after_lhs_substitution(
        &mut self,
        lvar: Option<VariablePtr>,
        could_use_higher_limit: bool,
    ) -> Option<EqualResult> {
        // Just in case the previous expression didn't get to fully dereference rhs...
        while self.rhs.dereference_once() {}
        let rvar = match self.check_and_handle_right_variable_substitution() {
            VariableResult::ResultReached(result) => return Some(result),
            VariableResult::ProceedWithVariable(var) => var,
        };
        if self.lhs.item.is_same_instance_as(&self.rhs.item) {
            if self.lhs.subs.len() > 0 || self.rhs.subs.len() > 0 {
                // todo!();
            } else {
                return Some(Ok(Equal::yes()));
            }
        }
        if let (Some(lvar), Some(rvar)) = (lvar, rvar) {
            // We can only get here if the variables aren't substituted.  We do
            // an early return if either of the variables are substituted. We
            // only need to pass one var because we already know they are the
            // same variable.
            if lvar.is_same_instance_as(&rvar) {
                return Some(self.check_var_args_are_equal(lvar));
            } else {
                return if could_use_higher_limit {
                    Some(Ok(Equal::NeedsHigherLimit))
                } else {
                    Some(Ok(Equal::Unknown))
                };
            }
        }
        None
    }

    fn check_and_handle_left_variable_by_dereferencing_rhs(
        &mut self,
        could_use_higher_limit: &mut bool,
    ) -> VariableResult {
        let lvar = if let Some(lvar) = self.lhs.item.downcast_definition::<DVariable>() {
            lvar.get_variable().ptr_clone()
        } else {
            return VariableResult::ProceedWithVariable(None);
        };
        if let Some(result) = self.handle_substitutions(&lvar, |this| &mut this.lhs) {
            return VariableResult::ResultReached(result);
        }
        let ldeps = lvar
            .borrow()
            .dependencies()
            .iter()
            .flat_map(|dep| dep.get_dependencies().into_variables())
            .collect_vec();
        loop {
            match self.left_variable_equals_right(&ldeps) {
                Ok(res) => match res {
                    Equal::Yes(subs, rec_over) => {
                        return VariableResult::ResultReached(
                            self.make_result_where_right_is_assigned_to_left_variable(
                                &lvar, &ldeps, subs, rec_over,
                            ),
                        );
                    }
                    Equal::NeedsHigherLimit => *could_use_higher_limit = true,
                    Equal::Unknown => (),
                    Equal::No => {
                        return VariableResult::ResultReached(Ok(Equal::No));
                    }
                },
                Err(err) => {
                    return VariableResult::ResultReached(Err(err));
                }
            }
            if self.rhs.dereference_once() {
                if TRACE {
                    println!("Dereferenced and trying again.")
                }
            } else {
                if TRACE {
                    println!("Cannot dereference any more.")
                }
                break;
            }
        }
        VariableResult::ProceedWithVariable(Some(lvar))
    }

    fn left_variable_equals_right(&mut self, ldeps: &Vec<Dependency>) -> EqualResult {
        let rdeps = self
            .rhs
            .item
            .get_dependencies()
            .into_variables()
            .collect_vec();
        if ldeps.len() > rdeps.len() {
            return Ok(Equal::Unknown);
        }
        let mut equal = Equal::yes();
        for (ldep, rdep) in ldeps.iter().zip(rdeps.iter()) {
            let ldep = ldep.var.borrow().item().ptr_clone();
            let rdep = rdep.var.borrow().item().ptr_clone();
            if TRACE {
                println!(
                    "Testing if dependency {:#?} and {:#?} are equal.",
                    ldep, rdep
                );
            }
            let deps_equal = self.refine_and_get_equality(ldep, rdep, PermissionToRefine(()))?;
            equal = Equal::and(vec![equal, deps_equal]);
        }
        Ok(equal)
    }

    fn check_and_handle_right_variable_substitution(&mut self) -> VariableResult {
        let rvar = if let Some(rvar) = self.rhs.item.downcast_definition::<DVariable>() {
            rvar.get_variable().ptr_clone()
        } else {
            return VariableResult::ProceedWithVariable(None);
        };
        let result = self.handle_substitutions(&rvar, |this| &mut this.rhs);
        result.map_or_else(
            || VariableResult::ProceedWithVariable(Some(rvar)),
            |result| VariableResult::ResultReached(result),
        )
    }

    fn make_result_where_right_is_assigned_to_left_variable(
        &mut self,
        lvar: &VariablePtr,
        ldeps: &Vec<Dependency>,
        mut subs: Substitutions,
        rec_over: Vec<ItemPtr>,
    ) -> EqualResult {
        if TRACE {
            println!("Making result where right is assigned to left variable.");
            println!("Left variable is {:#?}", lvar);
            println!("Right is {:#?} {:#?}", self.rhs.item, self.rhs.subs);
        }
        // The algorithm:
        // 1. Store any substitutions that are not targeting dependencies that
        //    will get converted to the lhs' dependencies. This ensures we don't
        //    convert a statement about a[something_specific] to a statement
        //    about a.
        // 2. Replace all the rhs' dependencies with the corresponding lhs
        //    dependencies. E.G. If lhs is dependent on a, b and rhs is
        //    dependent on x, y then x -> a and y -> b.
        // 3. Check if the result is equal to the original lhs given the rhs
        //    subs.
        // 4. If they are not the same, append lhs -> replacement to the list of
        //    substitutions generated by checking if each lhs dependency was equal
        //    to an rhs dependency.
        // 5. Return Ok(Equal::Yes(subs, rec_over)).

        let rdeps = self
            .rhs
            .item
            .get_dependencies()
            .into_variables()
            .collect_vec();

        // The value the lhs dex is going to be replaced by.
        let dex_replacement = self.store_substitions_not_appearing_on_lhs(ldeps, &rdeps);
        println!("{:#?}", ldeps);
        println!("{:#?}", rdeps);
        println!("Dex replacement is {:#?}", dex_replacement);
        let dex_replacement = self.replace_rhs_vars_with_lhs_deps(dex_replacement, ldeps, &rdeps);
        // Step 3
        if !self.check_if_new_rhs_is_just_lhs(lvar, &dex_replacement) {
            // Step 4
            subs.insert_no_replace(lvar.ptr_clone(), dex_replacement);
        }
        // Step 5
        println!("Resulting subs are {:#?}", subs);
        Ok(Equal::Yes(subs, rec_over))
    }

    /// Step 1 of make_result_where_right_is_assigned_to_left_variable.
    fn store_substitions_not_appearing_on_lhs(
        &self,
        ldeps: &Vec<Dependency>,
        rdeps: &[Dependency],
    ) -> ItemPtr {
        // 1. Store any substitutions that are not targeting dependencies that
        //    will get converted to the lhs' dependencies. This ensures we don't
        //    convert a statement about a[something_specific] to a statement
        //    about a.
        let lhs_replacement = self.rhs.item.ptr_clone();
        let skipped_deps = &rdeps[0..ldeps.len()];
        let remaining_rdeps = rdeps
            .iter()
            .skip(ldeps.len())
            .collect_vec();
        let mut remaining_rdep_subs = Substitutions::new();
        for dep in remaining_rdeps {
            println!("DEP {:#?}", dep);
            let mut subbed = dep.var.borrow().item().ptr_clone();
            for sub in &self.rhs.subs {
                // This is only necessary to produce clean results I.E. produce
                // a blank list of substitutions when values are identical and
                // need no substitutions.
                let sdeps = subbed.get_dependencies();
                let mut filtered_subs = Substitutions::new();
                for dep in sdeps.into_variables() {
                    if skipped_deps.contains(&dep) {
                        continue;
                    }
                    if let Some(value) = sub.get(&dep.var) {
                        filtered_subs.insert_or_replace(dep.var.ptr_clone(), value.ptr_clone());
                    }
                }
                subbed = unchecked_substitution(subbed, &filtered_subs);
            }
            if !subbed.is_same_instance_as(dep.var.borrow().item()) {
                remaining_rdep_subs.insert_no_replace(dep.var.ptr_clone(), subbed);
            }
        }
        unchecked_substitution(lhs_replacement, &remaining_rdep_subs)
    }

    /// Step 2 of make_result_where_right_is_assigned_to_left_variable.
    pub(crate) fn replace_rhs_vars_with_lhs_deps(
        &self,
        dex_replacement: ItemPtr,
        ldeps: &[Dependency],
        rdeps: &[Dependency],
    ) -> ItemPtr {
        let mut dep_subs = Substitutions::new();
        for (ldep, rdep) in ldeps.iter().zip(rdeps.iter()) {
            // Skip unnecessary substitutions.
            if ldep.var.is_same_instance_as(&rdep.var) {
                continue;
            }
            let ldep = ldep.var.borrow().item().ptr_clone();
            dep_subs.insert_no_replace(rdep.var.ptr_clone(), ldep);
        }
        unchecked_substitution(dex_replacement, &dep_subs)
    }

    fn handle_substitutions(
        &mut self,
        of: &VariablePtr,
        selector: impl FnOnce(&mut Self) -> &mut ItemWithSubsAndRecursion,
    ) -> Option<EqualResult> {
        let selected = selector(self);
        for (index, subs) in selected.subs.iter().enumerate() {
            if let Some(sub) = subs.get(of) {
                let sub = sub.ptr_clone();
                selected.select_substitution(index, of, sub);
                if TRACE {
                    println!("{}", caller_location());
                }
                return Some(self.get_equality_impl());
            }
        }
        None
    }

    pub(crate) fn check_var_args_are_equal(&mut self, var: VariablePtr) -> EqualResult {
        let parts = var
            .borrow()
            .get_dependencies()
            .iter()
            .map(|dep| {
                if TRACE {
                    println!("{}", caller_location());
                }
                self.refine_and_get_equality(
                    dep.ptr_clone(),
                    dep.ptr_clone(),
                    PermissionToRefine(()),
                )
            })
            .collect::<Result<_, _>>()?;
        let result = Ok(Equal::and(parts));
        result
    }

    fn check_if_new_rhs_is_just_lhs(&mut self, lvar: &VariablePtr, new_rhs: &ItemPtr) -> bool {
        let old_rhs = self.rhs.clone();
        self.rhs.item = new_rhs.ptr_clone();
        let result =
            self.after_lhs_substitution(Some(lvar.ptr_clone()), false) == Some(Ok(Equal::yes()));
        self.rhs = old_rhs;
        result
    }
}
