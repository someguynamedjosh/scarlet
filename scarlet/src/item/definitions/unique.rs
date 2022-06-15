use std::rc::Rc;

use crate::{
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        dependencies::DependenciesFeature,
        equality::{Ecc, Equal, EqualResult, EqualSuccess, EqualityFeature, OnlyCalledByEcc},
        invariants::InvariantsFeature,
        ItemDefinition,
    },
    util::PtrExtension,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unique;
pub type UniquePtr = Rc<Unique>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DUnique(UniquePtr);

impl DUnique {
    pub fn new() -> Self {
        Self(Rc::new(Unique))
    }
}

impl_any_eq_from_regular_eq!(DUnique);

impl ItemDefinition for DUnique {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }
}

impl CheckFeature for DUnique {}
impl DependenciesFeature for DUnique {}
impl InvariantsFeature for DUnique {}

impl EqualityFeature for DUnique {
    fn get_equality_using_context(&self, ctx: &mut Ecc, _: OnlyCalledByEcc) -> EqualResult {
        let equal = if let Some(other) = ctx.other().downcast_definition::<Self>() {
            if self.0.is_same_instance_as(&other.0) {
                Equal::yes()
            } else {
                Equal::No
            }
        } else {
            Equal::Unknown
        };
        Ok(EqualSuccess {
            equal,
            unique: true,
        })
    }
}
