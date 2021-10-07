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
        ingest_map: &mut HashMap::new(),
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
    ingest_map: &'e mut HashMap<s2::ItemId, s3::ValueId>,
    variable_map: &'e mut HashMap<s2::VariableId, (s3::VariableId, s3::ValueId)>,
    variant_map: &'e mut HashMap<s2::VariantId, (s3::VariantId, s3::ValueId)>,
    path: Option<Path>,
    input: &'i s2::Environment,
    parent_scopes: Vec<&'i s2::Definitions>,
}

struct DereferencedItem {
    base: s2::ItemId,
    subs: Vec<(s3::VariableId, s3::ValueId)>,
}

impl DereferencedItem {
    fn wrapped_with(self, other: Self) -> Self {
        Self {
            base: self.base,
            subs: [self.subs, other.subs].concat(),
        }
    }
}

impl From<&s2::ItemId> for DereferencedItem {
    fn from(value: &s2::ItemId) -> Self {
        (*value).into()
    }
}

impl From<s2::ItemId> for DereferencedItem {
    fn from(value: s2::ItemId) -> Self {
        Self {
            base: value,
            subs: Vec::new(),
        }
    }
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
            s3::Value::Substituting { base, .. } => self.extract_variable(from),
            _ => None,
        }
    }

    fn resolve_variable(&mut self, item: s2::ItemId) -> Option<s3::VariableId> {
        let value = self.child().without_path().ingest(item);
        self.extract_variable(value)
    }

    fn dereference_identifier(&mut self, name: &String) -> DereferencedItem {
        for index in 0..self.parent_scopes.len() {
            let scope = &self.parent_scopes[index];
            if let Some(item) = scope.get(name) {
                let result = item.into();
                self.exclude_scopes(index);
                return result;
            }
        }
        todo!(
            "Nice error, no identifier {} in {:#?}",
            name,
            self.parent_scopes
        )
    }

    fn dereference_member(&mut self, base: s2::ItemId, name: &String) -> Option<DereferencedItem> {
        match &self.input.items[base] {
            s2::Item::Defining { base, definitions } => {
                self.parent_scopes.push(definitions);
                if let Some(result) = self.dereference_member(*base, name) {
                    return Some(result);
                }
                for (candidate, item) in definitions {
                    if candidate == name {
                        return Some(item.into());
                    }
                }
                None
            }
            s2::Item::From { base, .. } => self.dereference_member(*base, name),
            s2::Item::Identifier(ident) => {
                let ident = self.dereference_identifier(ident);
                let err = format!("No member {} in {:?}", name, ident.base);
                let member = self.dereference_member(ident.base, name).expect(&err);
                Some(member.wrapped_with(ident))
            }
            s2::Item::Member { base, name } => todo!(),
            s2::Item::Substituting {
                base,
                target,
                value,
            } => todo!(),
            _ => None,
        }
    }

    fn ingest_dereferenced(&mut self, item: DereferencedItem) -> s3::ValueId {
        let base = self.ingest(item.base);
        let mut result = base;
        for (target, value) in item.subs {
            result = self.gpv(s3::Value::Substituting {
                base: result,
                target,
                value,
            });
        }
        result
    }

    pub fn ingest(&mut self, input: s2::ItemId) -> s3::ValueId {
        if let Some(result) = self.ingest_map.get(&input) {
            return *result;
        }
        let result = match &self.input.items[input] {
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
                let (base, definitions) = (*base, definitions.clone());
                let rbase = child.ingest(base);
                // Manually insert this now so that we don't enter infinite loops when
                // processing children.
                self.ingest_map.insert(base, rbase);
                self.ingest_map.insert(input, rbase);
                let mut child = self.child().with_additional_parent_scope(&definitions);
                for (name, def) in &definitions {
                    child
                        .child()
                        .with_additional_path_component(s3::PathComponent::Member(name.clone()))
                        .ingest(*def);
                }
                // Skip adding a path for the base item again.
                return rbase;
            }
            s2::Item::From { base, value } => {
                let (base, value) = (*base, *value);
                let value = self.ingest(value);
                let variables = self.environment.dependencies(value);
                let base = self.ingest(base);
                self.environment.with_from_variables(base, &variables[..])
            }
            s2::Item::Identifier(name) => {
                let dereffed = self.dereference_identifier(name);
                self.ingest_dereferenced(dereffed)
            }
            s2::Item::Match { .. } => todo!(),
            s2::Item::Member { base, name } => {
                let dereffed = self.dereference_member(*base, name);
                match dereffed {
                    Some(dereffed) => self.ingest_dereferenced(dereffed),
                    None => todo!("Nice error, no member {} in {:?}", name, base),
                }
            }
            s2::Item::Substituting {
                base,
                target,
                value,
            } => self.ingest_substituting(base, target, value),
            s2::Item::Variant { typee, id } => {
                let (id, typee) = if let Some(vnt) = self.variant_map.get(id) {
                    *vnt
                } else {
                    let typee = self.child().without_path().ingest(*typee);
                    let new_id = self
                        .environment
                        .variants
                        .push(s3::Variant { stage2_id: *id });
                    self.variant_map.insert(*id, (new_id, typee));
                    (new_id, typee)
                };
                self.gpv(s3::Value::Variant { id, typee })
            }
        };
        self.ingest_map.insert(input, result);
        result
    }

    fn ingest_substituting(
        &mut self,
        base: &s2::ItemId,
        target: &s2::ItemId,
        value: &s2::ItemId,
    ) -> s3::ValueId {
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
}
