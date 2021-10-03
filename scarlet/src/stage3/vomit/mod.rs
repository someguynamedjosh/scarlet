use std::collections::HashMap;

use super::structure as s3;
use crate::stage2::structure as s2;

pub fn vomit(input: &s3::Environment, item: s3::Item) -> (s2::Environment, s2::Item) {
    let mut ctx = Context::new(input);
    let item = ctx.vomit_item(item);
    (ctx.output, item)
}

pub fn completely_vomit_item(input: &s3::Environment, item: s3::Item) -> String {
    let (env, item) = vomit(input, item);
    crate::stage2::completely_vomit_item(&env, item)
}

struct Context<'a> {
    input: &'a s3::Environment,
    output: s2::Environment,
    namespace_map: HashMap<s3::NamespaceId, s2::NamespaceId>,
    value_map: HashMap<s3::ValueId, s2::ValueId>,
    variable_map: HashMap<s3::VariableId, s2::VariableId>,
}

impl<'a> Context<'a> {
    fn new(input: &'a s3::Environment) -> Self {
        Self {
            input,
            output: s2::Environment::new(),
            namespace_map: HashMap::new(),
            value_map: HashMap::new(),
            variable_map: HashMap::new(),
        }
    }

    fn vomit_item(&mut self, item: s3::Item) -> s2::Item {
        s2::Item {
            namespace: self.vomit_namespace(item.namespace),
            value: self.vomit_value(item.value),
        }
    }

    fn get_parent(&self, child: s3::NamespaceId) -> s3::NamespaceId {
        for (candidate_id, candidate_ns) in &self.input.namespaces {
            match candidate_ns {
                s3::Namespace::Defining { base, definitions } => {
                    let mut is_parent = *base == child;
                    for (_, def) in definitions {
                        if def.namespace == child {
                            is_parent = true;
                        }
                    }
                    if is_parent {
                        return candidate_id;
                    }
                }
                s3::Namespace::Empty => (),
                s3::Namespace::Replacing { base, replacements } => {
                    if *base == child {
                        return candidate_id;
                    }
                }
                s3::Namespace::Root(item) => {
                    if item.namespace == child {
                        return candidate_id;
                    }
                }
            }
        }
        unreachable!()
    }

    fn vomit_variable(&mut self, variable_id: s3::VariableId) -> s2::VariableId {
        if let Some(result) = self.variable_map.get(&variable_id) {
            return *result;
        }
        let var_def = &self.input[variable_id];
        let definition = self.output.new_undefined_value();
        let original_type = self.vomit_value(var_def.original_type);
        let var = s2::Variable {
            definition,
            original_type,
        };
        let var_id = self.output.variables.get_or_push(var);
        let value = s2::Value::Any { variable: var_id };
        self.output.define_value(definition, value);
        var_id
    }

    fn vomit_namespace(&mut self, namespace_id: s3::NamespaceId) -> s2::NamespaceId {
        if let Some(result) = self.namespace_map.get(&namespace_id) {
            return *result;
        }
        let result_id = self.output.new_undefined_namespace();
        self.namespace_map.insert(namespace_id, result_id);
        let result = match &self.input[namespace_id] {
            s3::Namespace::Defining { base, definitions } => {
                let (base, definitions) = (*base, definitions.clone());
                let base = self.vomit_namespace(base);
                let definitions = definitions
                    .into_iter()
                    .map(|(name, item)| (name, self.vomit_item(item)))
                    .collect();
                let parent = self.get_parent(namespace_id);
                let parent = self.vomit_namespace(parent);
                s2::Namespace::Defining {
                    base,
                    definitions,
                    parent,
                }
            }
            s3::Namespace::Empty => s2::Namespace::Empty,
            s3::Namespace::Replacing { base, replacements } => {
                let (base, replacements) = (*base, replacements.clone());
                return self.vomit_namespace(base);
            }
            s3::Namespace::Root(item) => {
                let item = *item;
                let item = self.vomit_item(item);
                s2::Namespace::Root(item)
            }
        };
        self.output.define_namespace(result_id, result);
        result_id
    }

    fn vomit_value(&mut self, value_id: s3::ValueId) -> s2::ValueId {
        if let Some(result) = self.value_map.get(&value_id) {
            return *result;
        }
        let result = match &self.input[value_id] {
            s3::Value::Any { variable } => {
                let variable_id = *variable;
                let variable = self.vomit_variable(variable_id);
                s2::Value::Any { variable }
            }
            s3::Value::BuiltinOperation(_) => todo!(),
            s3::Value::BuiltinValue(value) => s2::Value::BuiltinValue(*value),
            s3::Value::From { base, variables } => todo!(),
            s3::Value::Replacing { base, replacements } => {
                let (base, replacements) = (*base, replacements.clone());
                return self.vomit_value(base);
            }
            s3::Value::Variant { variant } => todo!(),
        };
        let result_id = self.output.values.get_or_push(Some(result));
        self.value_map.insert(value_id, result_id);
        result_id
    }
}
