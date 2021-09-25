use std::{
    cell::RefCell,
    fmt::Debug,
    ops::{ControlFlow, FromResidual, Try},
    rc::Rc,
};

pub(crate) fn indented(source: &str) -> String {
    source.replace("\n", "\n    ")
}

pub type Rcrc<T> = Rc<RefCell<T>>;

pub fn new_rcrc<T>(value: T) -> Rcrc<T> {
    Rc::new(RefCell::new(value))
}

pub enum MaybeResult<T, E> {
    Ok(T),
    None,
    Err(E),
}

pub use MaybeResult::{Err as MErr, None as MNone, Ok as MOk};

impl<T, E: Debug> MaybeResult<T, E> {
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(t) => t,
            Self::None => panic!("Tried to unwrap a None value"),
            Self::Err(err) => panic!("Tried to unwrap an Error value: {:?}", err),
        }
    }

    pub fn into_option_or_err(self) -> Result<Option<T>, E> {
        match self {
            Self::Ok(t) => Ok(Some(t)),
            Self::None => Ok(None),
            Self::Err(e) => Err(e),
        }
    }
}

impl<T, T2, E> FromResidual<MaybeResult<T, E>> for MaybeResult<T2, E> {
    fn from_residual(residual: MaybeResult<T, E>) -> Self {
        match residual {
            MaybeResult::Ok(_v) => unreachable!(),
            MaybeResult::None => Self::None,
            MaybeResult::Err(e) => Self::Err(e),
        }
    }
}

impl<T, T2, E> FromResidual<Result<T, E>> for MaybeResult<T2, E> {
    fn from_residual(residual: Result<T, E>) -> Self {
        match residual {
            Result::Ok(_v) => unreachable!(),
            Result::Err(e) => Self::Err(e),
        }
    }
}

impl<T, E> FromResidual<E> for MaybeResult<T, E> {
    fn from_residual(residual: E) -> Self {
        Self::Err(residual)
    }
}

impl<T, E> Try for MaybeResult<T, E> {
    type Output = T;
    type Residual = Self;

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Ok(val) => ControlFlow::Continue(val),
            other => ControlFlow::Break(other),
        }
    }

    fn from_output(output: Self::Output) -> Self {
        Self::Ok(output)
    }
}
