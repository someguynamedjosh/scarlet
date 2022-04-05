use super::{as_struct, base::ItemId, Construct};
use crate::{
    environment::{
        dependencies::{DepResult, Dependencies},
        Environment,
    },
    impl_any_eq_for_construct,
    scope::{LookupIdentResult, ReverseLookupIdentResult, Scope},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CPopulatedStruct {
    label: String,
    value: ItemId,
    rest: ItemId,
}

impl CPopulatedStruct {
    pub fn new<'x>(label: String, value: ItemId, rest: ItemId) -> Self {
        Self { label, value, rest }
    }

    pub fn get_label(&self) -> &str {
        &self.label[..]
    }

    pub fn get_value(&self) -> ItemId {
        self.value
    }

    pub fn get_rest(&self) -> ItemId {
        self.rest
    }
}

impl_any_eq_for_construct!(CPopulatedStruct);

impl Construct for CPopulatedStruct {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn contents<'x>(&self) -> Vec<ItemId> {
        vec![self.value, self.rest]
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        let mut deps = env.get_dependencies(self.value);
        deps.append(env.get_dependencies(self.rest));
        deps
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtomicStructMember {
    Label,
    Value,
    Rest,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CAtomicStructMember(pub ItemId, pub AtomicStructMember);

impl_any_eq_for_construct!(CAtomicStructMember);

impl Construct for CAtomicStructMember {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
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
pub struct SField(pub ItemId);

impl Scope for SField {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> LookupIdentResult {
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

    fn local_reverse_lookup_ident<'x>(
        &self,
        env: &mut Environment<'x>,
        value: ItemId,
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

    fn parent(&self) -> Option<ItemId> {
        Some(self.0)
    }
}

#[derive(Debug, Clone)]
pub struct SFieldAndRest(pub ItemId);

fn lookup_ident_in<'x>(
    env: &mut Environment<'x>,
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

fn reverse_lookup_ident_in<'x>(
    env: &mut Environment<'x>,
    value: ItemId,
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

impl Scope for SFieldAndRest {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> LookupIdentResult {
        if let Some(structt) = as_struct(&**env.get_item_as_construct(self.0)?) {
            let structt = structt.clone();
            lookup_ident_in(env, ident, &structt)
        } else {
            unreachable!()
        }
    }

    fn local_reverse_lookup_ident<'a, 'x>(
        &self,
        env: &'a mut Environment<'x>,
        value: ItemId,
    ) -> ReverseLookupIdentResult {
        if let Some(structt) = as_struct(&**env.get_item_as_construct(self.0)?) {
            let structt = structt.clone();
            reverse_lookup_ident_in(env, value, &structt)
        } else {
            unreachable!()
        }
    }

    fn parent(&self) -> Option<ItemId> {
        Some(self.0)
    }
}
