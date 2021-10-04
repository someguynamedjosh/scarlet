use std::collections::HashMap;

use super::structure::Environment;
use crate::{stage2::structure as s2, stage3::structure as s3};

#[derive(Debug)]
struct Context<'e, 'i> {
    environment: &'e mut s3::Environment,
    variable_map: &'e mut HashMap<s2::VariableId, s3::VariableId>,
    variant_map: &'e mut HashMap<s2::VariantId, s3::VariantId>,
    input: &'i s2::Environment,
    parent_scopes: Vec<&'i s2::Definitions>,
}

impl<'e, 'i> Context<'e, 'i> {
    pub fn child<'e2>(&'e2 mut self) -> Context<'e2, 'i> {
        Context {
            environment: self.environment,
            variable_map: self.variable_map,
            variant_map: self.variant_map,
            input: self.input,
            parent_scopes: self.parent_scopes.clone(),
        }
    }

    pub fn with_additional_parent_scope(mut self, scope: &'i s2::Definitions) -> Self {
        // Search this one before other parents.
        self.parent_scopes.insert(0, scope);
        self
    }

    pub fn resolve_ident(&self, name: &String) -> Option<&'i s2::Item> {
        for scope in &self.parent_scopes {
            if let Some(item) = scope.get(name) {
                return Some(item);
            }
        }
        None
    }

    /// Get or push value
    pub fn gpv(&mut self, value: s3::Value) -> s3::ValueId {
        self.environment.values.get_or_push(value)
    }

    fn resolve_member(&self, of: &'i s2::Item, name: &String) -> Option<&'i s2::Item> {
        match of {
            s2::Item::Defining { base, definitions } => {
                if let Some(member) = self.resolve_member(base, name) {
                    return Some(member);
                }
                for (candidate_name, value) in definitions {
                    if name == candidate_name {
                        return Some(value);
                    }
                }
                None
            }
            s2::Item::From { base, .. } => self.resolve_member(base, name),
            s2::Item::Identifier(base_name) => {
                let base = self.resolve_ident(base_name)?;
                self.resolve_member(base, name)
            }
            s2::Item::Member {
                base: base_of,
                name: base_name,
            } => {
                let base = self.resolve_member(base_of, base_name)?;
                self.resolve_member(base, name)
            }
            s2::Item::Replacing { base, replacements } => todo!(),
            _ => None,
        }
    }

    fn resolve_variable(&mut self, item: &s2::Item) -> Option<s3::VariableId> {
        
    }

    pub fn ingest(&mut self, input: &s2::Item) -> s3::ValueId {
        match input {
            s2::Item::Any { typee, id } => {
                let variable = if let Some(id) = self.variable_map.get(id) {
                    *id
                } else {
                    let typee = self.ingest(typee);
                    let variable = s3::Variable { typee };
                    let variable = self.environment.variables.push(variable);
                    self.variable_map.insert(*id, variable);
                    variable
                };
                self.gpv(s3::Value::Any(variable))
            }
            s2::Item::BuiltinOperation(op) => {
                let op = op.map(|input| self.ingest(&input));
                self.gpv(s3::Value::BuiltinOperation(op))
            }
            s2::Item::BuiltinValue(value) => self.gpv(s3::Value::BuiltinValue(*value)),
            s2::Item::Defining { base, definitions } => {
                let mut child = self.child().with_additional_parent_scope(definitions);
                child.ingest(base)
            }
            s2::Item::From { base, values } => todo!(),
            s2::Item::Identifier(name) => {
                let resolved = self
                    .resolve_ident(name)
                    .expect("TODO: Nice error, bad ident");
                self.ingest(resolved)
            }
            s2::Item::Member { base, name } => {
                let resolved = self
                    .resolve_member(base, name)
                    .expect("TODO: Nice error, bad member");
                self.ingest(resolved)
            }
            s2::Item::Replacing { base, replacements } => todo!(),
            s2::Item::Variant { typee, id } => {
                let variant = if let Some(id) = self.variant_map.get(id) {
                    *id
                } else {
                    let typee = self.ingest(typee);
                    let variant = s3::Variant { typee };
                    let variant = self.environment.variants.push(variant);
                    self.variant_map.insert(*id, variant);
                    variant
                };
                self.gpv(s3::Value::Variant(variant))
            }
        }
    }
}

impl Environment {}

pub fn ingest(s2_env: &s2::Environment, input: &s2::Item) -> (s3::Environment, s3::ValueId) {
    let mut environment = s3::Environment::new();
    // let mut variable_map = HashMap::new();
    // let mut variant_map = HashMap::new();
    let mut ctx = Context {
        environment: &mut environment,
        variable_map: &mut HashMap::new(),
        variant_map: &mut HashMap::new(),
        input: s2_env,
        parent_scopes: Vec::new(),
    };
    let value = ctx.ingest(input);
    (environment, value)
}
