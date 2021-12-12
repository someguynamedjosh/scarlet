use super::{
    base::{Construct, ConstructId},
    downcast_construct,
    substitution::Substitutions,
    variable::CVariable,
};
use crate::{
    environment::Environment,
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
    pub fn new<'x>(env: &mut Environment<'x>, id: UniqueId) -> ConstructId {
        env.push_construct(Self(id))
    }
}

impl_any_eq_for_construct!(CUnique);

impl Construct for CUnique {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, _env: &mut Environment<'x>) -> Vec<CVariable> {
        vec![]
    }

    fn is_def_equal<'x>(&self, _env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        if let Some(other) = downcast_construct::<Self>(other) {
            if self.0 == other.0 {
                TripleBool::True
            } else {
                TripleBool::False
            }
        } else {
            TripleBool::Unknown
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        _substitutions: &Substitutions,
    ) -> ConstructId {
        Self::new(env, self.0)
    }
}
