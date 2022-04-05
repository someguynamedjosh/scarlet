use super::base::Construct;
use crate::{
    environment::{
        dependencies::{DepResult, Dependencies},
        Environment,
    },
    impl_any_eq_for_construct,
    shared::{Id, Pool},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unique;
pub type UniquePool = Pool<Unique, 'U'>;
pub type UniqueId = Id<'U'>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CUnique(UniqueId);

impl CUnique {
    pub fn new<'x>(id: UniqueId) -> Self {
        Self(id)
    }
}

impl_any_eq_for_construct!(CUnique);

impl Construct for CUnique {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn get_dependencies<'x>(&self, _env: &mut Environment<'x>) -> DepResult {
        Dependencies::new()
    }
}
