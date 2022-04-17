use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        definitions::{decision::DDecision, substitution::Substitutions},
        dependencies::{Dcc, DepResult, DependenciesFeature, OnlyCalledByDcc, Dependencies},
        equality::{Equal, EqualResult, EqualityFeature},
        invariants::{
            Icc, InvariantSet, InvariantSetPtr, InvariantsFeature, InvariantsResult,
            OnlyCalledByIcc,
        },
        ItemDefinition, ItemPtr, check::CheckFeature,
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

    pub fn get_value(&self) -> ItemPtr {
        self.value
    }

    pub fn get_rest(&self) -> ItemPtr {
        self.rest
    }
}

impl_any_eq_from_regular_eq!(DPopulatedStruct);

impl ItemDefinition for DPopulatedStruct {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<ItemPtr> {
        vec![self.value, self.rest]
    }
}

impl CheckFeature for DPopulatedStruct {}
impl InvariantsFeature for DPopulatedStruct {}

impl DependenciesFeature for DPopulatedStruct {
    fn get_dependencies_using_context(&self, ctx: &mut Dcc, _: OnlyCalledByDcc) -> DepResult {
        let mut deps = ctx.get_dependencies(&self.value);
        deps.append(ctx.get_dependencies(&self.rest));
        deps
    }
}

impl EqualityFeature for DPopulatedStruct {
    fn get_equality_using_context(
        &self,
        env: &mut Environment,
        self_subs: Vec<&Substitutions>,
        other: ItemPtr,
        other_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> EqualResult {
        if let Some(other) = other.downcast() {
            let other = other.clone();
            if self.label != other.label {
                return Ok(Equal::No);
            }
            Ok(Equal::and(vec![
                env.discover_equal_with_subs(
                    self.value,
                    self_subs.clone(),
                    other.value,
                    other_subs.clone(),
                    limit,
                )?,
                env.discover_equal_with_subs(self.rest, self_subs, other.rest, other_subs, limit)?,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DAtomicStructMember(pub ItemPtr, pub AtomicStructMember);

impl_any_eq_from_regular_eq!(DAtomicStructMember);

impl ItemDefinition for DAtomicStructMember {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }
}

impl CheckFeature for DAtomicStructMember {}
impl EqualityFeature for DAtomicStructMember {}

impl DependenciesFeature for DAtomicStructMember {
    fn get_dependencies_using_context(&self, ctx: &mut Dcc, _: OnlyCalledByDcc) -> DepResult {
        let base_def = match ctx.get_item_as_construct(self.0) {
            Ok(def) => &**def,
            Err(err) => return Dependencies::new_error(err),
        };
        if let Some(structt) = base_def.downcast() {
            let structt = structt.clone();
            match self.1 {
                AtomicStructMember::Label => todo!(),
                AtomicStructMember::Value => ctx.get_dependencies(structt.value),
                AtomicStructMember::Rest => ctx.get_dependencies(structt.rest),
            }
        } else {
            ctx.get_dependencies(&self.0)
        }
    }
}

impl InvariantsFeature for DAtomicStructMember {
    fn get_invariants_using_context(&self, this: &ItemPtr, ctx: &mut Icc, _: OnlyCalledByIcc) -> InvariantsResult {
        if let Ok(Some(structt)) =
            self.get_and_downcast_construct_definition::<DPopulatedStruct>(self.0)
        {
            let structt = structt.clone();
            match self.1 {
                AtomicStructMember::Label => todo!(),
                AtomicStructMember::Value => ctx.generated_invariants(structt.value),
                AtomicStructMember::Rest => ctx.generated_invariants(structt.rest),
            }
        } else {
            ctx.generated_invariants(self.0)
        }
    }
}

#[derive(Debug, Clone)]
pub struct SField(pub ItemPtr);

impl Scope for SField {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident(&self, env: &mut Environment, ident: &str) -> LookupIdentResult {
        if let Some(structt) = self.0.downcast() {
            let structt = structt.clone();
            Ok(if structt.label == ident {
                Some(structt.value)
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
        if let Some(structt) = self.0.downcast() {
            let structt = structt.clone();
            Ok(if structt.value == value && structt.label.len() > 0 {
                Some(structt.label.clone())
            } else {
                None
            })
        } else {
            unreachable!()
        }
    }

    fn local_get_invariant_sets(&self, env: &mut Environment) -> Vec<InvariantSetPtr> {
        if let Ok(Some(structt)) =
            env.get_and_downcast_construct_definition::<DPopulatedStruct>(self.0)
        {
            let structt = structt.clone();
            vec![env.generated_invariants(structt.value)]
        } else {
            unreachable!()
        }
    }

    fn parent(&self) -> Option<ItemPtr> {
        Some(self.0)
    }
}

#[derive(Debug, Clone)]
pub struct SFieldAndRest(pub ItemPtr);

fn lookup_ident_in(
    env: &mut Environment,
    ident: &str,
    inn: &DPopulatedStruct,
) -> LookupIdentResult {
    Ok(if inn.label == ident {
        Some(inn.value)
    } else if let Some(rest) = inn.rest.downcast() {
        let rest = rest.clone();
        lookup_ident_in(env, ident, &rest)?
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
    } else if let Some(rest) = inn.rest.downcast() {
        let rest = rest.clone();
        reverse_lookup_ident_in(env, value, &rest)?
    } else {
        None
    })
}

fn get_invariant_sets_in(env: &mut Environment, inn: &DPopulatedStruct) -> Vec<InvariantSetPtr> {
    let mut result = vec![env.generated_invariants(inn.value)];
    if let Ok(Some(rest)) = env.get_and_downcast_construct_definition::<DPopulatedStruct>(inn.rest)
    {
        let rest = rest.clone();
        result.append(&mut get_invariant_sets_in(env, &rest));
    }
    result
}

impl Scope for SFieldAndRest {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident(&self, env: &mut Environment, ident: &str) -> LookupIdentResult {
        if let Some(structt) = self.0.downcast() {
            let structt = structt.clone();
            lookup_ident_in(env, ident, &structt)
        } else {
            unreachable!()
        }
    }

    fn local_reverse_lookup_ident<'a, 'x>(
        &self,
        env: &'a mut Environment,
        value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        if let Some(structt) = self.0.downcast() {
            let structt = structt.clone();
            reverse_lookup_ident_in(env, value, &structt)
        } else {
            unreachable!()
        }
    }

    fn local_get_invariant_sets(&self, env: &mut Environment) -> Vec<InvariantSetPtr> {
        if let Ok(Some(structt)) =
            env.get_and_downcast_construct_definition::<DPopulatedStruct>(self.0)
        {
            let structt = structt.clone();
            get_invariant_sets_in(env, &structt)
        } else {
            unreachable!()
        }
    }

    fn parent(&self) -> Option<ItemPtr> {
        Some(self.0)
    }
}
