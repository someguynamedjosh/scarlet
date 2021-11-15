use std::any::Any;

pub trait AnyEq: Any {
    fn eq(&self, other: &dyn AnyEq) -> bool;
}

#[macro_export]
macro_rules! impl_dyn_eq {
    ($On:ty) => {
        impl crate::shared::any_eq::AnyEq for $On {
            fn eq(&self, other: &dyn AnyEq) -> bool {
                (other as &dyn Any)
                    .downcast_ref::<Self>()
                    .map(|x| self == x)
                    .unwrap_or(false)
            }
        }
    };
}

// A test to make sure the macro works right.
impl_dyn_eq!(());
