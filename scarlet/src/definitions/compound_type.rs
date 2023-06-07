use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Formatter},
    hash::Hash,
    rc::Rc,
};

use itertools::Itertools;
use maplit::hashmap;

use super::parameter::ParameterPtr;
use crate::{
    environment::{Def2, Env, ItemId},
    util::PtrExtension,
};

#[derive(Clone, Debug)]
pub enum TypeId {
    GodType,
    UserType(Rc<()>),
}

impl PartialEq for TypeId {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::GodType, Self::GodType) => true,
            (Self::UserType(type_id), Self::UserType(other_type_id)) => {
                Rc::ptr_eq(type_id, other_type_id)
            }
            _ => false,
        }
    }
}

impl Eq for TypeId {}

impl Hash for TypeId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        if let Self::UserType(type_id) = &self {
            Rc::as_ptr(type_id).hash(state);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum Type {
    GodType,
    ModuleType {
        type_id: TypeId,
        declarations: Vec<String>,
    },
    UserType {
        type_id: TypeId,
        /// Names paired with parameters that accept values to be assigned to
        /// that field.
        fields: Vec<(String, ItemId)>,
    },
}

impl Type {
    pub fn is_god_type(&self) -> bool {
        matches!(self, Self::GodType)
    }

    pub fn is_constructable_type(&self) -> bool {
        matches!(self, Self::UserType { .. })
    }

    pub fn get_constructor_parameters(&self) -> &[(String, ItemId)] {
        match self {
            Self::UserType { fields, .. } => fields,
            _ => panic!("Not a constructable type."),
        }
    }

    pub fn get_type_id(&self) -> TypeId {
        match self {
            Self::GodType => TypeId::GodType,
            Self::ModuleType { type_id, .. } => type_id.clone(),
            Self::UserType { type_id, .. } => type_id.clone(),
        }
    }

    /// If you get "false", it means we don't know if it's a subtype, not
    /// necessarily that it's guaranteed to not be a subtype.
    pub fn is_subtype_of(&self, other: &DCompoundType) -> bool {
        other.component_types.contains_key(&self.get_type_id())
    }

    pub fn parameters(&self, env: &mut Env) -> Vec<ParameterPtr> {
        let mut parameters = Vec::new();
        if self.is_constructable_type() {
            for field in self.get_constructor_parameters() {
                let Def2::DParameter(param) = &env.get_def2(field.1) else { panic!() };
                let ty = param.get_type();
                parameters.extend(env.get_deps(ty).clone().into_iter());
            }
        }
        parameters
    }
}

#[derive(Clone, Debug)]
pub struct DCompoundType {
    // These are ORed together. ANDing them would result in an empty type any
    // time you have at least 2 non-identical components.
    component_types: HashMap<TypeId, Rc<Type>>,
}

impl PartialEq for DCompoundType {
    fn eq(&self, other: &Self) -> bool {
        if self.component_types.len() != other.component_types.len() {
            false
        } else {
            for component in self.component_types.keys() {
                if !other.component_types.contains_key(component) {
                    return false;
                }
            }
            true
        }
    }
}

impl DCompoundType {
    pub fn new_single(r#type: Rc<Type>) -> Self {
        Self {
            component_types: hashmap![r#type.get_type_id() => r#type],
        }
    }

    pub fn get_component_types(&self) -> &HashMap<TypeId, Rc<Type>> {
        &self.component_types
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut component_types = self.component_types.clone();
        component_types.extend(
            other
                .component_types
                .iter()
                .map(|(id, ty)| (id.clone(), ty.ptr_clone())),
        );
        Self { component_types }
    }

    pub fn is_exactly_god_type(&self) -> bool {
        self.component_types.len() == 1 && self.component_types.contains_key(&TypeId::GodType)
    }

    /// If you get "false", it means we don't know if it's a subtype, not
    /// necessarily that it's guaranteed to not be a subtype.
    pub fn is_subtype_of(&self, other: &Self) -> bool {
        for (key, _) in &self.component_types {
            if !other.component_types.contains_key(key) {
                return false;
            }
        }
        true
    }

    pub fn parameters(&self, env: &mut Env) -> Vec<ParameterPtr> {
        let mut parameters = Vec::new();
        for ty in self.component_types.values() {
            parameters.extend(ty.parameters(env));
        }
        parameters
    }

    pub(crate) fn god_type() -> Self {
        Self::new_single(Rc::new(Type::GodType))
    }

    pub fn get_single_type(&self) -> Option<&Rc<Type>> {
        if self.component_types.len() == 1 {
            Some(&self.component_types.values().next().unwrap())
        } else {
            None
        }
    }
}
