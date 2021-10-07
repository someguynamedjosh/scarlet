use std::collections::HashMap;

use crate::{stage2::structure as s2, stage3::structure as s3};

#[derive(Debug)]
pub(super) struct Context<'e, 'i> {
    pub environment: &'e mut s3::Environment,
    pub ingest_map: &'e mut HashMap<s2::ItemId, s3::ValueId>,
    pub variable_map: &'e mut HashMap<s2::VariableId, (s3::VariableId, s3::ValueId)>,
    pub variant_map: &'e mut HashMap<s2::VariantId, (s3::VariantId, s3::ValueId)>,
    pub path: Option<s3::Path>,
    pub input: &'i s2::Environment,
    pub parent_scopes: Vec<&'i s2::Definitions>,
}

impl<'e, 'i> Context<'e, 'i> {
    pub fn child<'e2>(&'e2 mut self) -> Context<'e2, 'i> {
        Context {
            environment: self.environment,
            ingest_map: self.ingest_map,
            variable_map: self.variable_map,
            variant_map: self.variant_map,
            path: self.path.clone(),
            input: self.input,
            parent_scopes: self.parent_scopes.clone(),
        }
    }

    pub fn with_additional_parent_scope(mut self, scope: &'i s2::Definitions) -> Self {
        // Search this one before other parents.
        self.parent_scopes.insert(0, scope);
        self
    }

    pub fn exclude_scopes(&mut self, number_of_scopes_to_exclude: usize) {
        for _ in 0..number_of_scopes_to_exclude {
            // O(n^2) but who gives a fuck anyway?
            self.parent_scopes.remove(0);
        }
    }

    pub fn with_additional_path_component(self, component: s3::PathComponent) -> Self {
        Self {
            path: self.path.map(|p| [p, vec![component]].concat()),
            ..self
        }
    }

    pub fn without_path(self) -> Self {
        Self { path: None, ..self }
    }

    /// Get or push value
    pub fn gpv(&mut self, value: s3::Value) -> s3::ValueId {
        self.environment.get_or_push_value(value)
    }

    fn extract_variable(&mut self, from: s3::ValueId) -> Option<s3::VariableId> {
        match &self.environment.values[from].value {
            s3::Value::Any { id, .. } => Some(*id),
            // TODO: This is dumb
            s3::Value::Substituting { base, .. } => {
                let base = *base;
                self.extract_variable(base)
            }
            _ => None,
        }
    }

    pub fn resolve_variable(&mut self, item: s2::ItemId) -> Option<s3::VariableId> {
        let value = self.child().without_path().ingest(item);
        self.extract_variable(value)
    }
}
