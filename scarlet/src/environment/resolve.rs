mod transform;

use super::{BoxedConstruct, ConstructDefinition, ConstructId, Environment};
use crate::tokens::structure::Token;

impl<'x> Environment<'x> {
    pub fn resolve(&mut self, con_id: ConstructId<'x>) -> ConstructId<'x> {
        let con = &self.constructs[con_id];
        if let ConstructDefinition::Unresolved(token) = &con.definition {
            if let Token::Construct(con_id) = token {
                *con_id
            } else {
                let token = token.clone();
                let new_def = self.resolve_token(token);
                self.constructs[con_id].definition = ConstructDefinition::Resolved(new_def);
                con_id
            }
        } else {
            con_id
        }
    }

    fn resolve_token(&mut self, token: Token<'x>) -> BoxedConstruct<'x> {
        match token {
            Token::Construct(..) => unreachable!(),
            Token::Plain(ident) => todo!(),
            Token::Stream {
                label: "construct_syntax",
                contents,
            } => todo!(),
            Token::Stream { label, .. } => todo!(
                "Nice error, token stream with label '{:?}' cannot be resolved",
                label
            ),
        }
    }
}
