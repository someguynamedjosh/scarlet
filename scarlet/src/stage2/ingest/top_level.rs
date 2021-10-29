use std::collections::{HashMap, HashSet};

use crate::{
    stage1::structure::{Module, Token, TokenTree},
    stage2::{
        ingest::util,
        structure::{Environment, ItemId, VariableId},
    },
};

pub(super) struct IngestionContext<'e, 'x> {
    pub env: &'e mut Environment<'x>,
    pub in_scopes: &'e [&'e HashMap<Token<'x>, ItemId<'x>>],
    pub without_consuming: &'e HashSet<VariableId<'x>>,
}

impl<'e, 'x> IngestionContext<'e, 'x> {
    pub(super) fn ingest_tree(&mut self, src: &'x TokenTree<'x>) -> ItemId<'x> {
        let into = self.begin_item(src);
        self.ingest_tree_into(src, into);
        into
    }

    pub(super) fn ingest_tree_into(&mut self, src: &'x TokenTree<'x>, into: ItemId<'x>) {
        let definition = self.definition_from_tree(src, into);
        self.env.items.get_mut(into).definition = Some(definition);
    }

    pub(super) fn ingest_module(&mut self, src: &'x Module, into: ItemId<'x>) {
        let mut children = Vec::new();
        for (name, module) in &src.children {
            assert_eq!(module.self_content.len(), 1);
            let src = &module.self_content[0];
            children.push((&name[..], module, self.begin_item(src)));
        }

        let scope_map: HashMap<_, _> = children.iter().map(|(name, _, id)| (*name, *id)).collect();
        let new_scopes = util::with_extra_scope(&self.in_scopes, &scope_map);

        let mut child = IngestionContext {
            env: &mut *self.env,
            in_scopes: &new_scopes,
            without_consuming: &*self.without_consuming,
        };

        assert_eq!(src.self_content.len(), 1);
        child.ingest_tree_into(&src.self_content[0], into);

        for (_, src, id) in children {
            self.ingest_module(src, id);
        }
    }
}

pub fn ingest<'x>(src: &'x Module) -> (Environment<'x>, ItemId<'x>) {
    assert_eq!(src.self_content.len(), 1);
    let mut env = Environment::new();
    let mut ctx = IngestionContext {
        env: &mut env,
        in_scopes: &[],
        without_consuming: &HashSet::new(),
    };
    let into = ctx.begin_item(&src.self_content[0]);
    ctx.ingest_module(src, into);
    (env, into)
}
