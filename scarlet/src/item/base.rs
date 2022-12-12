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

use super::query::{
    AllowsChildQuery, ParametersQuery, Query, QueryContext, QueryResultCache, ResolveQuery,
    TypeCheckQuery, TypeQuery,
};
use crate::{
    definitions::{
        builtin::{Builtin, DBuiltin},
        compound_type::DCompoundType,
        new_value::DNewValue,
        parameter::{DParameter, ParameterPtr},
        reference::DReference,
        struct_literal::DStructLiteral,
    },
    diagnostic::{Diagnostic, Position},
    environment::{r#true, Environment, ENV, FLAG},
    item::query::QueryResult,
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

pub trait NamedAny: Any {
    fn type_name<'a>(&'a self) -> &'static str;
}

impl<T: Any> NamedAny for T {
    fn type_name<'a>(&'a self) -> &'static str {
        std::any::type_name::<T>()
    }
}

pub trait ItemDefinition: Any + NamedAny + CycleDetectingDebug + DynClone {
    fn children(&self) -> Vec<ItemPtr>;
    fn collect_constraints(&self, this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)>;
    fn local_lookup_identifier(&self, _identifier: &str) -> Option<ItemPtr> {
        None
    }
    fn local_reverse_lookup_identifier(&self, _item: &ItemPtr) -> Option<String> {
        None
    }
    fn recompute_resolved(
        &self,
        this: &ItemPtr,
        ctx: &mut QueryContext<ResolveQuery>,
    ) -> <ResolveQuery as Query>::Result;
    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
        this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result;
    fn recompute_type(&self, ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result;
    fn recompute_type_check(
        &self,
        ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result;
    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr;
}

impl dyn ItemDefinition {
    pub fn dyn_clone(&self) -> Box<Self> {
        dyn_clone::clone_box(self)
    }
}

impl Debug for dyn ItemDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} instance", self.type_name())
    }
}

pub trait IntoItemPtr: ItemDefinition {
    fn into_ptr(self) -> ItemPtr;
    fn into_ptr_mimicking(self, other: &ItemPtr) -> ItemPtr;
}

impl<T: ItemDefinition + 'static> IntoItemPtr for T {
    fn into_ptr(self) -> ItemPtr {
        ItemPtr::from_definition(self)
    }

    fn into_ptr_mimicking(self, other: &ItemPtr) -> ItemPtr {
        let mut result = ItemPtr::from_definition(self);
        if let Some(parent) = other.get_parent() {
            // result.set_parent(parent);
        }
        result.set_position(other.get_position());
        result
    }
}

/// Data that is stored for all items, regardless of definition.
#[derive(Debug)]
pub struct UniversalItemInfo {
    parent: Option<ItemPtr>,
}

#[derive(Debug, Clone)]
pub struct ItemQueryResultCaches {
    plain_reduced: Option<ItemPtr>,
    parameters: QueryResultCache<ParametersQuery>,
    resolved: QueryResultCache<ResolveQuery>,
    r#type: QueryResultCache<TypeQuery>,
    type_check: QueryResultCache<TypeCheckQuery>,
}

impl ItemQueryResultCaches {
    fn new() -> Self {
        Self {
            plain_reduced: None,
            parameters: QueryResultCache::new(),
            resolved: QueryResultCache::new(),
            r#type: QueryResultCache::new(),
            type_check: QueryResultCache::new(),
        }
    }
}

#[derive(Debug)]
pub struct Item {
    definition: Box<dyn ItemDefinition>,
    universal_info: UniversalItemInfo,
    query_result_caches: ItemQueryResultCaches,
}

