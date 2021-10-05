use super::Environment;
use crate::{
    shared::{Id, OrderedMap, OrderedSet},
    stage2::{
        self,
        structure::{BuiltinOperation, BuiltinValue},
    },
    util::indented,
};

pub type Substitution = (VariableId, ValueId);
pub type Variables = OrderedSet<VariableId>;

pub type ValueId = Id<AnnotatedValue, 'L'>;
pub type VariableId = Id<Variable, 'V'>;
pub type VariantId = Id<Variant, 'T'>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    Any {
        id: VariableId,
        typee: ValueId,
    },
    BuiltinOperation(BuiltinOperation<ValueId>),
    BuiltinValue(BuiltinValue),
    From {
        base: ValueId,
        variable: VariableId,
    },
    Substituting {
        base: ValueId,
        target: VariableId,
        value: ValueId,
    },
    Variant(VariantId),
}

impl Value {
    pub fn contextual_fmt(&self, env: &Environment) -> String {
        match self {
            Value::Any { id, typee } => {
                format!("any{{\n    {}\n}} at {:?}", indented(&env.cfv(*typee)), id)
            }
            Value::BuiltinOperation(_) => todo!(),
            Value::BuiltinValue(val) => format!("{:?}", val),
            Value::From { base, variable } => {
                format!("{}\n    From{{{:?}}}", env.cfv(*base), variable)
            }
            Value::Substituting {
                base,
                target,
                value,
            } => {
                format!(
                    "{}\n    substituting{{\n        {:?} is {}\n    }}",
                    env.cfv(*base),
                    target,
                    indented(&indented(&env.cfv(*value)))
                )
            }
            Value::Variant(_) => todo!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnnotatedValue {
    pub cached_type: Option<ValueId>,
    pub cached_reduction: Option<ValueId>,
    pub defined_at: Option<stage2::structure::ItemId>,
    pub value: Value,
}

impl AnnotatedValue {
    pub fn contextual_fmt(&self, env: &Environment) -> String {
        let mut result = self.value.contextual_fmt(env);
        if let Some(typee) = self.cached_type {
            result.push_str(&format!("\n:{}", env.cfv(typee)));
        }
        if let Some(definition) = self.defined_at {
            result.push_str(&format!("\ndefined at {:?}", definition));
        }
        if let Some(value) = self.cached_reduction {
            result.push_str(&format!("\nreduces to {:?}", value));
        }
        result
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub stage2_id: crate::stage2::structure::VariableId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub typee: ValueId,
}
