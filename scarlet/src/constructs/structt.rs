use super::base::{Construct, ConstructId};
use crate::{environment::Environment, impl_any_eq_for_construct};

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
}
