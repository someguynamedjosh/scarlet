pub mod base {
    use std::{any::Any, fmt::Debug};

    use crate::shared::AnyEq;

    pub type BoxedConstruct<'x> = Box<dyn Construct<'x>>;

    pub trait Construct<'x>: Any + Debug + AnyEq {}
}
