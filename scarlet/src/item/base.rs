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

use crate::{
    definitions::{
        builtin::{Builtin, DBuiltin},
        compound_type::DCompoundType,
        hole::DHole,
        identifier::DIdentifier,
        new_type::DNewType,
        parameter::{DParameter, ParameterPtr},
        struct_literal::DStructLiteral,
    },
    diagnostic::{Diagnostic, Position},
    environment::{Environment, ENV},
    util::PtrExtension,
};

pub struct CddContext<'a, 'b> {
    stack: &'a [*const ()],
    recursed_on: &'b mut HashSet<*const ()>,
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

pub trait ItemEnum: CycleDetectingDebug + Clone {
    fn into_any(self) -> Box<dyn Any>;
}

pub trait ResolvableItemDefinition: CycleDetectingDebug + Clone {
    fn resolve(&self, this: &ItemPtr<ResolvableItemEnum>) -> ItemPtr<ResolvedItemEnum>;
}

#[derive(Clone)]
pub enum ResolvableItemEnum {
    Builtin(DBuiltin<ResolvableItemEnum>),
    Hole(DHole<ResolvableItemEnum>),
    Identifier(DIdentifier),
    NewType(DNewType),
    Parameter(DParameter<ResolvableItemEnum>),
    StructLiteral(DStructLiteral<ResolvableItemEnum>),
}

impl ResolvableItemEnum {
    fn resolved(&self, this: &ItemPtr<ResolvableItemEnum>) -> ItemPtr<ResolvedItemEnum> {
        match self {
            Self::Builtin(x) => ResolvableItemDefinition::resolve(x, this),
            Self::Hole(x) => ResolvableItemDefinition::resolve(x, this),
            Self::Identifier(x) => ResolvableItemDefinition::resolve(x, this),
            Self::NewType(x) => ResolvableItemDefinition::resolve(x, this),
            Self::Parameter(x) => ResolvableItemDefinition::resolve(x, this),
            Self::StructLiteral(x) => ResolvableItemDefinition::resolve(x, this),
        }
    }
}

impl CycleDetectingDebug for ResolvableItemEnum {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        match self {
            Self::Builtin(x) => CycleDetectingDebug::fmt(x, f, ctx),
            Self::Hole(x) => CycleDetectingDebug::fmt(x, f, ctx),
            Self::Identifier(x) => CycleDetectingDebug::fmt(x, f, ctx),
            Self::NewType(x) => CycleDetectingDebug::fmt(x, f, ctx),
            Self::Parameter(x) => CycleDetectingDebug::fmt(x, f, ctx),
            Self::StructLiteral(x) => CycleDetectingDebug::fmt(x, f, ctx),
        }
    }
}

impl ItemEnum for ResolvableItemEnum {
    fn into_any(self) -> Box<dyn Any> {
        match self {
            Self::Builtin(x) => Box::new(x),
            Self::Hole(x) => Box::new(x),
            Self::Identifier(x) => Box::new(x),
            Self::NewType(x) => Box::new(x),
            Self::Parameter(x) => Box::new(x),
            Self::StructLiteral(x) => Box::new(x),
        }
    }
}

#[derive(Clone)]
pub enum ResolvedItemEnum {
    Builtin(DBuiltin<ResolvedItemEnum>),
    CompoundType(DCompoundType),
    Hole(DHole<ResolvedItemEnum>),
    Parameter(DParameter<ResolvedItemEnum>),
    StructLiteral(DStructLiteral<ResolvedItemEnum>),
}

impl CycleDetectingDebug for ResolvedItemEnum {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        match self {
            Self::Builtin(x) => CycleDetectingDebug::fmt(x, f, ctx),
            Self::CompoundType(x) => CycleDetectingDebug::fmt(x, f, ctx),
            Self::Hole(x) => CycleDetectingDebug::fmt(x, f, ctx),
            Self::Parameter(x) => CycleDetectingDebug::fmt(x, f, ctx),
            Self::StructLiteral(x) => CycleDetectingDebug::fmt(x, f, ctx),
        }
    }
}

impl ItemEnum for ResolvedItemEnum {
    fn into_any(self) -> Box<dyn Any> {
        match self {
            Self::Builtin(x) => Box::new(x),
            Self::CompoundType(x) => Box::new(x),
            Self::Hole(x) => Box::new(x),
            Self::Parameter(x) => Box::new(x),
            Self::StructLiteral(x) => Box::new(x),
        }
    }
}

