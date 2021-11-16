use itertools::Itertools;

use crate::{
    environment::resolve::transform::{
        basics::Extras,
        transformers::{
            special_members::base::SpecialMember,
            statements::{Else, OnPattern},
        },
        ApplyContext,
    },
    tfers,
    tokens::structure::Token,
};

pub struct MemberAtIndex;
impl SpecialMember for MemberAtIndex {
    fn aliases(&self) -> &'static [&'static str] {
        &["MemberAtIndex", "Member", "Mem"]
    }

    fn expects_bracket_group(&self) -> bool {
        true
    }

    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        base: Token<'t>,
        bracket_group: Option<Vec<Token<'t>>>,
    ) -> Token<'t> {
        let base = c.push_unresolved(base);
        let (index, proof_lt_len) = bracket_group.unwrap().into_iter().collect_tuple().unwrap();
        let index = c.push_unresolved(index);
        let proof_lt_len = c.push_unresolved(proof_lt_len);
        let def = Definition::Member(
            base,
            Member::Index {
                index,
                proof_lt_len,
            },
        );
        Token::Construct(c.push_construct(def))
    }
}
