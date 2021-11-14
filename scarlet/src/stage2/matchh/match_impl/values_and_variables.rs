use MatchResult::*;

use crate::stage2::{
    matchh::result::MatchResult,
    structure::{BuiltinValue, Environment, ConstructId, VarType, VariableId},
};

impl<'x> Environment<'x> {
    pub(super) fn on_right_variable(
        &mut self,
        original_value: ConstructId<'x>,
        value: ConstructId<'x>,
        other: ConstructId<'x>,
        eager_vars: &[VariableId<'x>],
        var: VariableId<'x>,
    ) -> MatchResult<'x> {
        let result = self.matches_impl(original_value, value, other, eager_vars);
        result
            .keeping_only_eager_subs(eager_vars)
            .with_sub_if_match(var, original_value)
    }

    pub(super) fn on_right_set_eager(
        &mut self,
        all: bool,
        eager: bool,
        eager_vars: &[VariableId<'x>],
        vals: Vec<ConstructId<'x>>,
        original_value: ConstructId<'x>,
        value: ConstructId<'x>,
        base: ConstructId<'x>,
    ) -> MatchResult<'x> {
        if eager {
            let mut new_eagers = eager_vars.to_owned();
            if all {
                for (dep, _) in self.get_deps(base) {
                    new_eagers.push(dep.var);
                }
            } else {
                for val in vals {
                    for (dep, _) in self.get_deps(val) {
                        new_eagers.push(dep.var);
                    }
                }
            }
            self.matches_impl(original_value, value, base, &new_eagers[..])
        } else {
            todo!()
        }
    }
}

pub(super) fn on_value_value<'x>(value: BuiltinValue, pattern: BuiltinValue) -> MatchResult<'x> {
    if value == pattern {
        MatchResult::non_capturing()
    } else {
        NoMatch
    }
}

pub(super) fn on_variable_variable<'x>(
    value_type: VarType<'x>,
    pattern_type: VarType<'x>,
    var: VariableId<'x>,
    original_value: ConstructId<'x>,
) -> MatchResult<'x> {
    if value_type == pattern_type || pattern_type == VarType::God {
        MatchResult::non_capturing().with_sub_if_match(var, original_value)
    } else {
        NoMatch
    }
}

pub(super) fn on_value_variable<'x>(
    bvalue: BuiltinValue,
    typee: VarType<'x>,
    var: VariableId<'x>,
    original_value: ConstructId<'x>,
) -> MatchResult<'x> {
    let matches = match bvalue {
        BuiltinValue::_32U(..) => typee == VarType::_32U || typee == VarType::God,
        BuiltinValue::Bool(..) => typee == VarType::Bool || typee == VarType::God,
    };
    if matches {
        MatchResult::non_capturing().with_sub_if_match(var, original_value)
    } else {
        NoMatch
    }
}
