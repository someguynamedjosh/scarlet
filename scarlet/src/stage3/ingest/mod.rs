use std::{borrow::Borrow, collections::HashMap};

use super::structure::Environment;
use crate::{stage2::structure as s2, stage3::structure as s3};

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
    if let Some(start) = environment.values.first() {
        let mut id = start;
        loop {
            environment.reduce(id);
            match environment.values.next(id) {
                Some(next) => id = next,
                None => break,
            }
        }
    }
    (environment, value)
}

#[derive(Debug)]
struct Context<'e, 'i> {
    environment: &'e mut s3::Environment,
    variable_map: &'e mut HashMap<s2::VariableId, s3::VariableId>,
    variant_map: &'e mut HashMap<s2::VariantId, s3::VariantId>,
    input: &'i s2::Environment,
    parent_scopes: Vec<&'i s2::Definitions>,
}

struct ItemBeingResolved<'i> {
    base: &'i s2::Item,
    reps: Vec<s2::Substitutions>,
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
            input: self.input,
            parent_scopes: self.parent_scopes.clone(),
        }
    }

    pub fn with_additional_parent_scope(mut self, scope: &'i s2::Definitions) -> Self {
        // Search this one before other parents.
        self.parent_scopes.insert(0, scope);
        self
    }

    pub fn resolve_ident(&self, name: &String) -> Option<ItemBeingResolved<'i>> {
        for scope in &self.parent_scopes {
            if let Some(item) = scope.get(name) {
                return Some(match item {
                    s2::Item::Identifier(name) => {
                        self.resolve_ident(name).expect("TODO: Nice error").into()
                    }
                    s2::Item::Member { base, name } => self
                        .resolve_member((&**base).into(), name)
                        .expect("TODO: Nice error")
                        .into(),
                    _ => item.into(),
                });
            }
        }
        None
    }

    /// Get or push value
    pub fn gpv(&mut self, value: s3::Value) -> s3::ValueId {
        self.environment.values.get_or_push(value)
    }

    fn resolve_member<'n>(
        &self,
        of: ItemBeingResolved<'n>,
        name: &String,
    ) -> Option<ItemBeingResolved<'n>>
    where
        'i: 'n,
    {
        match of.base {
            s2::Item::Defining { base, definitions } => {
                if let Some(member) = self.resolve_member((&**base).into(), name) {
                    return Some(member.wrapped_with(of));
                }
                for (candidate_name, value) in definitions {
                    if name == candidate_name {
                        return Some(value.into());
                    }
                }
                None
            }
            s2::Item::From { base, .. } => self.resolve_member((&**base).into(), name),
            s2::Item::Identifier(base_name) => {
                let base = self.resolve_ident(base_name).expect("TODO: Nice error");
                self.resolve_member(base, name)
            }
            s2::Item::Member {
                base: base_of,
                name: base_name,
            } => {
                let base = self
                    .resolve_member((&**base_of).into(), base_name)
                    .expect("TODO: Nice error");
                self.resolve_member(base, name)
            }
            s2::Item::Substituting {
                base,
                substitutions,
            } => {
                let mut base = self.resolve_member((&**base).into(), name)?;
                base.reps.push(substitutions.clone());
                Some(base.wrapped_with(of))
            }
            _ => None,
        }
    }

    fn extract_variable(&mut self, from: s3::ValueId) -> Option<s3::VariableId> {
        match self.environment.values[from] {
            s3::Value::Any(id) => Some(id),
            // TODO: This is dumb
            s3::Value::Substituting { base, .. } => self.extract_variable(from),
            _ => None,
        }
    }

    fn resolve_variable(&mut self, item: &s2::Item) -> Option<s3::VariableId> {
        let value = self.ingest(item);
        self.extract_variable(value)
    }

    fn ingest_resolved<'n>(&mut self, resolved: ItemBeingResolved<'n>) -> s3::ValueId
    where
        'i: 'n,
    {
        let mut input = resolved.base.clone();
        for rep in resolved.reps {
            input = s2::Item::Substituting {
                base: Box::new(input),
                substitutions: rep,
            };
        }
        self.ingest(&input)
    }

    pub fn ingest<'n>(&mut self, input: &'n s2::Item) -> s3::ValueId
    where
        'i: 'n,
    {
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
                for (_, def) in definitions {
                    child.ingest(def);
                }
                child.ingest(base)
            }
            s2::Item::From { base, values } => todo!(),
            s2::Item::Identifier(name) => {
                let resolved = self
                    .resolve_ident(name)
                    .expect("TODO: Nice error, bad ident");
                self.ingest_resolved(resolved)
            }
            s2::Item::Member { base, name } => {
                let resolved = self
                    .resolve_member((&**base).into(), name)
                    .expect("TODO: Nice error, bad member");
                self.ingest_resolved(resolved)
            }
            s2::Item::Substituting {
                base,
                substitutions,
            } => {
                let base = self.ingest(base);
                let mut result = base;
                for (target, value) in substitutions.iter().rev() {
                    let target = self.resolve_variable(target).expect("TODO: Nice error");
                    let value = self.ingest(value);
                    result = self.gpv(s3::Value::Substituting { base, target, value })
                }
                result
            }
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
