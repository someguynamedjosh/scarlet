use itertools::Itertools;

use crate::{
    constructs::member::{CMember, Member},
    environment::resolve::transform::{
        transformers::special_members::base::SpecialMember, ApplyContext,
    },
    tokens::structure::Token,
};

pub struct Indexing;
impl SpecialMember for Indexing {
    fn aliases(&self) -> &'static [&'static str] {
        &["INDEXING", "INDEX", "I"]
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
        let def = CMember(
            base,
            Member::Index {
                index,
                proof_lt_len,
            },
        );
        Token::Construct(c.push_construct(Box::new(def)))
    }
}
