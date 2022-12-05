use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::item::ItemPtr;

pub trait Scope: Debug + DynClone {
    fn local_lookup_identifier(&self, identifier: &str) -> Option<ItemPtr>;
}

impl dyn Scope {
    pub fn dyn_clone(&self) -> Box<dyn Scope> {
        dyn_clone::clone_box(self)
    }
}

#[derive(Clone, Debug)]
pub struct SPlain;

impl Scope for SPlain {
    fn local_lookup_identifier(&self, _identifier: &str) -> Option<ItemPtr> {
        None
    }
}
