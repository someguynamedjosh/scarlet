use super::{
    as_struct,
    base::{Construct, ConstructId},
    substitution::Substitutions,
    variable::CVariable,
};
use crate::{environment::Environment, impl_any_eq_for_construct, shared::TripleBool};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructField {
    pub name: Option<String>,
    pub value: ConstructId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CStruct(pub Vec<StructField>);

impl_any_eq_for_construct!(CStruct);

impl Construct for CStruct {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        let mut deps = Vec::new();
        for field in &self.0 {
            deps.append(&mut env.get_dependencies(field.value));
        }
        deps
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>, _self_id: ConstructId) -> ConstructId {
        let mut fields = Vec::new();
        for field in &self.0 {
            fields.push(StructField {
                name: field.name.clone(),
                value: env.reduce(field.value),
            });
        }
        env.push_construct(Box::new(Self(fields)))
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let fields = self
            .0
            .iter()
            .map(|field| StructField {
                name: field.name.clone(),
                value: env.substitute(field.value, substitutions),
            })
            .collect();
        env.push_construct(Box::new(Self(fields)))
    }
}
