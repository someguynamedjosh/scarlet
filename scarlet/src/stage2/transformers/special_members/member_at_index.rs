use itertools::Itertools;

use super::base::SpecialMember;
use crate::stage2::structure::{Definition, Environment, Member, Token};

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
        env: &mut Environment<'t>,
        base: Token<'t>,
        paren_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = env.push_def(Definition::Unresolved(base));
        let (index, proof_lt_len) = paren_group.unwrap().into_iter().collect_tuple().unwrap();
        let index = env.push_def(Definition::Unresolved(index));
        let proof_lt_len = env.push_def(Definition::Unresolved(proof_lt_len));
        let def = Definition::Member(
            base,
            Member::Index {
                index,
                proof_lt_len,
            },
        );
        Token::Item(env.push_def(def))
    }
}