/// Data that is stored for all items, regardless of definition.
#[derive(Debug)]
pub struct UniversalItemInfo<I: ItemEnum> {
    parent: Option<ItemPtr<I>>,
}

#[derive(Clone)]
enum LazyTransformation<I: ItemEnum> {
    None,
    Resolved,
    Reduced(HashMap<ParameterPtr<I>, LazyItemPtr<I>>),
}

#[derive(Clone)]
pub struct LazyItemPtr<I: ItemEnum> {
    base: ItemPtr<I>,
    transformation: LazyTransformation<I>,
}

impl<I: ItemEnum> PartialEq for LazyItemPtr<I> {
    fn eq(&self, other: &Self) -> bool {
        self.base.is_same_instance_as(&other.base)
    }
}

impl<I: ItemEnum> Eq for LazyItemPtr<I> {}

impl<I: ItemEnum> Hash for LazyItemPtr<I> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.hash(state);
    }
}

impl<I: ItemEnum> CycleDetectingDebug for LazyItemPtr<I> {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        CycleDetectingDebug::fmt(&self.base, f, ctx)
    }
}

impl<I: ItemEnum> Debug for LazyItemPtr<I> {
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

impl<I: ItemEnum> LazyItemPtr<I> {
    pub fn evaluate(&self) -> Result<ItemPtr<I>, Diagnostic> {
        match &self.transformation {
            LazyTransformation::None => Ok(self.base.ptr_clone()),
            // LazyTransformation::Resolved => self.base.resolve_now(&mut
            // Environment::root_query()),
            LazyTransformation::Resolved => todo!(),
            // LazyTransformation::Reduced(args) => Ok(self.base.reduce_now(args, true)),
            LazyTransformation::Reduced(args) => todo!(),
        }
    }

    pub fn ptr_clone(&self) -> LazyItemPtr<I> {
        self.clone()
    }
}

#[derive(Debug)]
pub struct Item<I: ItemEnum> {
    definition: I,
    universal_info: UniversalItemInfo<I>,
}

pub struct ItemPtr<I: ItemEnum>(Rc<RefCell<Item<I>>>, Option<Position>, bool);

impl<I: ItemEnum> Clone for ItemPtr<I> {
    fn clone(&self) -> Self {
        self.ptr_clone()
    }
}

impl<I: ItemEnum> PartialEq for ItemPtr<I> {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_same_instance_as(&other.0)
    }
}

impl<I: ItemEnum> Eq for ItemPtr<I> {}

impl<I: ItemEnum> Hash for ItemPtr<I> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.0.as_ptr().to_bits())
    }
}

