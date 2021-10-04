use super::structure::{Environment, PathComponent, ValueId, VariableId};
use crate::{
    stage2::structure::{Item, Substitutions},
    stage3::structure::Value,
};

impl Environment {
    fn path_as_item(path: &[PathComponent]) -> Item {
        let mut result = Item::Identifier(format!("ROOT"));
        for component in path {
            match component {
                PathComponent::Member(name) => {
                    result = Item::Member {
                        base: Box::new(result),
                        name: name.clone(),
                    }
                }
            }
        }
        result
    }

    fn get_value_name(&self, value: ValueId) -> Option<Item> {
        self.paths
            .get(value)
            .iter()
            .min_by_key(|i| i.len())
            .map(|path| Self::path_as_item(path))
    }

    fn get_variable_name(&self, variable: VariableId) -> Option<Item> {
        self.values
            .iter()
            .filter_map(|(id, value)| match value {
                Value::Any { id: variable, .. } => Some(id),
                _ => None,
            })
            .flat_map(|value| self.paths.get(value).iter())
            .min_by_key(|i| i.len())
            .map(|path| Self::path_as_item(path))
    }

    fn get_value_name_or_code(&self, value: ValueId) -> Item {
        if let Some(name) = self.get_value_name(value) {
            name
        } else {
            self.get_value_code(value)
        }
    }

    fn get_value_code(&self, value: ValueId) -> Item {
        let value = if let Some(rep) = self.reduce_cache.get(&value) {
            *rep
        } else {
            value
        };
        match &self.values[value] {
            Value::Any { typee, id } => Item::Any {
                typee: Box::new(self.get_value_name_or_code(*typee)),
                id: self.variables[*id].stage2_id,
            },
            Value::BuiltinOperation(_) => todo!(),
            Value::BuiltinValue(value) => Item::BuiltinValue(*value),
            Value::From { base, variable } => todo!(),
            Value::Substituting {
                base,
                target,
                value,
            } => {
                let base = Box::new(self.get_value_name_or_code(*base));
                let mut substitutions = Substitutions::new();
                let target = self.get_variable_name(*target).unwrap();
                let value = self.get_value_name_or_code(*value);
                substitutions.push((target, value));
                Item::Substituting {
                    base,
                    substitutions,
                }
            }
            Value::Variant(_) => todo!(),
        }
    }

    pub fn vomit(&self, value: ValueId) -> Item {
        self.get_value_code(value)
    }
}
