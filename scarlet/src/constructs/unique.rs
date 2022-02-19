use super::{base::Construct, downcast_construct, ConstructId};
use crate::{
    environment::{
        dependencies::{DepResult, Dependencies},
        discover_equality::{DeqPriority, DeqResult, DeqSide, Equal},
        sub_expr::{NestedSubstitutions, SubExpr},
        Environment,
    },
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

    fn get_dependencies<'x>(&self, _env: &mut Environment<'x>) -> DepResult {
        Dependencies::new()
    }

    fn deq_priority<'x>(&self) -> DeqPriority {
        2
    }

    fn discover_equality<'x>(
        &self,
        _env: &mut Environment<'x>,
        _other_id: ConstructId,
        other: &dyn Construct,
        _limit: u32,
        _tiebreaker: DeqSide,
    ) -> DeqResult {
        Ok(if let Some(other) = downcast_construct::<Self>(other) {
            if self.0 == other.0 {
                unreachable!("If this is the case, the two construct IDs should have already been identified as the same!");
            } else {
                Equal::No
            }
        } else {
            Equal::Unknown
        })
    }
}
