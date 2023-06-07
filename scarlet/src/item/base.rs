#[cfg(not(feature = "trace_borrows"))]
use std::cell::{Ref, RefCell};
use std::{
    any::Any,
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    rc::Rc,
};

#[cfg(feature = "trace_borrows")]
use debug_cell::{Ref, RefCell, RefMut};
use dyn_clone::DynClone;
use owning_ref::{OwningRef, OwningRefMut};

use super::query::{Query, QueryContext, QueryResultCache};
use crate::{
    definitions::{compound_type::DCompoundType, struct_literal::DStructLiteral},
    diagnostic::{Diagnostic, Position},
    environment::{Environment, ENV, FLAG},
    item::query::QueryResult,
    shared::TripleBool,
    util::PtrExtension,
};

pub trait NamedAny: Any {
    fn type_name<'a>(&'a self) -> &'static str;
}

impl<T: Any> NamedAny for T {
    fn type_name<'a>(&'a self) -> &'static str {
        std::any::type_name::<T>()
    }
}
