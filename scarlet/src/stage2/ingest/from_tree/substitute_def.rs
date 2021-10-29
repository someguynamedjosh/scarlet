use std::collections::HashMap;

use crate::{
    stage1::structure::{Token, TokenTree},
    stage2::{
        ingest::top_level::{self, IngestionContext},
        structure::{Definition, Environment, ItemId, UnresolvedSubstitution},
    },
};

impl<'e, 'x> IngestionContext<'e, 'x> {
    pub fn substitute_def(&mut self, body: &'x Vec<TokenTree<'x>>) -> Definition<'x> {
        assert_eq!(body.len(), 2);
        let base = &body[0];
        let base = self.ingest_tree(base);
        let substitution_source = body[1].unwrap_builtin("substitutions");
        let mut substitutions = Vec::new();
        for item in substitution_source {
            match item {
                TokenTree::BuiltinRule {
                    name: "target",
                    body,
                } => {
                    assert_eq!(body.len(), 2);
                    let target = &body[0];
                    let target_name = match target {
                        &TokenTree::Token(token) => Some(token),
                        _ => None,
                    };
                    let target_meaning = Some(self.ingest_tree(&target));
                    let value = self.ingest_tree(&body[1]);
                    substitutions.push(UnresolvedSubstitution {
                        target_name,
                        target_meaning,
                        value,
                    })
                }
                _ => {
                    let target_name = None;
                    let target_meaning = None;
                    let value = self.ingest_tree(item);
                    substitutions.push(UnresolvedSubstitution {
                        target_name,
                        target_meaning,
                        value,
                    })
                }
            }
        }
        Definition::UnresolvedSubstitute(base, substitutions)
    }
}
