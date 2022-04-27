use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    fmt::{self, Debug, Formatter},
    hash::Hash,
    rc::Rc,
};

use itertools::Itertools;
use owning_ref::{OwningRef, OwningRefMut};

use super::{
    dependencies::{DepResult, DependencyCalculationContext},
    equality::{Ecc, EqualResult, EqualityCalculationContext},
    from::create_from_dex,
    invariants::{InvariantCalculationContext, InvariantSetPtr, InvariantsResult},
    util::{RecursionPreventionStack, Stack},
};
use crate::{
    environment::Environment,
    item::{
        definitions::{
            other::DOther,
            placeholder::DPlaceholder,
            structt::{AtomicStructMember, DAtomicStructMember, DPopulatedStruct},
        },
        resolvable::{BoxedResolvable, DResolvable, Resolvable, UnresolvedItemError},
        ItemDefinition, ContainmentType,
    },
    scope::{LookupIdentResult, SRoot, Scope},
    util::{rcrc, PtrExtension},
};

pub struct ItemPtr(Rc<RefCell<Item>>);

impl Clone for ItemPtr {
    fn clone(&self) -> Self {
        self.ptr_clone()
    }
}

impl Debug for ItemPtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.debug_label())?;
        self.0.borrow().definition.fmt(f)
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
        self.0.as_ptr().to_bits().hash(state)
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
        self.0.as_ptr() == other.0.as_ptr()
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
        ctx.get_dependencies(self)
    }

    pub fn get_equality(&self, other: &Self, limit: u32) -> EqualResult {
        Ecc::get_equality(self.ptr_clone(), other.ptr_clone(), limit)
    }

    pub fn get_invariants(&self) -> InvariantsResult {
        let mut ctx = InvariantCalculationContext::new();
        ctx.get_invariants(self)
    }
}

/// Wrappers for methods that exist on Item.
impl ItemPtr {
    pub fn set_name(&self, name: String) {
        self.0.borrow_mut().name = Some(name);
    }

    pub fn downcast_definition<D: ItemDefinition>(&self) -> Option<OwningRef<Ref<'_, Item>, D>> {
        OwningRef::new(self.borrow())
            .try_map(|this| this.downcast_definition().ok_or(()))
            .ok()
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
        } else if let Some(asm) = self.downcast_definition::<DAtomicStructMember>() {
            if let Some(structt) = asm.base().downcast_definition::<DPopulatedStruct>() {
                Some(match asm.member() {
                    AtomicStructMember::Label => todo!(),
                    AtomicStructMember::Value => structt.get_value().ptr_clone(),
                    AtomicStructMember::Rest => structt.get_rest().ptr_clone(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn dereference(&self) -> ItemPtr {
        if let Some(other) = self.downcast_definition::<DOther>() {
            other.other().dereference()
        } else if let Some(asm) = self.downcast_definition::<DAtomicStructMember>() {
            if let Some(structt) = asm.base().downcast_definition::<DPopulatedStruct>() {
                match asm.member() {
                    AtomicStructMember::Label => todo!(),
                    AtomicStructMember::Value => structt.get_value().dereference(),
                    AtomicStructMember::Rest => structt.get_rest().dereference(),
                }
            } else {
                self.ptr_clone()
            }
        } else {
            self.ptr_clone()
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
            create_from_dex(env, self.ptr_clone())
        }
    }

    pub fn mark_recursion(&self) {
        let mut stack = Stack::<ItemPtr>::new();
        self.mark_recursion_impl(ContainmentType::Computational, &mut stack);
    }

    fn mark_recursion_impl(&self, containment_type: ContainmentType, stack: &mut Stack<ItemPtr>) {
        if stack.contains(self) {
            if let Some(mut other) = self.downcast_definition_mut::<DOther>() {
                other.mark_recursive(containment_type);
                return;
            }
        }
        stack.with_stack_frame(self.ptr_clone(), |stack| {
            let contents = self
                .borrow()
                .definition
                .contents()
                .into_iter()
                .map(|(t, x)| (t, x.ptr_clone()))
                .collect_vec();
            for (t, content) in contents {
                content.mark_recursion_impl(t, stack);
            }
        })
    }

    pub fn evaluation_recurses_over(&self) -> Vec<ItemPtr> {
        if let Some(other) = self.downcast_definition::<DOther>() {
            if other.is_computationally_recursive() {
                vec![other.other().ptr_clone()]
            } else {
                other.other().evaluation_recurses_over()
            }
        } else {
            let mut result = Vec::new();
            for (t, content) in self.borrow().definition.contents() {
                result.append(&mut content.evaluation_recurses_over());
            }
            result
        }
    }

    pub fn check_all(&self) {
        self.for_self_and_contents(&mut |item| {
            item.borrow().definition.check_self(item).unwrap();
        })
    }

    pub fn for_self_and_contents(&self, visitor: &mut impl FnMut(&ItemPtr)) {
        visitor(self);
        for (_, content) in self.borrow().definition.contents() {
            content.for_self_and_contents(visitor);
        }
    }

    pub fn lookup_ident(&self, ident: &str) -> LookupIdentResult {
        self.borrow().scope.lookup_ident(ident)
    }
}

#[derive(Debug)]
pub struct Item {
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
    pub fn placeholder() -> ItemPtr {
        Self::new_boxed(Box::new(DPlaceholder), Box::new(SRoot))
    }

    pub fn placeholder_with_scope(scope: Box<dyn Scope>) -> ItemPtr {
        Self::new_boxed(Box::new(DPlaceholder), scope)
    }

    pub fn new(definition: impl ItemDefinition, scope: impl Scope + 'static) -> ItemPtr {
        Self::new_boxed(Box::new(definition), Box::new(scope))
    }

    pub fn new_boxed(definition: Box<dyn ItemDefinition>, scope: Box<dyn Scope>) -> ItemPtr {
        ItemPtr(rcrc(Self {
            definition,
            scope,
            invariants: None,
            from_dex: None,
            name: None,
            show: false,
        }))
    }

    pub fn new_self_referencing<D: ItemDefinition>(
        definition: D,
        scope: Box<dyn Scope>,
        modify_self: impl FnOnce(ItemPtr, &mut D),
    ) -> ItemPtr {
        let this = ItemPtr(rcrc(Self {
            definition: Box::new(definition),
            scope,
            invariants: None,
            from_dex: None,
            name: None,
            show: false,
        }));
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
