use std::any::Any;

pub trait AnyEq: Any {
    fn eq(&self, other: &dyn AnyEq) -> bool;
}
