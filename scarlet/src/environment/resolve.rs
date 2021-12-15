use itertools::Itertools;

use super::{ConstructDefinition, ConstructId, Environment};
use crate::{
    constructs::{
        as_variable,
        substitution::{CSubstitution, Substitutions},
        variable::CVariable, self,
    },
    scope::{SPlain, Scope},
    tokens::structure::Token,
    transform::{self, ApplyContext},
};

impl<'x> Environment<'x> {
    pub fn resolve_all(&mut self) {
        let mut next_id = self.constructs.first();
        while let Some(id) = next_id {
            self.resolve(id);
            next_id = self.constructs.next(id);
        }
    }

    pub fn resolve(&mut self, con_id: ConstructId) -> ConstructId {
        let con = &self.constructs[con_id];
        if let ConstructDefinition::Unresolved(token) = &con.definition {
            let mut token = token.clone();
            token.resolve_items(self);
            if let &Token::Construct(con_id) = &token {
                self.resolve(con_id)
            } else {
                let token = token.clone();
                let new_def = self.resolve_token(token, con_id);
                self.constructs[con_id].definition = new_def;
                self.resolve(con_id)
            }
        } else {
            con_id
        }
    }

    fn resolve_token(&mut self, token: Token<'x>, this: ConstructId) -> ConstructDefinition<'x> {
        match token {
            Token::Construct(..) => unreachable!(),
            Token::Plain(ident) => {
                let scope = self.get_construct(this).scope.dyn_clone();
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
                let token = contents.into_iter().next().unwrap();
                let scope = self.get_construct(this).scope.dyn_clone();
                token.set_scope_of_items(self, &*scope);
                ConstructDefinition::Unresolved(token)
            }
            Token::Stream {
                label: "VARIABLE",
                mut contents,
            } => {
                let depends_on = contents
                    .pop()
                    .unwrap()
                    .into_stream()
                    .iter()
                    .map(Token::unwrap_construct)
                    .collect_vec();
                let invariants = contents
                    .pop()
                    .unwrap()
                    .into_stream()
                    .iter()
                    .map(Token::unwrap_construct)
                    .collect_vec();
                let depends_on = depends_on
                    .into_iter()
                    .map(|dep| {
                        let def = self.get_construct_definition(dep);
                        if let Some(var) = as_variable(&**def) {
                            var.clone()
                        } else {
                            todo!("Nice error, dependency is not a variable.")
                        }
                    })
                    .collect_vec();
                let id = self.variables.push(constructs::variable::Variable);
                CVariable::new(self, id, invariants, false, depends_on).into()
            }
            Token::Stream {
                label: "SUBSTITUTE",
                mut contents,
            } => {
                let base = self.push_unresolved(contents.remove(0));
                self.set_scope(base, &SPlain(this));
                let base = self.resolve(base);
                self.set_scope(base, &SPlain(this));
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
                    let sub = self.push_unresolved(sub);
                    self.set_scope(sub, &SPlain(this));
                    let sub = self.resolve(sub);
                    let mut match_found = false;
                    for (idx, _dep) in deps.iter().enumerate() {
                        // TODO: Type checking
                        if true {
                            let dep = deps.remove(idx);
                            subs.insert_no_replace(dep, sub);
                            self.set_scope(sub, &SPlain(this));
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
                CSubstitution::into(self, this, base, subs)
            }
            Token::Stream { label, .. } => todo!(
                "Nice error, bad label '{:?}', expected CONSTRUCT_SYNTAX, VARIABLE or SUBSTITUTE",
                label
            ),
        }
    }
}
