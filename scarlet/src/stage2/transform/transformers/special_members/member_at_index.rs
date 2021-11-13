use itertools::Itertools;

use super::base::SpecialMember;
use crate::stage2::{
    structure::{Definition, Environment, Member, Token},
    transform::ApplyContext,
};

pub struct MemberAtIndex;
impl SpecialMember for MemberAtIndex {
    fn aliases(&self) -> &'static [&'static str] {
        &["MemberAtIndex", "Member", "Mem"]
    }

    fn expects_paren_group(&self) -> bool {
        true
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = c.push_token(base);
        let (index, proof_lt_len) = paren_group.unwrap().into_iter().collect_tuple().unwrap();
        let index = c.push_token(index);
        let proof_lt_len = c.push_token(proof_lt_len);
        let def = Definition::Member(
            base,
            Member::Index {
                index,
                proof_lt_len,
            },
        );
        Token::Item(c.push_def(def))
    }
}
