mod pattern_connectives;
mod values_and_variables;

use MatchResult::*;

use crate::stage2::{
    matchh::result::MatchResult,
    structure::{Definition, Environment, ItemId, VarType, VariableId},
};

impl<'x> Environment<'x> {
    pub(super) fn matches_impl(
        &mut self,
        original_value: ItemId<'x>,
        value: ItemId<'x>,
        pattern: ItemId<'x>,
        eager_vars: &[VariableId<'x>],
    ) -> MatchResult<'x> {
        let value_def = self.get_resolved_definition(value).clone();
        let pattern_def = self.get_resolved_definition(pattern).clone();

        if let Definition::Variable { var, .. } = pattern_def {
            if eager_vars.contains(&var) {
                return Unknown;
            }
        }
        if let Definition::Variable {
            typee: VarType::And(left, right),
            var,
        } = pattern_def
        {
            self.on_right_and(original_value, value, left, eager_vars, right, var)
        } else if let Definition::Variable {
            typee: VarType::Or(left, right),
            var,
            ..
        } = value_def
        {
            self.on_left_or(original_value, left, pattern, eager_vars, right, var)
        } else if let Definition::Variable {
            typee: VarType::Or(left, right),
            var,
        } = pattern_def
        {
            self.on_right_or(original_value, value, left, eager_vars, right, var)
        } else if let Definition::Variable {
            typee: VarType::And(left, right),
            var,
        } = value_def
        {
            self.on_left_and(original_value, left, pattern, eager_vars, right, var)
        } else if let Definition::Variable {
            typee: VarType::Just(other),
            var,
        } = pattern_def
        {
            self.on_right_variable(original_value, value, other, eager_vars, var)
        } else if let Definition::Variable {
            typee: VarType::Just(other),
            ..
        } = value_def
        {
            self.matches_impl(original_value, other, pattern, eager_vars)
        } else if let Definition::SetEager {
            base,
            vals,
            all,
            eager,
        } = pattern_def
        {
            self.on_right_set_eager(all, eager, eager_vars, vals, original_value, value, base)
        } else if let Definition::SetEager { base, .. } = value_def {
            self.matches_impl(original_value, base, pattern, eager_vars)
        } else if let (Definition::Struct(..), Definition::Struct(..)) = (&value_def, &pattern_def)
        {
            todo!()
        } else if let (&Definition::BuiltinValue(value), &Definition::BuiltinValue(pattern)) =
            (&value_def, &pattern_def)
        {
            values_and_variables::on_value_value(value, pattern)
        } else if let (
            &Definition::Variable {
                typee: value_type, ..
            },
            &Definition::Variable {
                typee: pattern_type,
                var,
            },
        ) = (&value_def, &pattern_def)
        {
            values_and_variables::on_variable_variable(
                value_type,
                pattern_type,
                var,
                original_value,
            )
        } else if let (&Definition::BuiltinValue(bvalue), &Definition::Variable { typee, var }) =
            (&value_def, &pattern_def)
        {
            values_and_variables::on_value_variable(bvalue, typee, var, original_value)
        } else if let (
            Definition::BuiltinOperation(value_op, value_args),
            Definition::BuiltinOperation(pattern_op, pattern_args),
        ) = (value_def.clone(), pattern_def.clone())
        {
            if value_op == pattern_op {
                let mut results = Vec::new();
                assert_eq!(value_args.len(), pattern_args.len());
                for (value_arg, pattern_arg) in value_args.into_iter().zip(pattern_args.into_iter())
                {
                    results.push(self.matches(value_arg, pattern_arg));
                }
                let result = MatchResult::and(results);
                if result.is_guaranteed_match() {
                    return result;
                }
            }
            let value_bp = self.find_bounding_pattern(value);
            self.matches_impl(original_value, value_bp, pattern, eager_vars)
        } else if let Definition::BuiltinOperation(_value_op, _value_args) = value_def.clone() {
            let value_bp = self.find_bounding_pattern(value);
            self.matches_impl(original_value, value_bp, pattern, eager_vars)
        } else {
            Unknown
        }
    }
}
