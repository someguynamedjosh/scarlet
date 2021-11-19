mod transform;

use super::{ConstructDefinition, ConstructId, Environment};
use crate::{
    constructs::{
        self,
        builtin_value::CBuiltinValue,
        substitution::{CSubstitution, Substitutions},
        variable::{CVariable, VarType},
        Construct,
    },
    environment::resolve::transform::ApplyContext,
    shared::OrderedMap,
    tokens::structure::Token,
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
            for (index, field) in structt.0.iter().enumerate() {
                let index_string = format!("{}", index);
                let name = field.name.as_ref().unwrap_or(&index_string);
                if name == ident {
                    return Some(field.value);
                }
            }
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
                    ConstructDefinition::Resolved(Box::new(CBuiltinValue::Bool(true)))
                } else if ident == "false" {
                    ConstructDefinition::Resolved(Box::new(CBuiltinValue::Bool(false)))
                } else if let Ok(int) = ident.parse() {
                    ConstructDefinition::Resolved(Box::new(CBuiltinValue::_32U(int)))
                } else {
                    match self.lookup_ident(scope, ident) {
                        Some(id) => ConstructDefinition::Unresolved(Token::Construct(id)),
                        None => todo!("Nice error, bad ident {}", ident),
                    }
                }
            }
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
            Token::Stream {
                label: "CAPTURING",
                contents,
            } => {
                let base = contents[0].unwrap_construct();
                let mut to_set = Vec::new();
                for capture in &contents[1..] {
                    match capture {
                        Token::Plain("ALL") => to_set.append(&mut self.get_dependencies(base)),
                        &Token::Construct(capture) => {
                            to_set.append(&mut self.get_dependencies(capture))
                        }
                        _ => unreachable!(),
                    }
                }
                let mut subs = OrderedMap::new();
                for var in to_set {
                    if !var.capturing && !subs.contains_key(&var) {
                        let value = CVariable {
                            capturing: true,
                            ..var.clone()
                        };
                        let value = self.push_construct(Box::new(value));
                        subs.insert_no_replace(var, value)
                    }
                }
                ConstructDefinition::Unresolved(Token::Construct(self.substitute(base, &subs)))
            }
            Token::Stream {
                label: "substitute",
                mut contents,
            } => {
                let base = self.push_unresolved(contents.remove(0));
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
                    let sub = self.push_unresolved(sub);
                    self.constructs[sub].parent_scope = scope;
                    for (idx, dep) in deps.iter().enumerate() {
                        if self
                            .var_type_matches_var_type(&VarType::Just(sub), &dep.typee)
                            .is_guaranteed_match()
                        {
                            let dep = deps.remove(idx);
                            subs.insert_no_replace(dep, sub);
                            break;
                        }
                    }
                }
                ConstructDefinition::Resolved(Box::new(CSubstitution(base, subs)))
            }
            Token::Stream { label, .. } => todo!(
                "Nice error, token stream with label '{:?}' cannot be resolved",
                label
            ),
        }
    }
}
