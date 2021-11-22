mod result;

pub use result::MatchResult;

use super::{ConstructId, Environment};
use crate::constructs::{self, as_variable, variable::VarType};

impl<'x> Environment<'x> {
    pub fn construct_as_var_type(&mut self, id: ConstructId) -> VarType {
        if let Some(var) = constructs::as_variable(&**self.get_construct(id)) {
            var.typee.clone()
        } else {
            VarType::Just(id)
        }
    }

    pub fn var_type_matches_var_type(&mut self, value: &VarType, pattern: &VarType) -> MatchResult {
        if let (VarType::Just(original_value), VarType::Just(con)) = (value, pattern) {
            if let Some(var) = as_variable(&**self.get_construct(*con)) {
                if var.capturing {
                    let var = var.clone();
                    let pattern = &var.typee;
                    return match self.var_type_matches_var_type(value, pattern) {
                        MatchResult::Match(..) => {
                            MatchResult::non_capturing().with_sub_if_match(var, *original_value)
                        }
                        other => other,
                    };
                }
            }
        }
        match (value, pattern) {
            (_, VarType::Anything) => MatchResult::non_capturing(),
            (VarType::Or(l, r), _) => {
                let l = self.construct_as_var_type(*l);
                let r = self.construct_as_var_type(*r);
                MatchResult::and(vec![
                    self.var_type_matches_var_type(&l, pattern),
                    self.var_type_matches_var_type(&r, pattern),
                ])
            }
            (_, VarType::And(l, r)) => {
                let l = self.construct_as_var_type(*l);
                let r = self.construct_as_var_type(*r);
                MatchResult::and(vec![
                    self.var_type_matches_var_type(value, &l),
                    self.var_type_matches_var_type(value, &r),
                ])
            }
            (VarType::And(l, r), _) => {
                let l = self.construct_as_var_type(*l);
                let r = self.construct_as_var_type(*r);
                MatchResult::or(vec![
                    self.var_type_matches_var_type(&l, pattern),
                    self.var_type_matches_var_type(&r, pattern),
                ])
            }
            (_, VarType::Or(l, r)) => {
                let l = self.construct_as_var_type(*l);
                let r = self.construct_as_var_type(*r);
                MatchResult::or(vec![
                    self.var_type_matches_var_type(value, &l),
                    self.var_type_matches_var_type(value, &r),
                ])
            }
            (VarType::Anything, _) => MatchResult::NoMatch,
            (VarType::Bool, VarType::Bool) => MatchResult::non_capturing(),
            (VarType::Bool, _) => MatchResult::NoMatch,
            (VarType::_32U, VarType::_32U) => MatchResult::non_capturing(),
            (VarType::_32U, _) => MatchResult::NoMatch,
            (VarType::Array { .. }, _) => todo!(),
            (VarType::Just(value), _) => self.construct_matches_simple_var_type(*value, pattern),
        }
    }

    pub fn construct_matches_construct(
        &mut self,
        value: ConstructId,
        pattern: ConstructId,
    ) -> MatchResult {
        self.var_type_matches_var_type(&VarType::Just(value), &VarType::Just(pattern))
    }

    pub fn construct_matches_simple_var_type(
        &mut self,
        value: ConstructId,
        pattern: &VarType,
    ) -> MatchResult {
        match pattern {
            VarType::Anything | VarType::And(..) | VarType::Or(..) => {
                panic!("{:?} is not a simple pattern", pattern)
            }
            _ => (),
        }
        if let VarType::Just(con) = pattern {
            if let Some(var) = as_variable(&**self.get_construct(*con)) {
                if var.capturing {
                    panic!("{:?} is not a simple pattern", pattern)
                }
            }
        }
        self.get_construct(value)
            .dyn_clone()
            .matches_simple_var_type(self, pattern)
    }
}
