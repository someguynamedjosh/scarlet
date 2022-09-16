use std::fmt::{self, Debug, Formatter};

use itertools::Itertools;

use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        dependencies::{Dcc, DepResult, DependenciesFeature, OnlyCalledByDcc},
        equality::{Ecc, Equal, EqualResult, EqualityFeature, OnlyCalledByEcc},
        invariants::{Icc, PredicateSet, PredicatesFeature, PredicatesResult, OnlyCalledByIcc},
        ContainmentType, ItemDefinition, ItemPtr,
    },
    scope::{LookupIdentResult, ReverseLookupIdentResult, Scope},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DPopulatedStruct {
    body: ItemPtr,
    label: String,
    value: ItemPtr,
}

impl DPopulatedStruct {
    pub fn new(body: ItemPtr, label: String, value: ItemPtr) -> Self {
        Self { body, label, value }
    }

    pub fn get_body(&self) -> &ItemPtr {
        &self.body
    }

    pub fn get_tail_label(&self) -> &str {
        &self.label[..]
    }

    pub fn get_tail_value(&self) -> &ItemPtr {
        &self.value
    }
}

impl_any_eq_from_regular_eq!(DPopulatedStruct);

impl ItemDefinition for DPopulatedStruct {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<(ContainmentType, ItemPtr)> {
        vec![
            (ContainmentType::Computational, self.body.ptr_clone()),
            (ContainmentType::Computational, self.value.ptr_clone()),
        ]
    }
}

impl CheckFeature for DPopulatedStruct {}
impl PredicatesFeature for DPopulatedStruct {}

impl DependenciesFeature for DPopulatedStruct {
    fn get_dependencies_using_context(
        &self,
        _this: &ItemPtr,
        ctx: &mut Dcc,
        affects_return_value: bool,
        _: OnlyCalledByDcc,
    ) -> DepResult {
        let mut deps = ctx.get_dependencies(&self.body, affects_return_value);
        deps.append(ctx.get_dependencies(&self.value, affects_return_value));
        deps
    }
}

impl EqualityFeature for DPopulatedStruct {
    fn get_equality_using_context(&self, ctx: &mut Ecc, _: OnlyCalledByEcc) -> EqualResult {
        let others = if let Some(other) = ctx.other().downcast_definition::<Self>() {
            if self.label != other.label {
                return Ok(Equal::No);
            }
            Some([other.body.ptr_clone(), other.value.ptr_clone()])
        } else {
            None
        };
        let equal = if let Some([other_body, other_value]) = others {
            Equal::and(vec![
                ctx.with_primary_and_other(self.body.ptr_clone(), other_body)
                    .get_equality_left()?,
                ctx.with_primary_and_other(self.value.ptr_clone(), other_value)
                    .get_equality_left()?,
            ])
        } else {
            Equal::Unknown
        };
        Ok(equal)
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
        _env: &mut Environment,
        value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        if let Some(structt) = self.0.downcast_definition::<DPopulatedStruct>() {
            Ok(
                if structt.value.dereference().is_same_instance_as(&value)
                    && structt.label.len() > 0
                {
                    Some(structt.label.clone())
                } else {
                    None
                },
            )
        } else {
            unreachable!()
        }
    }

    fn local_get_invariant_sets(&self) -> Vec<PredicateSet> {
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
    } else if let Some(body) = inn.body.downcast_definition::<DPopulatedStruct>() {
        lookup_ident_in(ident, &body)?
    } else {
        None
    })
}

fn reverse_lookup_ident_in(
    env: &mut Environment,
    value: ItemPtr,
    inn: &DPopulatedStruct,
) -> ReverseLookupIdentResult {
    Ok(
        if inn.value.dereference().is_same_instance_as(&value) && inn.label.len() > 0 {
            Some(inn.label.clone())
        } else if let Some(body) = inn.body.downcast_definition::<DPopulatedStruct>() {
            reverse_lookup_ident_in(env, value, &body)?
        } else {
            None
        },
    )
}

fn get_invariant_sets_in(inn: &DPopulatedStruct) -> Vec<PredicateSet> {
    let mut result = inn.value.get_invariants().into_iter().collect_vec();
    if let Some(body) = inn.body.downcast_definition::<DPopulatedStruct>() {
        result.append(&mut get_invariant_sets_in(&*body));
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

    fn local_get_invariant_sets(&self) -> Vec<PredicateSet> {
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
