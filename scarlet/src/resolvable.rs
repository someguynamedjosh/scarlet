use std::fmt::Debug;

pub trait Resolvable<'x>: Debug {

}

pub type BoxedResolvable<'x> = Box<dyn Resolvable<'x> + 'x>;

#[derive(Debug)]
pub struct RPlaceholder;

impl<'x> Resolvable<'x> for RPlaceholder {

}
