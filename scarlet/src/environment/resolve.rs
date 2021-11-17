mod transform;

use super::{ConstructDefinition, ConstructId, Environment};
use crate::{environment::resolve::transform::ApplyContext, tokens::structure::Token};

impl<'x> Environment<'x> {
    pub fn resolve(&mut self, con_id: ConstructId) -> ConstructId {
        let con = &self.constructs[con_id];
        if let ConstructDefinition::Unresolved(token) = &con.definition {
            if let Token::Construct(con_id) = token {
                *con_id
            } else {
                let token = token.clone();
                let new_def = self.resolve_token(token);
                self.constructs[con_id].definition = new_def;
                self.resolve(con_id)
            }
        } else {
            con_id
        }
    }

    fn resolve_token(&mut self, token: Token<'x>) -> ConstructDefinition<'x> {
        match token {
            Token::Construct(..) => unreachable!(),
            Token::Plain(_ident) => todo!(),
            Token::Stream {
                label: "construct_syntax",
                contents,
            } => {
                let mut context = ApplyContext {
                    env: self,
                    parent_scope: None,
                };
                let mut contents = contents;
                transform::apply_transformers(&mut context, &mut contents, &Default::default());
                assert_eq!(contents.len(), 1);
                ConstructDefinition::Unresolved(contents.into_iter().next().unwrap())
            }
            Token::Stream { label, .. } => todo!(
                "Nice error, token stream with label '{:?}' cannot be resolved",
                label
            ),
        }
    }
}
