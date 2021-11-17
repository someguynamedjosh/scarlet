use super::base::{Construct, ConstructId};
use crate::impl_any_eq_for_construct;

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
}
