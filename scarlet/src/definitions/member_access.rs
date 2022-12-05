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
            no_type_check_errors, ParametersQuery, Query, QueryContext, ResolveQuery,
            TypeCheckQuery, TypeQuery,
        },
        CddContext, CycleDetectingDebug, IntoItemPtr, ItemDefinition, ItemPtr, LazyItemPtr,
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
    base: LazyItemPtr,
    member_name: String,
    member_index: Member,
    r#type: Option<LazyItemPtr>,
}

impl CycleDetectingDebug for DMemberAccess {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        self.base.fmt(f, ctx)?;
        write!(f, ".{}", self.member_name)
    }
}

impl ItemDefinition for DMemberAccess {
    fn children(&self) -> Vec<LazyItemPtr> {
        vec![self.base.ptr_clone()]
    }

    fn collect_constraints(&self, _this: &ItemPtr) -> Vec<(LazyItemPtr, ItemPtr)> {
        vec![]
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
        this: &ItemPtr,
    ) -> <ParametersQuery as Query>::Result {
        self.base.evaluate().unwrap().query_parameters(ctx)
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

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, LazyItemPtr>) -> ItemPtr {
        let self_base = self.base.evaluate().unwrap();
        let base = self_base.reduce_now(args, false);
        if self.member_index == Member::Unknown {
        } else if self.member_index == Member::Constructor {
            if let Some(r#type) = base.downcast_definition::<DCompoundType>() {
                if let Some(constructor) = r#type.constructor(&base) {
                    return constructor.reduced(args.clone()).evaluate().unwrap();
                }
            }
        } else if let Member::IndexIntoUserType(index) = self.member_index {
            if let Some(value) = base.downcast_definition::<DNewValue>() {
                return value.fields()[index]
                    .evaluate()
                    .unwrap()
                    .reduced(args.clone())
                    .evaluate()
                    .unwrap();
            }
        }
        if base.is_same_instance_as(&self_base) {
            this.ptr_clone()
        } else {
            Self {
                base: base.into_lazy(),
                member_name: self.member_name.clone(),
                member_index: self.member_index,
                r#type: self
                    .r#type
                    .as_ref()
                    .map(|t| t.evaluate().unwrap().reduced(args.clone())),
            }
            .into_ptr_mimicking(this)
        }
    }

    fn recompute_resolved(
        &self,
        this: &ItemPtr,
        ctx: &mut QueryContext<ResolveQuery>,
    ) -> <ResolveQuery as Query>::Result {
        let self_base = self.base.evaluate().unwrap();
        let rbase = self_base.resolved().evaluate().unwrap();
        let r#type = rbase.query_type(&mut Environment::root_query()).ok_or(
            Diagnostic::new()
                .with_text_error(format!("Failed to determine type of base."))
                .with_item_error(this),
        )?;
        let type_ptr = r#type
            .evaluate()
            .unwrap()
            .resolved()
            .evaluate()?
            .reduced(Default::default())
            .evaluate()
            .unwrap();
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
            let component = component.evaluate().unwrap();
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
                                .evaluate()
                                .unwrap()
                                .query_type(&mut Environment::root_query())
                                .unwrap(),
                        )
                    })
            } else {
                None
            };
            if let Some((index, r#type)) = index {
                if index == Member::Constructor {
                    if let Some(r#type) = rbase.downcast_definition::<DCompoundType>() {
                        if let Some(constructor) = r#type.constructor(&rbase) {
                            return Ok(constructor
                                .resolved()
                                .evaluate()?
                                .with_position(this.get_position()));
                        }
                    }
                }
                Ok(Self {
                    base: rbase.into_lazy(),
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
    pub fn new(base: LazyItemPtr, member_name: String) -> Self {
        Self {
            base,
            member_name,
            member_index: Member::Unknown,
            r#type: None,
        }
    }
}
