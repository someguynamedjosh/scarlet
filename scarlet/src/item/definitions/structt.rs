use std::fmt::{self, Debug, Formatter};

use itertools::Itertools;

use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        definitions::{decision::DDecision, substitution::Substitutions},
        dependencies::{Dcc, DepResult, Dependencies, DependenciesFeature, OnlyCalledByDcc},
        equality::{Ecc, Equal, EqualResult, EqualityFeature, EqualityTestSide, OnlyCalledByEcc},
        invariants::{
            Icc, InvariantSet, InvariantSetPtr, InvariantsFeature, InvariantsResult,
            OnlyCalledByIcc,
        },
        ContainmentType, ItemDefinition, ItemPtr,
    },
    scope::{
        LookupIdentResult, LookupInvariantError, LookupInvariantResult, ReverseLookupIdentResult,
        SPlain, Scope,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DPopulatedStruct {
    label: String,
    value: ItemPtr,
    rest: ItemPtr,
}

impl DPopulatedStruct {
    pub fn new(label: String, value: ItemPtr, rest: ItemPtr) -> Self {
        Self { label, value, rest }
    }

    pub fn get_label(&self) -> &str {
        &self.label[..]
    }

    pub fn get_value(&self) -> &ItemPtr {
        &self.value
    }

    pub fn get_rest(&self) -> &ItemPtr {
        &self.rest
    }
}

impl_any_eq_from_regular_eq!(DPopulatedStruct);

impl ItemDefinition for DPopulatedStruct {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<(ContainmentType, &ItemPtr)> {
        vec![
            (ContainmentType::Computational, &self.value),
            (ContainmentType::Computational, &self.rest),
        ]
    }
}

impl CheckFeature for DPopulatedStruct {}
impl InvariantsFeature for DPopulatedStruct {}

impl DependenciesFeature for DPopulatedStruct {
    fn get_dependencies_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Dcc,
        affects_return_value: bool,
        _: OnlyCalledByDcc,
    ) -> DepResult {
        let mut deps = ctx.get_dependencies(&self.value, affects_return_value);
        deps.append(ctx.get_dependencies(&self.rest, affects_return_value));
        deps
    }
}

impl EqualityFeature for DPopulatedStruct {
    fn get_equality_using_context(&self, ctx: &mut Ecc, _: OnlyCalledByEcc) -> EqualResult {
        let others = if let Some(other) = ctx.other().downcast_definition::<Self>() {
            if self.label != other.label {
                return Ok(Equal::No);
            }
            Some([other.value.ptr_clone(), other.rest.ptr_clone()])
        } else {
            None
        };
        if let Some([other_value, other_rest]) = others {
            Ok(Equal::and(vec![
                ctx.with_primary_and_other(self.value.ptr_clone(), other_value)
                    .get_equality_left()?,
                ctx.with_primary_and_other(self.rest.ptr_clone(), other_rest)
                    .get_equality_left()?,
            ]))
        } else {
            Ok(Equal::Unknown)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtomicStructMember {
    Label,
    Value,
    Rest,
}

#[derive(Clone, PartialEq, Eq)]
pub struct DAtomicStructMember(ItemPtr, AtomicStructMember);

impl Debug for DAtomicStructMember {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "DAtomicStructMember({:?})", self.1)
    }
}

impl DAtomicStructMember {
    pub fn new(base: ItemPtr, member: AtomicStructMember) -> Self {
        Self(base, member)
    }

    pub fn base(&self) -> &ItemPtr {
        &self.0
    }

    pub fn member(&self) -> AtomicStructMember {
        self.1
    }
}

impl_any_eq_from_regular_eq!(DAtomicStructMember);

impl ItemDefinition for DAtomicStructMember {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<(ContainmentType, &ItemPtr)> {
        vec![(ContainmentType::Computational, &self.0)]
    }
}

impl CheckFeature for DAtomicStructMember {}
impl EqualityFeature for DAtomicStructMember {}

impl DependenciesFeature for DAtomicStructMember {
    fn get_dependencies_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Dcc,
        affects_return_value: bool,
        _: OnlyCalledByDcc,
    ) -> DepResult {
        if let Some(structt) = self
            .0
            .dereference()
            .downcast_definition::<DPopulatedStruct>()
        {
            match self.1 {
                AtomicStructMember::Label => todo!(),
                AtomicStructMember::Value => ctx.get_dependencies(&structt.value, affects_return_value),
                AtomicStructMember::Rest => ctx.get_dependencies(&structt.rest, affects_return_value),
            }
        } else {
            ctx.get_dependencies(&self.0, affects_return_value)
        }
    }
}