impl<I: ItemEnum> CycleDetectingDebug for ItemPtr<I> {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        let ptr = self.0.as_ptr() as *const _;
        if let Some(ident) = self.reverse_lookup_identifier(self) {
            if self.lookup_identifier(&ident).unwrap().get_position() != self.get_position() {
                return write!(f, "{}", ident);
            }
        }
        if ctx.stack.contains(&ptr) {
            ctx.recursed_on.insert(ptr);
            write!(f, "@{:?}", ptr)
        } else {
            let mut new_stack = Vec::from(ctx.stack);
            new_stack.push(ptr);
            CycleDetectingDebug::fmt(
                &self.0.borrow().definition,
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

impl<I: ItemEnum> Debug for ItemPtr<I> {
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

impl<I: ItemEnum> ItemPtr<I> {
    pub fn from_definition(definition: I) -> Self {
        Self(
            Rc::new(RefCell::new(Item {
                definition,
                universal_info: UniversalItemInfo { parent: None },
            })),
            None,
            true,
        )
    }

    pub fn set_position(&mut self, position: Position) {
        self.1 = Some(position);
    }

    pub fn get_position(&self) -> Position {
        self.1.unwrap_or(Position::placeholder())
    }

    pub fn with_position(&self, position: Position) -> Self {
        Self(self.0.ptr_clone(), Some(position), true)
    }

    pub fn marked_as_non_parent(&self) -> Self {
        Self(self.0.ptr_clone(), self.1, false)
    }

    pub fn set_parent(&self, parent: ItemPtr<I>) {
        self.0.borrow_mut().universal_info.parent = Some(parent);
    }

    pub fn get_parent(&self) -> Option<ItemPtr<I>> {
        self.0.borrow().universal_info.parent.clone()
    }

    pub fn ptr_clone(&self) -> ItemPtr<I> {
        Self(self.0.ptr_clone(), self.1, true)
    }

    pub fn is_same_instance_as(&self, other: &ItemPtr<I>) -> bool {
        self.0.is_same_instance_as(&other.0)
    }

    pub fn clone_definition(&self) -> I {
        self.0.borrow().definition.clone()
    }

    pub fn downcast_definition<T>(&self) -> Option<T> {
        self.0
            .borrow()
            .definition
            .clone()
            .into_any()
            .downcast()
            .map(|x| *x)
            .ok()
    }

    pub fn get_args_if_builtin(&self, builtin: Builtin) -> Option<Vec<LazyItemPtr<I>>> {
        self.downcast_definition::<DBuiltin<I>>()
            .map(|x| {
                if x.get_builtin() == builtin {
                    Some(x.get_args().clone())
                } else {
                    None
                }
            })
            .flatten()
    }

    pub fn set_parent_recursive(&self, parent: Option<ItemPtr<I>>) {
        {
            let sb = self.0.borrow();
            println!("{:#?}", (*sb.definition).type_name());
        }
        self.0.borrow_mut().universal_info.parent = parent;
        let parent = Some(self.ptr_clone());
        let children = self.0.borrow().definition.children();
        assert!(self.0.try_borrow_mut().is_ok());
        assert!(self.0.try_borrow_mut().is_ok());
        for child in &children {
            let child = child.evaluate().unwrap();
            if child.2 {
                child.set_parent_recursive(parent.clone());
            }
        }
    }

    pub fn collect_self_and_children(&self, into: &mut Vec<ItemPtr<I>>) {
        into.push(self.ptr_clone());
        let children = self.0.borrow().definition.children();
        for child in &children {
            let child = child.evaluate().unwrap();
            if child.2 {
                child.collect_self_and_children(into);
            }
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

    pub fn resolved(&self) -> LazyItemPtr<I> {
        LazyItemPtr {
            base: self.ptr_clone(),
            transformation: LazyTransformation::Resolved,
        }
    }

    pub fn collect_constraints(&self) -> Vec<(LazyItemPtr<I>, ItemPtr<I>)> {
        self.0.borrow().definition.collect_constraints(self)
    }

    pub fn reduced(&self, args: HashMap<ParameterPtr<I>, LazyItemPtr<I>>) -> LazyItemPtr<I> {
        LazyItemPtr {
            base: self.ptr_clone(),
            transformation: LazyTransformation::Reduced(args),
        }
    }

    pub fn reduce_now(
        &self,
        args: &HashMap<ParameterPtr<I>, LazyItemPtr<I>>,
        allow_cacheing: bool,
    ) -> ItemPtr<I> {
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
        self.get_args_if_builtin(Builtin::Type).is_some()
            || self
                .downcast_definition::<DCompoundType>()
                .map(|ct| ct.is_exactly_type())
                == Some(true)
    }

    pub fn is_type_parameter(&self) -> bool {
        self.downcast_definition::<DParameter>()
            .map(|param| param.get_type().evaluate().unwrap().is_exactly_type())
            == Some(true)
    }

    /// True if this item is any type. E.G. True, Type, Int OR Null, Int WHERE
    /// IT.is_greater_than(10)
    pub fn is_a_type(&self) -> bool {
        self.downcast_definition::<DNewType>().is_some()
            || self.is_exactly_type()
            || self.is_type_parameter()
    }

    pub(crate) fn into_lazy(self) -> LazyItemPtr<I> {
        LazyItemPtr {
            base: self,
            transformation: LazyTransformation::None,
        }
    }
}

impl ItemPtr<ResolvableItemEnum> {
    pub fn lookup_identifier(&self, identifier: &str) -> Option<ItemPtr<ResolvableItemEnum>> {
        let this = self.0.borrow();
        todo!();
        // if let Some(item) =
        // this.definition.local_lookup_identifier(identifier) {
        //     Some(item)
        // } else if let Some(parent) = &this.universal_info.parent {
        //     parent.lookup_identifier(identifier)
        // } else {
        //     None
        // }
    }

    fn reverse_lookup_identifier(&self, item: &ItemPtr<ResolvableItemEnum>) -> Option<String> {
        let this = self.0.borrow();
        todo!()
        // if let Some(name) =
        // this.definition.local_reverse_lookup_identifier(item) {
        //     Some(name)
        // } else if let Some(parent) = &this.universal_info.parent {
        //     parent.reverse_lookup_identifier(item)
        // } else {
        //     None
        // }
    }
}
