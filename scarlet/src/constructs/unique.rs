use super::{
    base::Construct,
    downcast_construct,
    substitution::{NestedSubstitutions, SubExpr, Substitutions},
    BoxedConstruct, ConstructDefinition,
};
use crate::{
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct,
    shared::{Id, Pool, TripleBool},
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

    fn get_dependencies<'x>(&self, _env: &mut Environment<'x>) -> Dependencies {
        Dependencies::new()
    }

    fn is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        SubExpr(other, _): SubExpr,
    ) -> TripleBool {
        if let Some(other) = env.get_and_downcast_construct_definition::<Self>(other) {
            if self.0 == other.0 {
                TripleBool::True
            } else {
                TripleBool::False
            }
        } else {
            TripleBool::Unknown
        }
    }
}
