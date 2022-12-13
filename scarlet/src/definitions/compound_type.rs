use std::{
    collections::HashMap,
    fmt::{self, Formatter},
    rc::Rc,
};

use itertools::Itertools;
use maplit::hashmap;

use super::{builtin::DBuiltin, new_value::DNewValue, parameter::ParameterPtr};
use crate::{
    item::{
        parameters::Parameters,
        query::{
            no_type_check_errors, ParametersQuery, Query, QueryContext, ResolveQuery,
            TypeCheckQuery, TypeQuery,
        },
        CddContext, CycleDetectingDebug, IntoItemPtr, ItemDefinition, ItemPtr,
    },
    util::PtrExtension, shared::TripleBool,
};

pub type TypeId = Option<Rc<()>>;

#[derive(Clone, Debug)]
pub enum Type {
    GodType,
    UserType {
        type_id: Rc<()>,
        fields: Vec<(String, ItemPtr)>,
    },
}

impl CycleDetectingDebug for Type {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        match self {
            Type::GodType => write!(f, "BUILTIN(Type)"),
            Type::UserType { type_id, fields } => {
                writeln!(f, "NEW_TYPE(")?;
                for (name, param) in fields {
                    writeln!(f, "    {} IS {}", name, param.to_indented_string(ctx, 2))?;
                }
                write!(f, ")")
            }
        }
    }
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

    pub fn get_fields(&self) -> &[(String, ItemPtr)] {
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

    pub fn constructor(this: Rc<Self>, mimicking: &ItemPtr) -> ItemPtr {
        DNewValue::new(
            this.ptr_clone(),
            this.get_fields().iter().map(|f| f.1.ptr_clone()).collect(),
        )
        .into_ptr_mimicking(mimicking)
    }

    /// False is non-definitive here.
    pub fn is_subtype_of(&self, other: &DCompoundType) -> bool {
        other.component_types.contains_key(&self.get_type_id())
    }

    pub fn resolved(&self) -> Self {
        match self {
            Type::GodType => Type::GodType,
            Type::UserType { type_id, fields } => Type::UserType {
                type_id: type_id.ptr_clone(),
                fields: fields
                    .iter()
                    .map(|(k, v)| (k.clone(), v.resolved()))
                    .collect(),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct DCompoundType {
    // These are ORed together. ANDing them would result in an empty type any
    // time you have at least 2 non-identical components.
    component_types: HashMap<TypeId, Rc<Type>>,
}

impl CycleDetectingDebug for DCompoundType {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        if self.component_types.len() == 1 {
            self.component_types.iter().next().unwrap().1.fmt(f, ctx)
        } else {
            write!(f, "UNION(\n")?;
            for (_id, r#type) in &self.component_types {
                write!(f, "   {}", r#type.to_indented_string(ctx, 1))?;
                write!(f, ",\n")?;
            }
            write!(f, ")")
        }
    }
}

impl ItemDefinition for DCompoundType {
    fn children(&self) -> Vec<ItemPtr> {
        self.component_types
            .iter()
            .flat_map(|t| t.1.get_fields().iter())
            .map(|field| field.1.ptr_clone())
            .collect_vec()
    }

    fn collect_constraints(&self, _this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        vec![]
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
        this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result {
        let mut result = Parameters::new_empty();
        for typ in &self.component_types {
            for field in typ.1.get_fields() {
                result.append(field.1.dereference().unwrap().query_parameters(ctx));
            }
        }
        result
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(Self::r#type().into_ptr())
    }

    fn recompute_type_check(
        &self,
        _ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        no_type_check_errors()
    }

    fn recompute_resolved(
        &self,
        this: &ItemPtr,
        ctx: &mut QueryContext<ResolveQuery>,
    ) -> <ResolveQuery as Query>::Result {
        Ok(Self {
            component_types: self
                .component_types
                .iter()
                .map(|(k, v)| (k.clone(), Rc::new(v.resolved())))
                .collect(),
        }
        .into_ptr_mimicking(this))
    }

    fn reduce(&self, this: &ItemPtr, _args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        this.ptr_clone()
    }

    fn is_equal_to(&self, other: &ItemPtr) -> TripleBool {
        if let Some(other) = other.dereference().unwrap().downcast_definition::<Self>() {
            'next_ltype: for ltype in self.component_types.values() {
                for rtype in other.component_types.values() {
                    if ltype.is_same_type_as(rtype) {
                        continue 'next_ltype;
                    }
                }
                return TripleBool::False;
            }
            'next_rtype: for rtype in other.component_types.values() {
                for ltype in self.component_types.values() {
                    if ltype.is_same_type_as(rtype) {
                        continue 'next_rtype;
                    }
                }
                return TripleBool::False;
            }
            TripleBool::True
        } else {
            TripleBool::Unknown
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

    pub fn constructor(&self, this: &ItemPtr) -> Option<ItemPtr> {
        if self.component_types.len() == 1 {
            let r#type = self.component_types.iter().next().unwrap().1.ptr_clone();
            Some(Type::constructor(r#type, this))
        } else {
            None
        }
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

    pub(crate) fn r#type() -> Self {
        Self::new_single(Rc::new(Type::GodType))
    }
}
