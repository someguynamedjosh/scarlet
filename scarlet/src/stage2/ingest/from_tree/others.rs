use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use crate::{
    stage1::structure::TokenTree,
    stage2::{
        ingest::top_level::IngestionContext,
        structure::{BuiltinValue, Definition, ItemId, VarType, Variable},
    },
};

impl<'e, 'x> IngestionContext<'e, 'x> {
    pub fn after_def(&mut self, body: &'x Vec<TokenTree<'x>>) -> Definition<'x> {
        if body.len() != 2 {
            todo!("Nice error");
        }

        let base = self.ingest_tree(&body[1]);
        let vals: Vec<_> = body[0]
            .unwrap_builtin("vals")
            .iter()
            .map(|tt| self.ingest_tree(tt))
            .collect();

        Definition::After { base, vals }
    }

    pub fn member_def(&mut self, body: &'x Vec<TokenTree<'x>>) -> Definition<'x> {
        assert_eq!(body.len(), 2);
        let base = &body[0];
        let base = self.ingest_tree(base);
        let name = body[1].as_token().unwrap().to_owned();
        Definition::Member(base, name)
    }

    pub fn show_def(&mut self, body: &'x Vec<TokenTree<'x>>, into: ItemId<'x>) -> Definition<'x> {
        if body.len() != 1 {
            todo!("Nice error");
        }
        let value = &body[0];
        let value = self.ingest_tree(value);
        self.env.items[value].shown_from.push(into);
        Definition::Other(value)
    }

    pub fn token_def(&mut self, token: &&str) -> Definition<'x> {
        if let Ok(num) = token.parse() {
            Definition::BuiltinValue(BuiltinValue::_32U(num))
        } else if token == &"true" {
            Definition::BuiltinValue(BuiltinValue::Bool(true))
        } else if token == &"false" {
            Definition::BuiltinValue(BuiltinValue::Bool(false))
        } else {
            let mut result = None;
            // Reversed so we search more local scopes first.
            for scope in self.in_scopes.iter().rev() {
                if let Some(id) = scope.get(token) {
                    result = Some(*id);
                    break;
                }
            }
            let id = result.expect(&format!("TODO: Nice error, bad ident {}", token));
            Definition::Other(id)
        }
    }

    pub fn variable_def(&mut self, body: &'x Vec<TokenTree<'x>>) -> Definition<'x> {
        if body.len() != 1 {
            todo!("Nice error");
        }
        let matches = &body[0];
        let typee = self.ingest_tree(matches);
        let typee = VarType::Just(typee);
        let var = self.env.vars.push(Variable { pd: PhantomData });
        Definition::Variable { var, typee }
    }
}
