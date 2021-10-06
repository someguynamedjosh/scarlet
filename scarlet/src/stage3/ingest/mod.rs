use std::{borrow::Borrow, collections::HashMap};

use super::structure::{Environment, Path};
use crate::{
    stage2::structure::{self as s2, ItemId},
    stage3::structure as s3,
};

pub fn ingest(s2_env: &s2::Environment, input: s2::ItemId) -> (s3::Environment, s3::ValueId) {
    let mut environment = s3::Environment::new();
    // let mut variable_map = HashMap::new();
    // let mut variant_map = HashMap::new();
    let mut ctx = Context {
        environment: &mut environment,
        variable_map: &mut HashMap::new(),
        variant_map: &mut HashMap::new(),
        path: Some(Path::new()),
        input: s2_env,
        parent_scopes: Vec::new(),
    };
    let value = ctx.ingest(input);
    (environment, value)
}

#[derive(Debug)]
struct Context<'e, 'i> {
    environment: &'e mut s3::Environment,
    variable_map: &'e mut HashMap<s2::VariableId, (s3::VariableId, s3::ValueId)>,
    variant_map: &'e mut HashMap<s2::VariantId, s3::VariantId>,
    path: Option<Path>,
    input: &'i s2::Environment,
    parent_scopes: Vec<&'i s2::Definitions>,
}

struct ItemBeingResolved<'i> {
    base: &'i s2::Item,
    reps: Vec<(ItemId, ItemId)>,
}

impl<'i> ItemBeingResolved<'i> {
    fn wrapped_with(self, other: Self) -> Self {
        Self {
            base: self.base,
            reps: [self.reps, other.reps].concat(),
        }
    }
}

impl<'i> From<&'i s2::Item> for ItemBeingResolved<'i> {
    fn from(value: &'i s2::Item) -> Self {
        Self {
            base: value.borrow(),
            reps: Vec::new(),
        }
    }
}

impl<'e, 'i> Context<'e, 'i> {
    pub fn child<'e2>(&'e2 mut self) -> Context<'e2, 'i> {
        Context {
            environment: self.environment,
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
            s3::Value::Substituting { base, .. } => self.extract_variable(from),
            _ => None,
        }
    }

    fn resolve_variable(&mut self, item: s2::ItemId) -> Option<s3::VariableId> {
        let value = self.child().without_path().ingest(item);
        self.extract_variable(value)
    }

    pub fn ingest(&mut self, input: s2::ItemId) -> s3::ValueId {
        match &self.input.items[input] {
            s2::Item::Any { typee, id } => {
                let (id, typee) = if let Some(var) = self.variable_map.get(id) {
                    *var
                } else {
                    let typee = self.child().without_path().ingest(*typee);
                    let new_id = self
                        .environment
                        .variables
                        .push(s3::Variable { stage2_id: *id });
                    self.variable_map.insert(*id, (new_id, typee));
                    (new_id, typee)
                };
                self.gpv(s3::Value::Any { id, typee })
            }
            s2::Item::BuiltinOperation(op) => {
                let op = op.map(|input| self.child().without_path().ingest(input));
                self.gpv(s3::Value::BuiltinOperation(op))
            }
            s2::Item::BuiltinValue(value) => self.gpv(s3::Value::BuiltinValue(*value)),
            s2::Item::Defining { base, definitions } => {
                let mut child = self.child().with_additional_parent_scope(definitions);
                for (name, def) in definitions {
                    child
                        .child()
                        .with_additional_path_component(s3::PathComponent::Member(name.clone()))
                        .ingest(*def);
                }
                // Skip adding a path for the base item again.
                return child.ingest(*base);
            }
            s2::Item::From { base, value } => todo!(),
            s2::Item::Identifier(name) => {
                let mut result = None;
                for scope in &self.parent_scopes {
                    if let Some(item) = scope.get(name) {
                        let item = *item;
                        result = Some(self.ingest(item));
                        break;
                    }
                }
                result.expect("TOOO: Nice error")
            }
            s2::Item::Member { base, name } => {
                todo!()
            }
            s2::Item::Substituting {
                base,
                target,
                value,
            } => {
                let result = self.ingest(*base);
                let target = self
                    .resolve_variable(*target)
                    .expect("TODO: Nice error, not a variable");
                let value = self.ingest(*value);
                self.gpv(s3::Value::Substituting {
                    base: result,
                    target,
                    value,
                })
            }
            s2::Item::Variant { typee, id } => {
                todo!()
            }
        }
    }
}
