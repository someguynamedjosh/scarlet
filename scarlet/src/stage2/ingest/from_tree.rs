mod match_def;
mod others;
mod struct_def;
mod substitute_def;
mod using;

use std::marker::PhantomData;

use super::top_level::IngestionContext;
use crate::{
    stage1::structure::TokenTree,
    stage2::structure::{BuiltinOperation, Definition, ItemId, VarType, Variable},
};

impl<'e, 'x> IngestionContext<'e, 'x> {
    pub fn definition_from_tree(
        &mut self,
        src: &'x TokenTree<'x>,
        into: ItemId<'x>,
    ) -> Definition<'x> {
        match src {
            TokenTree::Token(token) => self.token_def(token),

            TokenTree::BuiltinRule {
                name: "eager",
                body,
            } => self.eagerness_def(body, true),
            TokenTree::BuiltinRule {
                name: "match",
                body,
            } => self.match_def(body),
            TokenTree::BuiltinRule {
                name: "matches",
                body,
            } => self.matches_def(body),
            TokenTree::BuiltinRule {
                name: "member",
                body,
            } => self.member_def(body),
            TokenTree::BuiltinRule { name: "show", body } => self.show_def(body, into),
            TokenTree::BuiltinRule {
                name: "struct",
                body,
            } => self.struct_def(body),
            TokenTree::BuiltinRule {
                name: "substitute",
                body,
            } => self.substitute_def(body),
            TokenTree::BuiltinRule {
                name: "using",
                body,
            } => self.using_def(body),
            TokenTree::BuiltinRule {
                name: "variable",
                body,
            } => self.variable_def(body),

            TokenTree::BuiltinRule {
                name: "PATTERN", ..
            } => self.var_with_special_type(VarType::God),
            TokenTree::BuiltinRule { name: "32U", .. } => self.var_with_special_type(VarType::_32U),
            TokenTree::BuiltinRule { name: "BOOL", .. } => {
                self.var_with_special_type(VarType::Bool)
            }
            TokenTree::BuiltinRule { name: "AND", body } => self.and_pattern_def(body),
            TokenTree::BuiltinRule { name: "OR", body } => self.or_pattern_def(body),

            TokenTree::BuiltinRule {
                name: "sum_32u",
                body,
            } => self.builtin_op_def(BuiltinOperation::Sum32U, body),
            TokenTree::BuiltinRule {
                name: "dif_32u",
                body,
            } => self.builtin_op_def(BuiltinOperation::Dif32U, body),

            TokenTree::BuiltinRule { name, .. } => {
                todo!("Nice error, unrecognized builtin {}", name)
            }
        }
    }

    fn builtin_op_def(
        &mut self,
        op: BuiltinOperation,
        body: &'x Vec<TokenTree<'x>>,
    ) -> Definition<'x> {
        let args: Vec<_> = body.iter().map(|tt| self.ingest_tree(tt)).collect();
        Definition::BuiltinOperation(op, args)
    }

    fn and_pattern_def(&mut self, body: &'x Vec<TokenTree<'x>>) -> Definition<'x> {
        assert_eq!(body.len(), 2);
        let left = self.ingest_tree(&body[0]);
        let right = self.ingest_tree(&body[1]);
        self.var_with_special_type(VarType::And(left, right))
    }

    fn or_pattern_def(&mut self, body: &'x Vec<TokenTree<'x>>) -> Definition<'x> {
        assert_eq!(body.len(), 2);
        let left = self.ingest_tree(&body[0]);
        let right = self.ingest_tree(&body[1]);
        self.var_with_special_type(VarType::Or(left, right))
    }

    fn var_with_special_type(&mut self, typee: VarType<'x>) -> Definition<'x> {
        // assert_eq!(
        //     body.len(),
        //     0,
        //     "TODO: Nice error, expected zero argument to builtin."
        // );
        let var = Variable { pd: PhantomData };
        let var = self.env.vars.push(var);
        Definition::Variable { var, typee }
    }
}
