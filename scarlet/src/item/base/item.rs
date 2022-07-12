#[cfg(not(feature = "trace_borrows"))]
use std::cell::{Ref, RefCell, RefMut};
use std::{
    any::Any,
    collections::HashSet,
    fmt::{self, Debug, Formatter},
    hash::Hash,
    rc::Rc,
    time::{Duration, Instant},
};

#[cfg(feature = "trace_borrows")]
use debug_cell::{Ref, RefCell, RefMut};
use itertools::Itertools;
use owning_ref::{OwningRef, OwningRefMut};

use super::{
    dependencies::{DepResult, DependencyCalculationContext},
    from::create_from_dex,
    invariants::{InvariantCalculationContext, InvariantSetPtr, InvariantsResult},
    util::Stack,
};
use crate::{
    diagnostic::Position,
    environment::Environment,
    item::{
        definitions::{
            builtin_function::{BuiltinFunction, DBuiltinFunction},
            other::DOther,
            placeholder::DPlaceholder,
            structt::DPopulatedStruct,
            substitution::DSubstitution,
        },
        resolvable::{DResolvable, Resolvable, UnresolvedItemError},
        ContainmentType, ItemDefinition,
    },
    scope::{LookupIdentResult, SRoot, Scope},
    util::PtrExtension,
};

pub struct ItemPtr(Rc<RefCell<Item>>);

impl Clone for ItemPtr {
    fn clone(&self) -> Self {
        self.ptr_clone()
    }
}

impl Debug for ItemPtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.debug_label())
    }
}

impl PartialEq for ItemPtr {
    fn eq(&self, other: &Self) -> bool {
        self.is_same_instance_as(other)
    }
}

impl Eq for ItemPtr {}

impl Hash for ItemPtr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).to_bits().hash(state)
    }
}

/// Basic pointer functionality
impl ItemPtr {
    pub fn debug_label(&self) -> String {
        format!(
            "{}@0x{:x}",
            self.0.borrow().name.as_ref().unwrap_or(&format!("")),
            Rc::as_ptr(&self.0).to_bits()
        )
    }

    pub fn is_same_instance_as(&self, other: &Self) -> bool {
        Rc::as_ptr(&self.0) == Rc::as_ptr(&other.0)
    }

    pub fn ptr_clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }

    #[track_caller]
    pub fn borrow(&self) -> Ref<'_, Item> {
        self.0.borrow()
    }

    #[track_caller]
    pub fn borrow_mut(&self) -> RefMut<'_, Item> {
        self.0.borrow_mut()
    }

    #[track_caller]
    pub(crate) fn redefine(&self, new_definition: Box<dyn ItemDefinition>) {
        self.borrow_mut().definition = new_definition;
    }
}

/// Wrappers for things that require contexts.
impl ItemPtr {
    pub fn get_dependencies(&self) -> DepResult {
        let mut ctx = DependencyCalculationContext::new();
        ctx.get_dependencies(self, true)
    }

    pub fn get_invariants(&self) -> InvariantsResult {
        if self.borrow().invariants.is_none() {
            let mut ctx = InvariantCalculationContext::new();
            let invs = ctx.get_invariants(self)?;
            self.borrow_mut().invariants = Some(invs);
        }
        Ok(self.borrow().invariants.as_ref().unwrap().ptr_clone())
    }
}

/// Wrappers for methods that exist on Item.
impl ItemPtr {
    pub fn set_name(&self, name: String) {
        self.0.borrow_mut().name = Some(name);
    }

    pub fn set_position(&self, position: Position) {
        self.0.borrow_mut().position = Some(position);
    }

