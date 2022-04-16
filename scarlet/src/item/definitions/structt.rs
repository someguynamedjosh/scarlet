use crate::item::{
    as_struct, base::ItemPtr, downcast_construct, substitution::Substitutions, ItemDefinition,
    GenInvResult,
};
use crate::{
    environment::{
        dependencies::{DepResult, Dependencies},
        discover_equality::{DeqResult, DeqSide, Equal},
        invariants::InvariantSetPtr,
        Environment,
    },
    impl_any_eq_for_construct,
    scope::{
        LookupIdentResult, LookupInvariantError, LookupInvariantResult, ReverseLookupIdentResult,
        Scope,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CPopulatedStruct {
    label: String,
    value: ItemPtr,
    rest: ItemPtr,
}

impl CPopulatedStruct {
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

impl_any_eq_for_construct!(CPopulatedStruct);

impl ItemDefinition for CPopulatedStruct {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<ItemPtr> {
        vec![self.value, self.rest]
    }

    fn get_dependencies(&self, env: &mut Environment) -> DepResult {
        let mut deps = env.get_dependencies(self.value);
        deps.append(env.get_dependencies(self.rest));
        deps
    }

    fn discover_equality(
        &self,
        env: &mut Environment,
        self_subs: Vec<&Substitutions>,
        other_id: ItemPtr,
        other: &dyn ItemDefinition,
        other_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> DeqResult {
        if let Some(other) = downcast_construct::<Self>(other) {
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
pub struct CAtomicStructMember(pub ItemPtr, pub AtomicStructMember);

impl_any_eq_for_construct!(CAtomicStructMember);

impl ItemDefinition for CAtomicStructMember {
    fn dyn_clone(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn generated_invariants(&self, _this: ItemPtr, env: &mut Environment) -> GenInvResult {
        if let Ok(Some(structt)) =
            env.get_and_downcast_construct_definition::<CPopulatedStruct>(self.0)
        {
            let structt = structt.clone();
            match self.1 {
                AtomicStructMember::Label => todo!(),
                AtomicStructMember::Value => env.generated_invariants(structt.value),
                AtomicStructMember::Rest => env.generated_invariants(structt.rest),
            }
        } else {
            env.generated_invariants(self.0)
        }
    }

    fn get_dependencies(&self, env: &mut Environment) -> DepResult {
        let base_def = match env.get_item_as_construct(self.0) {
            Ok(def) => &**def,
            Err(err) => return Dependencies::new_error(err),
        };
        if let Some(structt) = as_struct(base_def) {
            let structt = structt.clone();
            match self.1 {
                AtomicStructMember::Label => todo!(),
                AtomicStructMember::Value => env.get_dependencies(structt.value),
                AtomicStructMember::Rest => env.get_dependencies(structt.rest),
            }
        } else {
            env.get_dependencies(self.0)
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
        if let Some(structt) = as_struct(&**env.get_item_as_construct(self.0)?) {
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
        if let Some(structt) = as_struct(&**env.get_item_as_construct(self.0)?) {
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
            env.get_and_downcast_construct_definition::<CPopulatedStruct>(self.0)
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
    inn: &CPopulatedStruct,
) -> LookupIdentResult {
    Ok(if inn.label == ident {
        Some(inn.value)
    } else if let Some(rest) = as_struct(&**env.get_item_as_construct(inn.rest)?) {
        let rest = rest.clone();
        lookup_ident_in(env, ident, &rest)?
    } else {
        None
    })
}

fn reverse_lookup_ident_in(
    env: &mut Environment,
    value: ItemPtr,
    inn: &CPopulatedStruct,
) -> ReverseLookupIdentResult {
    Ok(if inn.value == value && inn.label.len() > 0 {
        Some(inn.label.clone())
    } else if let Some(rest) = as_struct(&**env.get_item_as_construct(inn.rest)?) {
        let rest = rest.clone();
        reverse_lookup_ident_in(env, value, &rest)?
    } else {
        None
    })
}

fn get_invariant_sets_in(
    env: &mut Environment,
    inn: &CPopulatedStruct,
) -> Vec<InvariantSetPtr> {
    let mut result = vec![env.generated_invariants(inn.value)];
    if let Ok(Some(rest)) = env.get_and_downcast_construct_definition::<CPopulatedStruct>(inn.rest)
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
        if let Some(structt) = as_struct(&**env.get_item_as_construct(self.0)?) {
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
        if let Some(structt) = as_struct(&**env.get_item_as_construct(self.0)?) {
            let structt = structt.clone();
            reverse_lookup_ident_in(env, value, &structt)
        } else {
            unreachable!()
        }
    }

    fn local_get_invariant_sets(&self, env: &mut Environment) -> Vec<InvariantSetPtr> {
        if let Ok(Some(structt)) =
            env.get_and_downcast_construct_definition::<CPopulatedStruct>(self.0)
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
