use std::{
    collections::HashMap,
    fmt::{self, Formatter},
    rc::Rc,
};

use itertools::Itertools;
use maplit::hashmap;

use crate::{environment::ItemId, util::PtrExtension};

pub type TypeId = Option<Rc<()>>;

#[derive(Clone, Debug)]
pub enum Type {
    GodType,
    UserType {
        type_id: Rc<()>,
        fields: Vec<(String, ItemId)>,
    },
}

impl Type {
    pub fn is_same_type_as(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::GodType, Self::GodType) => true,

            (
                Self::UserType { type_id, .. },
                Self::UserType {
                    type_id: other_type_id,
                    ..
                },
            ) => type_id.is_same_instance_as(&other_type_id),
            _ => false,
        }
    }

    pub fn is_god_type(&self) -> bool {
        matches!(self, Self::GodType)
    }

    pub fn get_fields(&self) -> &[(String, ItemId)] {
        match self {
            Self::GodType => &[],
            Self::UserType { fields, .. } => fields,
        }
    }

    pub fn get_type_id(&self) -> TypeId {
        match self {
            Self::GodType => None,
            Self::UserType { type_id, .. } => Some(type_id.ptr_clone()),
        }
    }

    /// False is non-definitive here.
    pub fn is_subtype_of(&self, other: &DCompoundType) -> bool {
        other.component_types.contains_key(&self.get_type_id())
    }
}

#[derive(Clone, Debug)]
pub struct DCompoundType {
    // These are ORed together. ANDing them would result in an empty type any
    // time you have at least 2 non-identical components.
    component_types: HashMap<TypeId, Rc<Type>>,
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

    pub fn is_exactly_type(&self) -> bool {
        self.component_types.len() == 1 && self.component_types.contains_key(&None)
    }

    /// False is non-definitive here.
    pub fn is_subtype_of(&self, other: &Self) -> bool {
        for (key, _) in &self.component_types {
            if !other.component_types.contains_key(key) {
                return false;
            }
        }
        true
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
