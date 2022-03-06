use super::{base::Construct, downcast_construct, substitution::Substitutions, ItemId};
use crate::{
    environment::{
        dependencies::{DepResult, Dependencies},
        discover_equality::{DeqPriority, DeqResult, DeqSide, Equal},
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

    fn discover_equality<'x>(
        &self,
        env: &mut Environment<'x>,
        self_subs: Vec<&Substitutions>,
        other_id: ItemId,
        other: &dyn Construct,
        other_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> DeqResult {
        Ok(if let Some(other) = downcast_construct::<Self>(other) {
            if self.0 == other.0 {
                Equal::Yes(Substitutions::new())
            } else {
                Equal::No
            }
        } else {
            Equal::Unknown
        })
    }
}
