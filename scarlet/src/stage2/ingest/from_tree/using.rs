use std::collections::HashMap;

use crate::{
    stage1::structure::{Token, TokenTree},
    stage2::{
        ingest::top_level,
        structure::{Definition, Environment, ItemId},
    },
};

pub fn ingest<'x>(
    body: &'x Vec<TokenTree<'x>>,
    env: &mut Environment<'x>,
    in_scopes: &[&HashMap<Token<'x>, ItemId<'x>>],
) -> Definition<'x> {
    assert_eq!(body.len(), 2, "TODO: Nice error.");
    let used = top_level::ingest_tree(&body[1], env, in_scopes);
    let members = env.get_members(used);
    let mut new_in_scopes = in_scopes.to_owned();
    new_in_scopes.push(&members);
    let base = top_level::ingest_tree(&body[0], env, &new_in_scopes[..]);
    Definition::Other(base)
}

impl<'x> Environment<'x> {
    pub fn get_members(&self, of: ItemId<'x>) -> HashMap<Token<'x>, ItemId<'x>> {
        match self.definition_of(of) {
            Definition::Other(other) => self.get_members(*other),
            Definition::Struct(fields) => {
                let mut result = HashMap::new();
                for field in fields {
                    if let Some(name) = &field.name {
                        result.insert(&name[..], field.value);
                    }
                }
                result
            }
            _ => todo!(),
        }
    }
}
