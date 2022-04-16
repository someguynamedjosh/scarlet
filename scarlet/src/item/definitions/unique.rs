use crate::item::{base::ItemDefinition, downcast_construct, substitution::Substitutions, ItemPtr};
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
    pub fn new(id: UniqueId) -> Self {
        Self(id)
    }
}

impl_any_eq_for_construct!(CUnique);

impl ItemDefinition for CUnique {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn get_dependencies(&self, _env: &mut Environment) -> DepResult {
        Dependencies::new()
    }

    fn discover_equality(
        &self,
        env: &mut Environment,
        self_subs: Vec<&Substitutions>,
        other_id: ItemPtr,
        other: &dyn ItemDefinition,
        other_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> DeqResult {
        Ok(if let Some(other) = downcast_construct::<Self>(other) {
            if self.0 == other.0 {
                Equal::yes()
            } else {
                Equal::No
            }
        } else {
            Equal::Unknown
        })
    }
}
