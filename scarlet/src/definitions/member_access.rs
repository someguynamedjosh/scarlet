use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use super::{
    compound_type::DCompoundType, new_type::DNewType, new_value::DNewValue, parameter::ParameterPtr,
};
use crate::{
    diagnostic::Diagnostic,
    environment::Environment,
    item::{
        query::{
            no_type_check_errors, ParametersQuery, Query, QueryContext, TypeCheckQuery, TypeQuery,
        },
        CddContext, CycleDetectingDebug, IntoItemPtr, ItemDefinition, ItemPtr,
    },
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Member {
    Unknown,
    IndexIntoUserType(usize),
    Constructor,
}

#[derive(Clone)]
pub struct DMemberAccess {
    base: ItemPtr,
    member_name: String,
    member_index: Member,
    r#type: Option<ItemPtr>,
}

impl CycleDetectingDebug for DMemberAccess {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        self.base.fmt(f, ctx)?;
        write!(f, ".{}", self.member_name)
    }
}

impl ItemDefinition for DMemberAccess {
    fn children(&self) -> Vec<ItemPtr> {
        vec![self.base.ptr_clone()]
    }

    fn collect_constraints(&self, _this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        vec![]
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
        this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result {
        self.base.query_parameters(ctx)
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        self.r#type.clone()
    }

    fn recompute_type_check(
        &self,
        _ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        no_type_check_errors()
    }

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        let base = self.base.reduce_impl(args, false);
        if self.member_index == Member::Unknown {
        } else if self.member_index == Member::Constructor {
            if let Some(r#type) = base.downcast_definition::<DCompoundType>() {
                if let Some(constructor) = r#type.constructor(&base) {
                    return constructor.reduce(args);
                }
            }
        } else if let Member::IndexIntoUserType(index) = self.member_index {
            if let Some(value) = base.downcast_definition::<DNewValue>() {
                return value.fields()[index].reduce(args);
            }
        }
        if base.is_same_instance_as(&self.base) {
            this.ptr_clone()
        } else {
            Self {
                base,
                member_name: self.member_name.clone(),
                member_index: self.member_index,
                r#type: self.r#type.as_ref().map(|t| t.reduce(args)),
            }
            .into_ptr()
        }
    }

    fn resolve(&mut self, this: &ItemPtr) -> Result<(), Diagnostic> {
        let r#type = self.base.query_type(&mut Environment::root_query()).ok_or(
            Diagnostic::new()
                .with_text_error(format!("Failed to determine type of base."))
                .with_item_error(this),
        )?;
        let type_ptr = r#type.reduce(&HashMap::new());
        let downcast = type_ptr.downcast_definition::<DCompoundType>();
        if let Some(r#type) = downcast {
            let components = r#type.get_component_types();
            if components.len() > 1 {
                return Err(Diagnostic::new()
                    .with_text_error(format!("Member access on compound value:"))
                    .with_item_error(this)
                    .with_text_error(format!("The compound type is:"))
                    .with_item_error(&type_ptr));
            } else if components.len() == 0 {
                return Err(Diagnostic::new()
                    .with_text_error(format!("Member access on never value:"))
                    .with_item_error(this));
            }
            let component = components.iter().next().unwrap().1;
            let index = if component.is_exactly_type() {
                match &self.member_name[..] {
                    "new" => Some((Member::Constructor, self.base.ptr_clone())),
                    _ => None,
                }
            } else if let Some(base_type) = component.downcast_definition::<DNewType>() {
                base_type
                    .get_fields()
                    .iter()
                    .position(|x| x.0 == self.member_name)
                    .map(|index| {
                        (
                            Member::IndexIntoUserType(index),
                            base_type.get_fields()[index]
                                .1
                                .query_type(&mut Environment::root_query())
                                .unwrap(),
                        )
                    })
            } else {
                None
            };
            if let Some((index, r#type)) = index {
                self.member_index = index;
                self.r#type = Some(r#type);
                Ok(())
            } else {
                Err(Diagnostic::new())
            }
        } else {
            Err(Diagnostic::new())
        }
    }
}

impl DMemberAccess {
    pub fn new(base: ItemPtr, member_name: String) -> Self {
        Self {
            base,
            member_name,
            member_index: Member::Unknown,
            r#type: None,
        }
    }
}
