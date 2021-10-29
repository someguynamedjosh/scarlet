use std::collections::HashMap;

use crate::{
    stage1::structure::{Token, TokenTree},
    stage2::{
        ingest::top_level::IngestionContext,
        structure::{Definition, Environment, ItemId},
    },
};

impl<'e, 'x> IngestionContext<'e, 'x> {
    pub fn using_def(&mut self, body: &'x Vec<TokenTree<'x>>) -> Definition<'x> {
        assert_eq!(body.len(), 2, "TODO: Nice error.");
        let used = self.ingest_tree(&body[1]);
        let members = self.env.get_members(used);

        let mut new_in_scopes = self.in_scopes.to_owned();
        new_in_scopes.push(&members);
        let mut child = IngestionContext {
            env: &mut *self.env,
            in_scopes: &new_in_scopes[..],
        };
        let base = child.ingest_tree(&body[0]);
        Definition::Other(base)
    }
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
