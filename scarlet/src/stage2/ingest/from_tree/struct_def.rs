use std::collections::HashMap;

use crate::{
    stage1::structure::{Token, TokenTree},
    stage2::{
        ingest::{
            top_level::{self, IngestionContext},
            util,
        },
        structure::{Definition, Environment, ItemId, StructField},
    },
};

impl<'e, 'x> IngestionContext<'e, 'x> {
    pub fn struct_def(&mut self, body: &'x Vec<TokenTree<'x>>) -> Definition<'x> {
        let fields: Vec<_> = body.iter().map(util::maybe_target).collect();
        let ids: Vec<_> = fields
            .iter()
            .map(|target| self.begin_item(&target.value))
            .collect();
        let mut scope_map = HashMap::new();
        for (field, id) in fields.iter().zip(ids.iter()) {
            if let Some((_, name)) = &field.target {
                scope_map.insert(*name, *id);
            }
        }

        let new_scopes = util::with_extra_scope(self.in_scopes, &scope_map);
        let mut child = IngestionContext {
            env: &mut *self.env,
            in_scopes: &new_scopes,
        };
        for (field, id) in fields.iter().zip(ids.iter()) {
            child.ingest_tree_into(field.value, *id);
        }
        let mut labeled_fields = Vec::new();
        for (field, id) in fields.iter().zip(ids.iter()) {
            let name = field.target.clone().map(|x| x.1);
            labeled_fields.push(StructField { name, value: *id });
        }
        Definition::Struct(labeled_fields)
    }
}
