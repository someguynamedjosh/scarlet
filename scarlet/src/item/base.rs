#[cfg(not(feature = "trace_borrows"))]
use std::cell::RefCell;
use std::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    rc::Rc,
};

#[cfg(feature = "trace_borrows")]
use debug_cell::{RefCell, RefMut};

use super::query::{
    AllowsChildQuery, Query, QueryContext, QueryResultCache, TypeCheckQuery, TypeQuery,
};
use crate::{diagnostic::Position, util::PtrExtension};

pub trait CycleDetectingDebug {
    fn fmt(&self, f: &mut Formatter, stack: &[*const Item]) -> fmt::Result;

    fn to_string(&self, stack: &[*const Item]) -> String {
        let mut string = String::new();
        self.fmt(&mut Formatter::new(&mut string), stack).unwrap();
        string
    }

    fn to_indented_string(&self, stack: &[*const Item], indent_size: u8) -> String {
        let mut result = self.to_string(stack);
        for _ in 0..indent_size {
            result = result.replace("\n", "\n   ");
        }
        result
    }
}

pub trait ItemDefinition: CycleDetectingDebug {
    fn recompute_type(&self, ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result;
    fn recompute_type_check(
        &self,
        ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result;
}

pub trait IntoItemPtr: ItemDefinition {
    fn into_ptr(self) -> ItemPtr;
}

impl<T: ItemDefinition + 'static> IntoItemPtr for T {
    fn into_ptr(self) -> ItemPtr {
        ItemPtr::from_definition(self)
    }
}

/// Data that is stored for all items, regardless of definition.
pub struct UniversalItemInfo {
    position: Option<Position>,
}

pub struct ItemQueryResultCaches {
    r#type: QueryResultCache<TypeQuery>,
    type_check: QueryResultCache<TypeCheckQuery>,
}

impl ItemQueryResultCaches {
    fn new() -> Self {
        Self {
            r#type: QueryResultCache::new(),
            type_check: QueryResultCache::new(),
        }
    }
}

pub struct Item {
    definition: Box<dyn ItemDefinition>,
    universal_info: UniversalItemInfo,
    query_result_caches: ItemQueryResultCaches,
}

pub struct ItemPtr(Rc<RefCell<Item>>);

impl Clone for ItemPtr {
    fn clone(&self) -> Self {
        self.ptr_clone()
    }
}

impl PartialEq for ItemPtr {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_same_instance_as(&other.0)
    }
}

impl Eq for ItemPtr {}

impl Hash for ItemPtr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.0.as_ptr().to_bits())
    }
}

impl CycleDetectingDebug for ItemPtr {
    fn fmt(&self, f: &mut Formatter, stack: &[*const Item]) -> fmt::Result {
        let ptr = self.0.as_ptr() as *const _;
        if stack.contains(&ptr) {
            write!(f, "@{:?}", ptr)
        } else {
            writeln!(f, "@{:?}", ptr)?;
            let mut new_stack = Vec::from(stack);
            new_stack.push(ptr);
            self.0.borrow().definition.fmt(f, &new_stack)
        }
    }
}

impl Debug for ItemPtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        CycleDetectingDebug::fmt(self, f, &[])
    }
}

impl ItemPtr {
    pub fn from_definition(def: impl ItemDefinition + 'static) -> Self {
        Self(Rc::new(RefCell::new(Item {
            definition: Box::new(def),
            universal_info: UniversalItemInfo { position: None },
            query_result_caches: ItemQueryResultCaches::new(),
        })))
    }

    pub fn set_position(&self, position: Position) {
        self.0.borrow_mut().universal_info.position = Some(position);
    }

    pub fn ptr_clone(&self) -> ItemPtr {
        Self(self.0.ptr_clone())
    }

    fn query<Q: Query>(
        &self,
        ctx: &mut impl AllowsChildQuery<Q>,
        get_cache: impl FnOnce(&mut ItemQueryResultCaches) -> &mut QueryResultCache<Q>,
        recompute_result: impl FnOnce(&mut QueryContext<Q>, &mut Box<dyn ItemDefinition>) -> Q::Result,
    ) -> Q::Result {
        let mut this = self.0.borrow_mut();
        let Item {
            definition,
            query_result_caches,
            ..
        } = &mut *this;
        ctx.with_child_context(|ctx| {
            ctx.get_query_result(
                self,
                |ctx| recompute_result(ctx, definition),
                get_cache(query_result_caches),
            )
        })
    }

    pub fn query_type(
        &self,
        ctx: &mut impl AllowsChildQuery<TypeQuery>,
    ) -> <TypeQuery as Query>::Result {
        self.query(
            ctx,
            |caches| &mut caches.r#type,
            |ctx, definition| definition.recompute_type(ctx),
        )
    }

    pub fn query_type_check(
        &self,
        ctx: &mut impl AllowsChildQuery<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        self.query(
            ctx,
            |caches| &mut caches.type_check,
            |ctx, definition| definition.recompute_type_check(ctx),
        )
    }
}
