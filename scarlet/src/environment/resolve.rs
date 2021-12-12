use super::{ConstructDefinition, ConstructId, Environment};
use crate::{
    constructs::substitution::{CSubstitution, Substitutions},
    tokens::structure::Token,
    transform::{self, ApplyContext}, scope::Scope,
};

impl<'x> Environment<'x> {
    pub fn resolve(&mut self, con_id: ConstructId) -> ConstructId {
        let con = &self.constructs[con_id];
        if let ConstructDefinition::Unresolved(token) = &con.definition {
            if let &Token::Construct(con_id) = token {
                self.resolve(con_id)
            } else {
                let token = token.clone();
                let scope = &*con.scope;
                let new_def = self.resolve_token(token, scope);
                self.constructs[con_id].definition = new_def;
                self.check(con_id);
                self.resolve(con_id)
            }
        } else {
            con_id
        }
    }

    fn resolve_token(&mut self, token: Token<'x>, scope: &dyn Scope) -> ConstructDefinition<'x> {
        match token {
            Token::Construct(..) => unreachable!(),
            Token::Plain(ident) => {
                let scope = scope.dyn_clone();
                match scope.lookup_ident(self, ident.as_ref()) {
                    Some(id) => ConstructDefinition::Unresolved(Token::Construct(id)),
                    None => {
                        println!("{:#?}", self);
                        todo!("Nice error, bad ident {} in {:?}", ident, scope)
                    }
                }
            }
            Token::Stream {
                label: "CONSTRUCT_SYNTAX",
                contents,
            } => {
                let mut context = ApplyContext { env: self };
                let mut contents = contents;
                transform::apply_transformers(&mut context, &mut contents, &Default::default());
                assert_eq!(contents.len(), 1);
                ConstructDefinition::Unresolved(contents.into_iter().next().unwrap())
            }
            Token::Stream {
                label: "SUBSTITUTE",
                mut contents,
            } => {
                let base = self.push_unresolved(contents.remove(0), scope);
                self.reduce(base);
                let base = self.resolve(base);
                self.constructs[base].scope = scope.dyn_clone();
                let mut deps = self.get_dependencies(base);
                let mut subs = Substitutions::new();
                let mut anonymous_subs = Vec::new();
                for sub in contents {
                    match sub {
                        Token::Stream {
                            label: "target",
                            contents: _,
                        } => {
                            todo!()
                        }
                        _ => anonymous_subs.push(sub),
                    }
                }
                for sub in anonymous_subs {
                    let sub = self.push_unresolved(sub, scope);
                    self.reduce(sub);
                    let sub = self.resolve(sub);
                    let mut match_found = false;
                    for (idx, _dep) in deps.iter().enumerate() {
                        // TODO: Type checking
                        if true {
                            let dep = deps.remove(idx);
                            subs.insert_no_replace(dep, sub);
                            match_found = true;
                            break;
                        }
                    }
                    if !match_found {
                        println!("{:#?}", self);
                        todo!(
                            "Nice error, {:?} cannot be assigned to any of:\n{:#?}",
                            sub,
                            deps
                        );
                    }
                }
                ConstructDefinition::Resolved(Box::new(CSubstitution(base, subs)))
            }
            Token::Stream { label, .. } => todo!(
                "Nice error, bad label '{:?}', expected CONSTRUCT_SYNTAX or SUBSTITUTE",
                label
            ),
        }
    }
}
