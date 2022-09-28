use std::fmt::Debug;

use dyn_clone::DynClone;

pub trait Scope: Debug + DynClone {}

impl dyn Scope {
    pub fn dyn_clone(&self) -> Box<dyn Scope> {
        dyn_clone::clone_box(self)
    }
}
