use super::base::{Construct, ConstructId};
use crate::impl_any_eq_for_construct;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CShown(pub ConstructId);

impl_any_eq_for_construct!(CShown);

impl Construct for CShown {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }
}