    pub fn downcast_definition<D: ItemDefinition>(&self) -> Option<OwningRef<Ref<'_, Item>, D>> {
        OwningRef::new(self.borrow())
            .try_map(|this| this.downcast_definition().ok_or(()))
            .ok()
    }

    pub fn downcast_builtin_function_call(&self) -> Option<(BuiltinFunction, Vec<ItemPtr>)> {
        if let Some(sub) = self.downcast_definition::<DSubstitution>() {
            if let Some(bf) = sub
                .base()
                .dereference()
                .downcast_definition::<DBuiltinFunction>()
            {
                let params = sub.base().get_dependencies();
                let args = params
                    .into_variables()
                    .map(|var| sub.substitutions().get(&var.var).cloned())
                    .collect::<Option<_>>()?;
                Some((bf.get_function(), args))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn downcast_resolved_definition<D: ItemDefinition>(
        &self,
    ) -> Result<Option<OwningRef<Ref<'_, Item>, D>>, UnresolvedItemError> {
        if self.downcast_definition::<DResolvable>().is_some() {
            Err(UnresolvedItemError(self.ptr_clone()))
        } else {
            Ok(OwningRef::new(self.borrow())
                .try_map(|this| this.downcast_definition().ok_or(()))
                .ok())
        }
    }

    pub fn downcast_definition_mut<D: ItemDefinition>(
        &self,
    ) -> Option<OwningRefMut<RefMut<Item>, D>> {
        OwningRefMut::new(self.borrow_mut())
            .try_map_mut(|this| this.downcast_definition_mut().ok_or(()))
            .ok()
    }

    pub fn is_unresolved(&self) -> bool {
        self.borrow().is_unresolved()
    }
}

/// Extensions.
impl ItemPtr {
    pub fn dereference_once(&self) -> Option<ItemPtr> {
        if let Some(other) = self.downcast_definition::<DOther>() {
            Some(other.other().ptr_clone())
        } else if let Some((bf, args)) = self.downcast_builtin_function_call() {
            if let Some(structt) = args[0]
                .dereference()
                .downcast_definition::<DPopulatedStruct>()
            {
                match bf {
                    BuiltinFunction::Decision => None,
                    BuiltinFunction::Body => Some(structt.get_body().ptr_clone()),
                    BuiltinFunction::TailLabel => todo!(),
                    BuiltinFunction::TailValue => Some(structt.get_tail_value().ptr_clone()),
                    BuiltinFunction::HasTail => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn dereference(&self) -> ItemPtr {
        self.dereference_impl(&mut HashSet::new())
    }

    fn dereference_impl(&self, visited: &mut HashSet<Self>) -> ItemPtr {
        if visited.contains(self) {
            self.ptr_clone()
        } else if let Some(dereferenced) = self.dereference_once() {
            visited.insert(self.ptr_clone());
            dereferenced.dereference_impl(visited)
        } else {
            self.ptr_clone()
        }
    }

    pub fn dereference_resolved(&self) -> Result<ItemPtr, UnresolvedItemError> {
        self.dereference_resolved_impl(&mut HashSet::new())
    }

    fn dereference_resolved_impl(
        &self,
        visited: &mut HashSet<Self>,
    ) -> Result<ItemPtr, UnresolvedItemError> {
        if visited.contains(self) {
            Ok(self.ptr_clone())
        } else if let Err(err) = self.resolved() {
            Err(err)
        } else if let Some(dereferenced) = self.dereference_once() {
            visited.insert(self.ptr_clone());
            dereferenced.dereference_resolved_impl(visited)
        } else {
            Ok(self.ptr_clone())
        }
    }

    pub fn resolved(&self) -> Result<(), UnresolvedItemError> {
        if self.downcast_definition::<DResolvable>().is_some() {
            Err(UnresolvedItemError(self.ptr_clone()))
        } else {
            Ok(())
        }
    }

    pub fn clone_scope(&self) -> Box<dyn Scope> {
        self.borrow().scope.dyn_clone()
    }

    /// Returns a dex that tells you if the language item "x" could have been
    /// returned by this item.
    pub fn get_from_dex(&self, env: &Environment) -> ItemPtr {
        let ptr = self.borrow();
        if let Some(from) = &ptr.from_dex {
            return from.ptr_clone();
        } else {
            drop(ptr);
            assert!(self.0.try_borrow_mut().is_ok());
            create_from_dex(env, self.ptr_clone(), Position::placeholder())
        }
    }

    pub fn check_all(&self) {
        self.for_self_and_deep_contents(&mut |item| {
            item.borrow().definition.check_self(item).unwrap();
        })
    }

    pub fn for_self_and_deep_contents_impl(
        &self,
        visitor: &mut impl FnMut(&ItemPtr),
        visited: &mut HashSet<ItemPtr>,
    ) {
        if !visited.contains(self) {
            visited.insert(self.ptr_clone());
            visitor(self);
            for (_, content) in self.borrow().definition.contents() {
                content.for_self_and_deep_contents_impl(visitor, visited);
            }
        }
    }

    pub fn for_self_and_deep_contents(&self, visitor: &mut impl FnMut(&ItemPtr)) {
        self.for_self_and_deep_contents_impl(visitor, &mut HashSet::new())
    }

    pub fn visit_contents(&self, mut visitor: impl FnMut(&ItemPtr)) {
        for (_, content) in self.borrow().definition.contents() {
            visitor(&content);
        }
    }

    pub fn lookup_ident(&self, ident: &str) -> LookupIdentResult {
        self.borrow().scope.lookup_ident(ident)
    }
}

#[derive(Debug)]
pub struct Item {
    pub position: Option<Position>,
    pub definition: Box<dyn ItemDefinition>,
    pub scope: Box<dyn Scope>,
    pub invariants: Option<InvariantSetPtr>,
    /// A dex that, when a value is plugged in for its first dependency, will
    /// evaluate to true if and only if the plugged in value could have been
    /// generated by this construct.
    pub from_dex: Option<ItemPtr>,
    pub name: Option<String>,
    pub show: bool,
}

impl Item {
    pub fn placeholder(name: String) -> ItemPtr {
        Self::new_boxed(Box::new(DPlaceholder { name }), Box::new(SRoot))
    }

    pub fn placeholder_with_scope(name: String, scope: Box<dyn Scope>) -> ItemPtr {
        Self::new_boxed(Box::new(DPlaceholder { name }), scope)
    }

    pub fn new(definition: impl ItemDefinition, scope: impl Scope + 'static) -> ItemPtr {
        Self::new_boxed(Box::new(definition), Box::new(scope))
    }

    pub fn new_boxed(definition: Box<dyn ItemDefinition>, scope: Box<dyn Scope>) -> ItemPtr {
        ItemPtr(Rc::new(RefCell::new(Self {
            position: None,
            definition,
            scope,
            invariants: None,
            from_dex: None,
            name: None,
            show: false,
        })))
    }

    pub fn new_self_referencing<D: ItemDefinition>(
        definition: D,
        scope: Box<dyn Scope>,
        modify_self: impl FnOnce(ItemPtr, &mut D),
    ) -> ItemPtr {
        let this = ItemPtr(Rc::new(RefCell::new(Self {
            position: None,
            definition: Box::new(definition),
            scope,
            invariants: None,
            from_dex: None,
            name: None,
            show: false,
        })));
        let this2 = this.ptr_clone();
        let mut inner = this.downcast_definition_mut().unwrap();
        modify_self(this2, &mut *inner);
        drop(inner);
        this
    }

    pub fn downcast_definition<D: ItemDefinition>(&self) -> Option<&D> {
        (&*self.definition as &dyn Any).downcast_ref()
    }

    pub fn downcast_definition_mut<D: ItemDefinition>(&mut self) -> Option<&mut D> {
        (&mut *self.definition as &mut dyn Any).downcast_mut()
    }

    pub fn is_unresolved(&self) -> bool {
        self.downcast_definition::<DResolvable>().is_some()
    }
}
