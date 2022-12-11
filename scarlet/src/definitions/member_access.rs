use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use super::{compound_type::DCompoundType, new_value::DNewValue, parameter::ParameterPtr};
use crate::{
    diagnostic::Diagnostic,
    environment::Environment,
    item::{
        query::{
            no_type_check_errors, ParametersQuery, Query, QueryContext, ResolveQuery,
            TypeCheckQuery, TypeQuery,
        },
        CddContext, CycleDetectingDebug, IntoItemPtr, ItemDefinition, ItemPtr,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Member {
    Unknown,
    IndexIntoUserType(usize),
    Constructor,
}

#[derive(Clone, Debug)]
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
        let self_base = self.base.dereference().unwrap();
        let base = self_base.reduced(args, false);
        if self.member_index == Member::Unknown {
        } else if self.member_index == Member::Constructor {
            if let Some(r#type) = base.downcast_definition::<DCompoundType>() {
                if let Some(constructor) = r#type.constructor(&base) {
                    return constructor.reduced(args, true);
                }
            }
        } else if let Member::IndexIntoUserType(index) = self.member_index {
            if let Some(value) = base.downcast_definition::<DNewValue>() {
                return value.fields()[index]
                    .dereference()
                    .unwrap()
                    .reduced(args, true)
                    .dereference()
                    .unwrap();
            }
        }
        if base.is_same_instance_as(&self_base) {
            this.ptr_clone()
        } else {
            Self {
                base,
                member_name: self.member_name.clone(),
                member_index: self.member_index,
                r#type: self
                    .r#type
                    .as_ref()
                    .map(|t| t.dereference().unwrap().reduced(args, true)),
            }
            .into_ptr_mimicking(this)
        }
    }

    fn recompute_resolved(
        &self,
        this: &ItemPtr,
        ctx: &mut QueryContext<ResolveQuery>,
    ) -> <ResolveQuery as Query>::Result {
        let self_base = self.base.dereference().unwrap();
        let rbase = self_base.resolved().dereference().unwrap();
        let r#type = rbase.query_type(&mut Environment::root_query()).ok_or(
            Diagnostic::new()
                .with_text_error(format!("Failed to determine type of base."))
                .with_item_error(this),
        )?;
        let type_ptr = r#type
            .resolved()
            .reduced(&Default::default(), true)
            .dereference()
            .unwrap();
        let downcast = type_ptr.downcast_definition::<DCompoundType>();
        if let Some(r#type) = &downcast {
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
            let index = if component.is_god_type() {
                match &self.member_name[..] {
                    "new" => Some((Member::Constructor, self.base.ptr_clone())),
                    _ => None,
                }
            } else {
                component
                    .get_fields()
                    .iter()
                    .position(|x| x.0 == self.member_name)
                    .map(|index| {
                        (
                            Member::IndexIntoUserType(index),
                            component.get_fields()[index]
                                .1
                                .dereference()
                                .unwrap()
                                .query_type(&mut Environment::root_query())
                                .unwrap(),
                        )
                    })
            };
            if let Some((index, r#type)) = index {
                if index == Member::Constructor {
                    println!("{:#?}", rbase.clone_definition());
                    if let Some(r#type) = rbase
                        .dereference()
                        .unwrap()
                        .downcast_definition::<DCompoundType>()
                    {
                        if let Some(constructor) = r#type.constructor(&rbase) {
                            return Ok(constructor
                                .resolved()
                                .dereference()?
                                .with_position(this.get_position()));
                        } else {
                            panic!("Attempt to get constructor of Type.");
                        }
                    } else {
                        panic!("Attempted to get constructor of type only known at runtime!");
                    }
                }
                Ok(Self {
                    base: rbase,
                    member_index: index,
                    r#type: Some(r#type),
                    member_name: self.member_name.clone(),
                }
                .into_ptr_mimicking(this))
            } else {
                Err(Diagnostic::new().with_text_error(format!(
                    "Failed to determine which member is being referred to."
                )))
            }
        } else {
            println!("type: {:#?}", type_ptr);
            Err(Diagnostic::new().with_text_error(format!(
                "Internal error: Something that's supposed to be a type is not actually a type."
            )))
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
