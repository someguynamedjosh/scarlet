#[cfg(not(feature = "trace_borrows"))]
use std::cell::{Ref, RefCell, RefMut};
use std::{
    any::{self, Any},
    collections::{HashMap, HashSet},
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    rc::Rc,
};

#[cfg(feature = "trace_borrows")]
use debug_cell::{Ref, RefCell, RefMut};
use dyn_clone::DynClone;
use owning_ref::OwningRef;

use super::{
    query::{
        AllowsChildQuery, ChildrenQuery, FlattenQuery, ParametersQuery, Query, QueryContext,
        QueryResultCache, TypeCheckQuery, TypeQuery,
    },
    type_hints::TypeHint,
};
use crate::{
    definitions::{
        new_value::DNewValue,
        parameter::{Parameter, ParameterPtr},
    },
    diagnostic::{Diagnostic, Position},
    util::PtrExtension,
};

pub struct CddContext<'a, 'b> {
    stack: &'a [*const Item],
    recursed_on: &'b mut HashSet<*const Item>,
}

pub trait CycleDetectingDebug {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result;

    fn to_string(&self, ctx: &mut CddContext) -> String {
        let mut string = String::new();
        self.fmt(&mut Formatter::new(&mut string), ctx).unwrap();
        string
    }

    fn to_indented_string(&self, ctx: &mut CddContext, indent_size: u8) -> String {
        let mut result = self.to_string(ctx);
        for _ in 0..indent_size {
            result = result.replace("\n", "\n   ");
        }
        result
    }
}

pub trait ItemDefinition: Any + CycleDetectingDebug + DynClone {
    fn children(&self) -> Vec<ItemPtr>;
    fn collect_constraints(&self, this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)>;
    fn local_lookup_identifier(&self, identifier: &str) -> Option<ItemPtr> {
        None
    }
    fn recompute_flattened(
        &self,
        ctx: &mut QueryContext<FlattenQuery>,
    ) -> <FlattenQuery as Query>::Result {
        None
    }
    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
    ) -> <ParametersQuery as Query>::Result;
    fn recompute_type(&self, ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result;
    fn recompute_type_check(
        &self,
        ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result;
    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr;
    #[allow(unused_variables)]
    fn resolve(&mut self, this: &ItemPtr) -> Result<(), Diagnostic> {
        Ok(())
    }
}

impl dyn ItemDefinition {
    pub fn dyn_clone(&self) -> Box<Self> {
        dyn_clone::clone_box(self)
    }
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
    parent: Option<ItemPtr>,
    position: Option<Position>,
}

pub struct ItemQueryResultCaches {
    flattened: QueryResultCache<FlattenQuery>,
    r#type: QueryResultCache<TypeQuery>,
    type_check: QueryResultCache<TypeCheckQuery>,
}

impl ItemQueryResultCaches {
    fn new() -> Self {
        Self {
            flattened: QueryResultCache::new(),
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
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        let ptr = self.0.as_ptr() as *const _;
        if ctx.stack.contains(&ptr) {
            ctx.recursed_on.insert(ptr);
            write!(f, "@{:?}", ptr)
        } else {
            let mut new_stack = Vec::from(ctx.stack);
            new_stack.push(ptr);
            self.0.borrow().definition.fmt(
                f,
                &mut CddContext {
                    stack: &mut new_stack,
                    recursed_on: ctx.recursed_on,
                },
            )?;
            if ctx.recursed_on.remove(&ptr) {
                writeln!(f, "\n@{:?}", ptr)?;
            }
            Ok(())
        }
    }
}

impl Debug for ItemPtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        CycleDetectingDebug::fmt(
            self,
            f,
            &mut CddContext {
                stack: &[],
                recursed_on: &mut HashSet::new(),
            },
        )
    }
}

impl ItemPtr {
    pub fn from_definition(def: impl ItemDefinition + 'static) -> Self {
        Self(Rc::new(RefCell::new(Item {
            definition: Box::new(def),
            universal_info: UniversalItemInfo {
                parent: None,
                position: None,
            },
            query_result_caches: ItemQueryResultCaches::new(),
        })))
    }

    pub fn set_position(&self, position: Position) {
        self.0.borrow_mut().universal_info.position = Some(position);
    }

    pub fn get_position(&self) -> Position {
        self.0
            .borrow()
            .universal_info
            .position
            .unwrap_or(Position::placeholder())
    }

    pub fn set_parent(&self, parent: ItemPtr) {
        self.0.borrow_mut().universal_info.parent = Some(parent);
    }

    pub fn get_parent(&self) -> Option<ItemPtr> {
        self.0.borrow().universal_info.parent.clone()
    }

    pub fn ptr_clone(&self) -> ItemPtr {
        Self(self.0.ptr_clone())
    }

    pub fn is_same_instance_as(&self, other: &ItemPtr) -> bool {
        self.0.is_same_instance_as(&other.0)
    }

    pub fn clone_definition(&self) -> Box<dyn ItemDefinition> {
        self.0.borrow().definition.dyn_clone()
    }

    pub fn downcast_definition<D: ItemDefinition>(&self) -> Option<OwningRef<Ref<Item>, D>> {
        let r = OwningRef::new(self.0.borrow());
        r.try_map(|x| (&x.definition as &dyn Any).downcast_ref().ok_or(()))
            .ok()
    }

    pub fn is_literal_instance_of(&self, ty: &ItemPtr) -> bool {
        if let Some(value) = self.downcast_definition::<DNewValue>() {
            value.get_type().is_same_instance_as(ty)
        } else {
            false
        }
    }

    pub fn lookup_identifier(&self, identifier: &str) -> Option<ItemPtr> {
        let this = self.0.borrow();
        if let Some(item) = this.definition.local_lookup_identifier(identifier) {
            Some(item)
        } else if let Some(parent) = &this.universal_info.parent {
            parent.lookup_identifier(identifier)
        } else {
            None
        }
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

    pub fn set_parent_recursive(&self, parent: Option<ItemPtr>) {
        self.0.borrow_mut().universal_info.parent = parent;
        let parent = Some(self.ptr_clone());
        for child in &self.0.borrow().definition.children() {
            child.set_parent_recursive(parent.clone());
        }
    }

    pub fn collect_self_and_children(&self, into: &mut Vec<ItemPtr>) {
        into.push(self.ptr_clone());
        let children = self.0.borrow().definition.children();
        for child in &children {
            child.collect_self_and_children(into);
        }
        debug_assert_eq!(
            {
                let mut dd = into.clone();
                dd.dedup();
                dd
            },
            *into
        );
    }

    pub fn query_flattened(
        &self,
        ctx: &mut impl AllowsChildQuery<FlattenQuery>,
    ) -> <FlattenQuery as Query>::Result {
        self.query(
            ctx,
            |caches| &mut caches.flattened,
            |ctx, definition| definition.recompute_flattened(ctx),
        )
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

    pub fn collect_constraints(&self) -> Vec<(ItemPtr, ItemPtr)> {
        self.0.borrow().definition.collect_constraints(self)
    }

    pub(crate) fn reduce(&self, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        self.0.borrow().definition.reduce(self, args)
    }

    pub(crate) fn resolve(&self) -> Result<(), Diagnostic> {
        let borrow = self.0.borrow_mut();
        let mut def = borrow.definition.dyn_clone();
        drop(borrow);
        let result = def.resolve(self);
        self.0.borrow_mut().definition = def;
        result
    }
}
