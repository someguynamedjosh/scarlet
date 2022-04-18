use std::cell::Ref;

use itertools::Itertools;
use owning_ref::OwningRef;

use super::{
    item_wrapper::ItemWithSubsAndRecursion, Equal, EqualResult, EqualityCalculationContext,
};
use crate::{
    item::{
        definitions::{
            other::DOther,
            substitution::{DSubstitution, Substitutions},
            variable::{DVariable, VariablePtr},
        },
        resolvable::UnresolvedItemError,
        Item, ItemPtr,
    },
    util::PtrExtension,
};

enum VariableResult {
    ResultReached(EqualResult),
    ProceedWithVariable(Option<VariablePtr>),
}

impl EqualityCalculationContext {
    pub(super) fn get_equality_impl(&mut self) -> EqualResult {
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
        let lvar = match self.check_and_handle_left_variable_by_dereferencing_rhs() {
            VariableResult::ResultReached(result) => return result,
            VariableResult::ProceedWithVariable(var) => var,
        };
        let rvar = match self.check_and_handle_right_variable_substitution() {
            VariableResult::ResultReached(result) => return result,
            VariableResult::ProceedWithVariable(var) => var,
        };
        todo!()
    }

    fn check_and_handle_left_variable_by_dereferencing_rhs(&mut self) -> VariableResult {
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

        todo!()
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
                return Some(self.get_equality_impl());
            }
        }
        None
    }
}
