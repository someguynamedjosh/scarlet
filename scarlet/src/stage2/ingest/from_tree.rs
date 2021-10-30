mod match_def;
mod others;
mod struct_def;
mod substitute_def;
mod using;

use super::top_level::IngestionContext;
use crate::{
    stage1::structure::TokenTree,
    stage2::structure::{BuiltinOperation, Definition, Environment, ItemId, Pattern, Variable},
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
            } => Pattern::God.into(),
            TokenTree::BuiltinRule { name: "32U", .. } => Pattern::_32U.into(),
            TokenTree::BuiltinRule { name: "BOOL", .. } => Pattern::Bool.into(),
            TokenTree::BuiltinRule { name: "AND", body } => self.and_pattern_def(body),

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
        Pattern::And(left, right).into()
    }
}
