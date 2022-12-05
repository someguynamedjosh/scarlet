use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use super::{builtin::DBuiltin, parameter::ParameterPtr};
use crate::item::{CddContext, CycleDetectingDebug, ItemEnum, ItemPtr, LazyItemPtr};

#[derive(Clone)]
pub struct DHole<I: ItemEnum> {
    r#type: LazyItemPtr<I>,
}

impl<I: ItemEnum> CycleDetectingDebug for DHole<I> {
    fn fmt(&self, f: &mut Formatter, _ctx: &mut CddContext) -> fmt::Result {
        write!(f, "_")
    }
}

impl<I: ItemEnum> DHole<I> {
    pub fn new(r#type: LazyItemPtr<I>) -> Self {
        Self { r#type }
    }
}
