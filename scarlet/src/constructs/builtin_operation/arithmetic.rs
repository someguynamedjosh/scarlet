use std::{
    convert::TryInto,
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, Div, Mul, Rem, Sub},
};

use crate::{
    constructs::{builtin_operation::BuiltinOperation, builtin_value::CBuiltinValue, ConstructId},
    environment::Environment,
    shared::AnyEq,
};

pub trait Pow<Rhs> {
    type Output;
    fn pow(self, other: Rhs) -> Self::Output;
}

macro_rules! impl_pow {
    ($ty:ty) => {
        impl Pow<$ty> for $ty {
            type Output = Self;

            fn pow(self, other: Self) -> Self {
                self.pow(other as _)
            }
        }
    };
}

impl_pow!(u8);
impl_pow!(u16);
impl_pow!(u32);
impl_pow!(u64);
impl_pow!(usize);
impl_pow!(i8);
impl_pow!(i16);
impl_pow!(i32);
impl_pow!(i64);
impl_pow!(isize);

macro_rules! arithmetic_op {
    ($Name:ident $TraitName:ident $trait_fn:ident) => {
        pub struct $Name<T: $TraitName<T, Output = T>>(PhantomData<T>);

        impl<T: $TraitName<T, Output = T> + 'static> Debug for $Name<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "OAdd")
            }
        }

        impl<T: $TraitName<T, Output = T>> $Name<T> {
            pub fn new() -> Self {
                Self(PhantomData)
            }
        }

        impl<T: $TraitName<T, Output = T> + 'static> AnyEq for $Name<T> {
            fn eq(&self, _: &dyn AnyEq) -> bool {
                true
            }
        }

        impl<T: $TraitName<T, Output = T> + 'static> BuiltinOperation for $Name<T>
        where
            CBuiltinValue: TryInto<T> + From<T>,
        {
            fn check<'x>(
                &self,
                env: &mut Environment<'x>,
                args: &[ConstructId]
            ) {
                todo!()
            }

            fn compute<'x>(
                &self,
                env: &mut Environment<'x>,
                args: &[ConstructId],
            ) -> Option<ConstructId> {
                let a1 = args[0];
                let a2 = args[1];
                if let (Some(a1), Some(a2)) = (env.get_builtin_value(a1), env.get_builtin_value(a2))
                {
                    let val = $TraitName::$trait_fn(a1, a2);
                    let con: CBuiltinValue = val.into();
                    Some(env.push_construct(Box::new(con)))
                } else {
                    None
                }
            }

            fn dyn_clone(&self) -> Box<dyn BuiltinOperation> {
                Box::new(Self::new())
            }
        }
    };
}

arithmetic_op!(OAdd Add add);
arithmetic_op!(OSub Sub sub);
arithmetic_op!(OMul Mul mul);
arithmetic_op!(ODiv Div div);
arithmetic_op!(OMod Rem rem);
arithmetic_op!(OPow Pow pow);
