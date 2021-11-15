pub mod base {
    use std::{any::Any, fmt::Debug};

    use crate::shared::any_eq::AnyEq;

    pub type BoxedConstruct = Box<dyn Construct>;

    pub trait Construct: Any + Debug + AnyEq {}
}