pub struct ItemPtr(Rc<RefCell<Item>>, Option<Position>);

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
        // if let Some(ident) = self.reverse_lookup_identifier(self) {
        //     if self.lookup_identifier(&ident).unwrap().get_position() !=
        // self.get_position() {         return write!(f, "{}", ident);
        //     }
        // }
        if ctx.stack.contains(&ptr) {
            ctx.recursed_on.insert(ptr);
            write!(f, "{:?}", ptr)
        } else {
            let mut new_stack = Vec::from(ctx.stack);
            new_stack.push(ptr);
            CycleDetectingDebug::fmt(
                &*self.0.borrow().definition,
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
        Self(
            Rc::new(RefCell::new(Item {
                definition: Box::new(def),
                universal_info: UniversalItemInfo { parent: None },
                query_result_caches: ItemQueryResultCaches::new(),
            })),
            None,
        )
    }

    pub fn set_position(&mut self, position: Position) {
        self.1 = Some(position);
    }

    pub fn get_position(&self) -> Position {
        self.1.unwrap_or(Position::placeholder())
    }

    pub fn with_position(&self, position: Position) -> Self {
        Self(self.0.ptr_clone(), Some(position))
    }

    pub fn set_parent(&self, parent: ItemPtr) {
        self.0.borrow_mut().universal_info.parent = Some(parent);
    }

    pub fn get_parent(&self) -> Option<ItemPtr> {
        self.0.borrow().universal_info.parent.clone()
    }

    pub fn ptr_clone(&self) -> ItemPtr {
        Self(self.0.ptr_clone(), self.1)
    }

    pub fn is_same_instance_as(&self, other: &ItemPtr) -> bool {
        self.0.is_same_instance_as(&other.0)
    }

    pub fn clone_definition(&self) -> Box<dyn ItemDefinition> {
        self.0.borrow().definition.dyn_clone()
    }

    pub fn downcast_definition<D: ItemDefinition>(&self) -> Option<OwningRef<Ref<Item>, D>> {
        let r = OwningRef::new(self.0.borrow());
        r.try_map(|x| (&*x.definition as &dyn Any).downcast_ref().ok_or(()))
            .ok()
    }

    pub fn get_args_if_builtin(&self, builtin: Builtin) -> Option<Vec<ItemPtr>> {
        self.downcast_definition::<DBuiltin>()
            .map(|x| {
                if x.get_builtin() == builtin {
                    Some(x.get_args().clone())
                } else {
                    None
                }
            })
            .flatten()
    }

    pub fn is_literal_instance_of(&self, of_type: &ItemPtr) -> bool {
        if let Some(value) = self.downcast_definition::<DNewValue>() {
            let value_type = value.get_type();
            let of_type = of_type.downcast_definition::<DCompoundType>();
            let result = if let Some(of_type) = of_type {
                value_type.is_subtype_of(&*of_type)
            } else {
                false
            };
            result
        } else {
            false
        }
    }

    pub fn is_true(&self) -> bool {
        ENV.with(|env| {
            let of_type = &env
                .borrow()
                .get_language_item("True")
                .unwrap()
                .resolved()
                .dereference()
                .unwrap();
            self.is_literal_instance_of(of_type)
        })
    }

    pub fn is_false(&self) -> bool {
        ENV.with(|env| {
            self.is_literal_instance_of(
                &env.borrow()
                    .get_language_item("False")
                    .unwrap()
                    .resolved()
                    .dereference()
                    .unwrap(),
            )
        })
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

    fn reverse_lookup_identifier(&self, item: &ItemPtr) -> Option<String> {
        let this = self.0.borrow();
        if let Some(name) = this.definition.local_reverse_lookup_identifier(item) {
            Some(name)
        } else if let Some(parent) = &this.universal_info.parent {
            parent.reverse_lookup_identifier(item)
        } else {
            None
        }
    }

    pub fn address(&self) -> *const () {
        self.0.as_ptr().to_bits() as *const ()
    }

    fn query<Q: Query<Target = Self>>(
        &self,
        ctx: &mut impl AllowsChildQuery<Q>,
        get_cache: impl FnOnce(&ItemQueryResultCaches) -> &QueryResultCache<Q>,
        get_cache_mut: impl FnOnce(&mut ItemQueryResultCaches) -> &mut QueryResultCache<Q>,
        recompute_result: impl FnOnce(&mut QueryContext<Q>, &Box<dyn ItemDefinition>) -> Q::Result,
    ) -> Q::Result {
        ctx.with_child_context(|ctx| {
            let mut hasher = DefaultHasher::new();
            self.hash(&mut hasher);
            let key_hash = hasher.finish();
            if ctx.cycle_detection_stack.contains(&key_hash) {
                let result = Q::result_when_cycle_encountered(self);
                assert!(
                    !result.is_final(),
                    "Results returned when cycles are encountered should be temporary."
                );
                result
            } else {
                let this = self.0.borrow();
                if let Some(result) = get_cache(&this.query_result_caches).data.clone() {
                    result
                } else {
                    ctx.cycle_detection_stack.push(key_hash);
                    let result = recompute_result(ctx, &this.definition);
                    drop(this);
                    assert_eq!(ctx.cycle_detection_stack.pop(), Some(key_hash));
                    let mut this = self.0.borrow_mut();
                    get_cache_mut(&mut this.query_result_caches).data = Some(result.clone());
                    drop(this);
                    result
                }
            }
        })
    }

    pub fn set_parent_recursive(&self, parent: Option<ItemPtr>) {
        {
            let sb = self.0.borrow();
            // println!("{:#?}", sb.definition);
        }
        self.0.borrow_mut().universal_info.parent = parent;
        let parent = Some(self.ptr_clone());
        let children = self.0.borrow().definition.children();
        for child in &children {
            child.set_parent_recursive(parent.clone());
        }
    }

    pub fn collect_self_and_children(&self, into: &mut Vec<ItemPtr>) {
        {
            let sb = self.0.borrow();
            // println!("{:#?}", sb.definition);
        }
        into.push(self.ptr_clone());
        let children = self.0.borrow().definition.children();
        for child in &children {
            child.collect_self_and_children(into);
        }
        // debug_assert_eq!(
        //     {
        //         let mut dd = into.clone();
        //         dd.dedup();
        //         dd
        //     },
        //     *into
        // );
    }

    pub fn query_parameters(
        &self,
        ctx: &mut impl AllowsChildQuery<ParametersQuery>,
    ) -> <ParametersQuery as Query>::Result {
        self.query(
            ctx,
            |caches| &caches.parameters,
            |caches| &mut caches.parameters,
            |ctx, definition| {
                let mut result = definition.recompute_parameters(ctx, self);
                result.unmark_excluding(self);
                result
            },
        )
    }

    pub fn resolved(&self) -> ItemPtr {
        DReference::new_resolve(self.ptr_clone()).into_ptr_mimicking(self)
    }

    pub fn resolve_now(
        &self,
        ctx: &mut impl AllowsChildQuery<ResolveQuery>,
    ) -> <ResolveQuery as Query>::Result {
        let result = self.query(
            ctx,
            |caches| &caches.resolved,
            |caches| &mut caches.resolved,
            |ctx, definition| definition.recompute_resolved(self, ctx),
        )?;
        Ok(result)
    }

    pub fn query_type(
        &self,
        ctx: &mut impl AllowsChildQuery<TypeQuery>,
    ) -> <TypeQuery as Query>::Result {
        self.query(
            ctx,
            |caches| &caches.r#type,
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
            |caches| &caches.type_check,
            |caches| &mut caches.type_check,
            |ctx, definition| definition.recompute_type_check(ctx),
        )
    }

    pub fn collect_constraints(&self) -> Vec<(ItemPtr, ItemPtr)> {
        self.0.borrow().definition.collect_constraints(self)
    }

    pub fn reduced(&self, args: &HashMap<ParameterPtr, ItemPtr>, allow_cacheing: bool) -> ItemPtr {
        let this = self.0.borrow();
        if args.len() == 0 && allow_cacheing {
            if let Some(ptr) = this.query_result_caches.plain_reduced.as_ref() {
                ptr.ptr_clone()
            } else {
                let reduced = this.definition.reduce(self, args);
                drop(this);
                self.0.borrow_mut().query_result_caches.plain_reduced = Some(reduced.ptr_clone());
                reduced
            }
        } else {
            this.definition.reduce(self, args)
        }
    }

    /// True if this item is Type.
    pub fn is_exactly_type(&self) -> bool {
        self.downcast_definition::<DCompoundType>()
            .map(|ct| ct.is_exactly_type())
            == Some(true)
    }

    pub fn is_type_parameter(&self) -> bool {
        self.downcast_definition::<DParameter>()
            .map(|param| param.get_type().dereference().unwrap().is_exactly_type())
            == Some(true)
    }

    /// True if this item is any type. E.G. True, Type, Int OR Null, Int WHERE
    /// IT.is_greater_than(10)
    pub fn is_a_type(&self) -> bool {
        self.downcast_definition::<DCompoundType>().is_some()
            || self.is_exactly_type()
            || self.is_type_parameter()
    }

    pub fn dereference(&self) -> Result<ItemPtr, Diagnostic> {
        if let Some(r#ref) = self.downcast_definition::<DReference>() {
            r#ref.target()?.dereference()
        } else {
            Ok(self.ptr_clone())
        }
    }
}
