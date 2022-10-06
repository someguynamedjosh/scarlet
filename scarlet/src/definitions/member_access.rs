use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use itertools::Itertools;

use super::{
    builtin::DBuiltin, compound_type::DCompoundType, new_value::DNewValue, parameter::ParameterPtr,
};
use crate::{
    diagnostic::Diagnostic,
    environment::Environment,
    item::{
        query::{
            no_type_check_errors, ChildrenQuery, ParametersQuery, Query, QueryContext,
            TypeCheckQuery, TypeQuery,
        },
        type_hints::TypeHint,
        CddContext, CycleDetectingDebug, IntoItemPtr, Item, ItemDefinition, ItemPtr,
    },
};

#[derive(Clone, Copy)]
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

    fn collect_constraints(&self, this: &ItemPtr) -> Vec<(ItemPtr, ItemPtr)> {
        vec![]
    }

    fn recompute_parameters(
        &self,
        ctx: &mut QueryContext<ParametersQuery>,
    ) -> <ParametersQuery as Query>::Result {
        todo!()
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        todo!()
    }

    fn recompute_type_check(
        &self,
        ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        no_type_check_errors()
    }

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        let base = self.base.reduce(args);
        if self.member_name == "new" {
            if let Some(r#type) = base.downcast_definition::<DCompoundType>() {
                if let Some(constructor) = r#type.constructor(&base) {
                    return constructor;
                }
            }
        }
        if let Some(value) = base.downcast_definition::<DNewValue>() {
            todo!()
        }
        if base.is_same_instance_as(&self.base) {
            this.ptr_clone()
        } else {
            Self {
                base,
                member_name: self.member_name.clone(),
                member_index: self.member_index,
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
        let r#type = r#type.reduce(&HashMap::new());
        let downcast = r#type.downcast_definition::<DCompoundType>();
        if let Some(r#type) = downcast {
            let components = r#type.get_component_types();
            if components.len() > 1 {
                return Err(Diagnostic::new()
                    .with_text_error(format!("Member access on compound value:"))
                    .with_item_error(this));
            } else if components.len() == 0 {
                return Err(Diagnostic::new()
                    .with_text_error(format!("Member access on never value:"))
                    .with_item_error(this));
            }
            let component = components.iter().next().unwrap().1;
            let index = if component.is_exactly_type() {
                match &self.member_name[..] {
                    "new" => Some(Member::Constructor),
                    _ => None,
                }
            } else {
                todo!()
            };
            if let Some(index) = index {
                self.member_index = index;
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
        }
    }
}
