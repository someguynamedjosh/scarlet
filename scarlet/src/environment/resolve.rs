use std::borrow::Cow;

use super::{ConstructDefinition, ConstructId, Environment};
use crate::{
    constructs::{
        self,
        substitution::{CSubstitution, Substitutions},
        variable::CVariable,
    },
    shared::OrderedMap,
    tokens::structure::Token,
    transform::{self, ApplyContext},
};

impl<'x> Environment<'x> {
    pub fn resolve(&mut self, con_id: ConstructId) -> ConstructId {
        let con = &self.constructs[con_id];
        if let ConstructDefinition::Unresolved(token) = &con.definition {
            if let &Token::Construct(con_id) = token {
                self.resolve(con_id)
            } else {
                let token = token.clone();
                let parent = con.parent_scope;
                let new_def = self.resolve_token(token, parent);
                self.constructs[con_id].definition = new_def;
                self.resolve(con_id)
            }
        } else {
            con_id
        }
    }

    fn lookup_ident(&mut self, in_scope: Option<ConstructId>, ident: &str) -> Option<ConstructId> {
        let in_scope = in_scope?;
        let as_struct = constructs::as_struct(&**self.get_construct(in_scope));
        if let Some(structt) = as_struct {
            todo!()
        }
        let parent = self.constructs[in_scope].parent_scope;
        self.lookup_ident(parent, ident)
    }

    fn resolve_token(
        &mut self,
        token: Token<'x>,
        scope: Option<ConstructId>,
    ) -> ConstructDefinition<'x> {
        match token {
            Token::Construct(..) => unreachable!(),
            Token::Plain(ident) => {
                if ident == "true" {
                    self.get_builtin_item("true").into()
                } else if ident == "false" {
                    self.get_builtin_item("false").into()
                // } else if let Ok(_) = ident.parse() {
                //     todo!()
                } else {
                    match self.lookup_ident(scope, ident.as_ref()) {
                        Some(id) => ConstructDefinition::Unresolved(Token::Construct(id)),
                        None => {
                            println!("{:#?}", self);
                            todo!("Nice error, bad ident {} in {:?}", ident, scope)
                        }
                    }
                }
            }
            Token::Stream {
                label: "CONSTRUCT_SYNTAX",
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
            Token::Stream {
                label: "SUBSTITUTE",
                mut contents,
            } => {
                let base = self.push_unresolved(contents.remove(0), scope);
                self.reduce(base);
                let base = self.resolve(base);
                self.constructs[base].parent_scope = scope;
                let mut deps = self.get_dependencies(base);
                let mut subs = Substitutions::new();
                let mut anonymous_subs = Vec::new();
                for sub in contents {
                    match sub {
                        Token::Stream {
                            label: "target",
                            contents,
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
                    for (idx, dep) in deps.iter().enumerate() {
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
