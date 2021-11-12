use itertools::Itertools;

use super::structure::{Member, Substitutions};
use crate::{
    shared::OrderedSet,
    stage2::{
        matchh::MatchResult,
        structure::{
            BuiltinOperation, BuiltinValue, Definition, Environment, ItemId, Token, VarType,
            VariableInfo,
        },
        transformers::{self, ApplyContext},
    },
};

impl<'x> Environment<'x> {
    fn index_in_bounds_theorem(&mut self, index: ItemId<'x>, length: u32) -> ItemId<'x> {
        let length = self.push_def(Definition::BuiltinValue(BuiltinValue::_32U(length)));
        let in_bounds = self.push_def(Definition::BuiltinOperation(
            BuiltinOperation::LessThan32U,
            vec![index, length],
        ));
        let in_bounds = self.reduce(in_bounds);
        let truee = self.push_def(Definition::BuiltinValue(BuiltinValue::Bool(true)));
        let theorem_var_type = VarType::And(in_bounds, truee);
        self.push_var(theorem_var_type)
    }

    fn check_index(
        &mut self,
        index: ItemId<'x>,
        t_index_in_range: ItemId<'x>,
        base_bounding_pattern: ItemId<'x>,
    ) -> bool {
        match self.get_definition(base_bounding_pattern) {
            Definition::BuiltinOperation(_, _) => unreachable!(),
            Definition::BuiltinValue(_) => false,
            Definition::Match { .. } => unreachable!(),
            Definition::Member(..) => todo!(),
            Definition::Unresolved(..) => unreachable!(),
            Definition::SetEager { .. } => unreachable!(),
            Definition::Struct(fields) => {
                let length = fields.len() as u32;
                let theorem_pattern = self.index_in_bounds_theorem(index, length);
                self.matches(t_index_in_range, theorem_pattern)
                    .is_guaranteed_match()
            }
            Definition::Substitute(..) => unreachable!(),
            Definition::Variable { var, typee } => todo!(),
        }
    }

    pub fn check(&mut self, item: ItemId<'x>) {
        // let pattern_bool = self.get_or_push_var(VarType::Bool);
        let pattern_32u = self.get_or_push_var(VarType::_32U);
        match self.get_definition(item).clone() {
            Definition::BuiltinOperation(op, args) => match op {
                BuiltinOperation::Sum32U
                | BuiltinOperation::Difference32U
                | BuiltinOperation::Product32U
                | BuiltinOperation::Quotient32U
                | BuiltinOperation::Modulo32U
                | BuiltinOperation::Power32U
                | BuiltinOperation::LessThan32U
                | BuiltinOperation::LessThanOrEqual32U
                | BuiltinOperation::GreaterThan32U
                | BuiltinOperation::GreaterThanOrEqual32U => {
                    for arg in args {
                        if !self.matches(arg, pattern_32u).is_guaranteed_match() {
                            todo!("Nice error, {:?} is not a 32U", arg)
                        }
                    }
                }
            },
            Definition::BuiltinValue(..) => (),
            Definition::Match { .. } => (),
            Definition::Member(base, member) => {
                let bound = self.find_bounding_pattern(base);
                match member {
                    Member::Named(_) => todo!(),
                    Member::Index {
                        index,
                        proof_lt_len,
                    } => {
                        let index = self.reduce(index);
                        let proof_lt_len = self.reduce(proof_lt_len);
                        if !self.check_index(index, proof_lt_len, bound) {
                            println!("{:#?}", self);
                            todo!(
                                "Nice error, {:?} is not a proof that {:?} is in range of {:?}",
                                proof_lt_len,
                                index,
                                base
                            )
                        }
                    }
                }
            }
            Definition::Unresolved(..) => {
                let item = self.resolve(item);
                self.check(item);
            }
            Definition::SetEager { .. } => (),
            Definition::Struct(..) => (),
            Definition::Substitute(..) => {
                // Checked in substitute resolution. TODO: Don't trust it.
                ()
            }
            Definition::Variable { .. } => (),
        }
    }
}
