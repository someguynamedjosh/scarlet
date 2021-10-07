use super::Environment;
use crate::{
    shared::{Id, OpaqueClass, OrderedSet},
    stage2::{
        self,
        structure::{BuiltinOperation, BuiltinValue},
    },
    util::indented,
};

pub type Substitution = (OpaqueId, ValueId);
pub type Variables = OrderedSet<OpaqueId>;

pub type ValueId = Id<AnnotatedValue, 'L'>;
pub type OpaqueId = Id<OpaqueValue, 'O'>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    BuiltinOperation(BuiltinOperation<ValueId>),
    BuiltinValue(BuiltinValue),
    From {
        base: ValueId,
        variable: OpaqueId,
    },
    Match {
        base: ValueId,
        cases: Vec<(ValueId, ValueId)>,
    },
    Opaque {
        class: OpaqueClass,
        id: OpaqueId,
        typee: ValueId,
    },
    Substituting {
        base: ValueId,
        target: OpaqueId,
        value: ValueId,
    },
}

impl Value {
    pub fn contextual_fmt(&self, env: &Environment) -> String {
        match self {
            Value::BuiltinOperation(_) => todo!(),
            Value::BuiltinValue(val) => format!("{:?}", val),
            Value::From { base, variable } => {
                format!("From{{{:?}}}\n{}", variable, env.cfv(*base))
            }
            Value::Match { base, cases } => {
                let mut result = format!("{}\nmatch{{", env.cfv(*base));
                for (condition, value) in cases {
                    let condition = indented(&indented(&env.cfv(*condition)));
                    let value = indented(&env.cfv(*value));
                    result.push_str(&format!(
                        "\n    on{{\n        {}\n    }}\n    {}\n",
                        condition, value
                    ));
                }
                result.push_str("}");
                result
            }
            Value::Opaque { class, id, typee } => {
                format!(
                    "{}{{\n    {}\n}} at {:?}",
                    match class {
                        OpaqueClass::Variable => "any",
                        OpaqueClass::Variant => "variant_of",
                    },
                    indented(&env.cfv(*typee)),
                    id
                )
            }
            Value::Substituting {
                base,
                target,
                value,
            } => {
                format!(
                    "{}\nsubstituting{{\n    {:?} is {}\n}}",
                    env.cfv(*base),
                    target,
                    indented(&env.cfv(*value))
                )
            }
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
pub struct OpaqueValue {
    pub stage2_id: crate::stage2::structure::OpaqueId,
}