impl InvariantsFeature for DAtomicStructMember {
    fn get_invariants_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        if let Some(structt) = self
            .0
            .dereference()
            .downcast_definition::<DPopulatedStruct>()
        {
            match self.1 {
                AtomicStructMember::Label => todo!(),
                AtomicStructMember::Value => structt.value.get_invariants(),
                AtomicStructMember::Rest => structt.rest.get_invariants(),
            }
        } else {
            self.0.get_invariants()
        }
    }
}

#[derive(Debug, Clone)]
pub struct SField(pub ItemPtr);

impl Scope for SField {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident(&self, ident: &str) -> LookupIdentResult {
        if let Some(structt) = self.0.downcast_definition::<DPopulatedStruct>() {
            Ok(if structt.label == ident {
                Some(structt.value.ptr_clone())
            } else {
                None
            })
        } else {
            unreachable!()
        }
    }

    fn local_reverse_lookup_ident(
        &self,
        env: &mut Environment,
        value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        if let Some(structt) = self.0.downcast_definition::<DPopulatedStruct>() {
            Ok(if structt.value == value && structt.label.len() > 0 {
                Some(structt.label.clone())
            } else {
                None
            })
        } else {
            unreachable!()
        }
    }

    fn local_get_invariant_sets(&self) -> Vec<InvariantSetPtr> {
        if let Some(structt) = self.0.downcast_definition::<DPopulatedStruct>() {
            structt.value.get_invariants().into_iter().collect()
        } else {
            unreachable!()
        }
    }

    fn parent(&self) -> Option<ItemPtr> {
        Some(self.0.ptr_clone())
    }
}

#[derive(Debug, Clone)]
pub struct SFieldAndRest(pub ItemPtr);

fn lookup_ident_in(ident: &str, inn: &DPopulatedStruct) -> LookupIdentResult {
    Ok(if inn.label == ident {
        Some(inn.value.ptr_clone())
    } else if let Some(rest) = inn.rest.downcast_definition::<DPopulatedStruct>() {
        lookup_ident_in(ident, &rest)?
    } else {
        None
    })
}

fn reverse_lookup_ident_in(
    env: &mut Environment,
    value: ItemPtr,
    inn: &DPopulatedStruct,
) -> ReverseLookupIdentResult {
    Ok(if inn.value == value && inn.label.len() > 0 {
        Some(inn.label.clone())
    } else if let Some(rest) = inn.rest.downcast_definition::<DPopulatedStruct>() {
        reverse_lookup_ident_in(env, value, &rest)?
    } else {
        None
    })
}

fn get_invariant_sets_in(inn: &DPopulatedStruct) -> Vec<InvariantSetPtr> {
    let mut result = inn.value.get_invariants().into_iter().collect_vec();
    if let Some(rest) = inn.rest.downcast_definition::<DPopulatedStruct>() {
        result.append(&mut get_invariant_sets_in(&*rest));
    }
    result
}

impl Scope for SFieldAndRest {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident(&self, ident: &str) -> LookupIdentResult {
        if let Some(structt) = self.0.downcast_definition::<DPopulatedStruct>() {
            lookup_ident_in(ident, &structt)
        } else {
            unreachable!()
        }
    }

    fn local_reverse_lookup_ident<'a, 'x>(
        &self,
        env: &'a mut Environment,
        value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        if let Some(structt) = self.0.downcast_definition() {
            reverse_lookup_ident_in(env, value, &structt)
        } else {
            unreachable!()
        }
    }

    fn local_get_invariant_sets(&self) -> Vec<InvariantSetPtr> {
        if let Some(structt) = self.0.downcast_definition() {
            get_invariant_sets_in(&structt)
        } else {
            unreachable!()
        }
    }

    fn parent(&self) -> Option<ItemPtr> {
        Some(self.0.ptr_clone())
    }
}
