use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

use itertools::Itertools;
use maplit::hashset;

use super::{
    compound_type::DCompoundType, new_type::DNewType, new_value::DNewValue, parameter::ParameterPtr,
};
use crate::{
    definitions::parameter::DParameter,
    diagnostic::Diagnostic,
    environment::{r#true, Environment, ENV},
    item::{
        query::{
            no_type_check_errors, ChildrenQuery, ParametersQuery, Query, QueryContext,
            TypeCheckQuery, TypeQuery,
        },
        type_hints::TypeHint,
        CddContext, CycleDetectingDebug, IntoItemPtr, Item, ItemDefinition, ItemPtr,
    },
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Builtin {
    IsExactly,
    IsSubtypeOf,
    IfThenElse,
    Type,
    Union,
}

impl Builtin {
    pub fn name(&self) -> &'static str {
        match self {
            Self::IsExactly => "is_exactly",
            Self::IsSubtypeOf => "is_subtype_of",
            Self::IfThenElse => "if_then_else",
            Self::Type => "Type",
            Self::Union => "Union",
        }
    }

    pub fn default_arg_names(&self) -> &'static [&'static str] {
        match self {
            Builtin::IsExactly => &["comparee", "comparand"][..],
            Builtin::IsSubtypeOf => &["Subtype", "Supertype"][..],
            Builtin::IfThenElse => &["condition", "result_when_true", "result_when_false"],
            Builtin::Type => &[],
            Builtin::Union => &["Subtype0", "Subtype1"],
        }
    }
}

#[derive(Clone)]
pub struct DBuiltin {
    builtin: Builtin,
    args: Vec<ItemPtr>,
}

impl CycleDetectingDebug for DBuiltin {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        write!(f, "BUILTIN({})", self.builtin.name())?;
        if self.args.len() == 0 {
            return Ok(());
        }
        write!(f, "(\n")?;
        for (param_name, arg) in self
            .builtin
            .default_arg_names()
            .iter()
            .zip(self.args.iter())
        {
            write!(f, "   {} IS {}", param_name, arg.to_indented_string(ctx, 2))?;
            write!(f, ",\n")?;
        }
        write!(f, ")")
    }
}

impl ItemDefinition for DBuiltin {
    fn children(&self) -> Vec<ItemPtr> {
        vec![]
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
        Some(Self::r#type().into_ptr())
    }

    fn recompute_type_check(
        &self,
        ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        no_type_check_errors()
    }

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        let rargs = self.args.iter().map(|arg| arg.reduce(args)).collect_vec();
        match self.builtin {
            Builtin::IsExactly => {
                if rargs[0].is_same_instance_as(&rargs[1]) {
                    return r#true();
                }
            }
            Builtin::IsSubtypeOf => {
                let subtype = &rargs[0];
                let supertype = &rargs[1];
                if supertype.is_same_instance_as(subtype) {
                    return r#true();
                } else if supertype.is_exactly_type() && subtype.is_exactly_type() {
                    return r#true();
                } else if let Some(supertype) = supertype.downcast_definition::<DNewType>() {
                    // todo!()
                }
            }
            Builtin::IfThenElse => (),
            Builtin::Type => return DCompoundType::new(this.ptr_clone(), 0).into_ptr(),
            Builtin::Union => {
                if let (Some(subtype_0), Some(subtype_1)) = (
                    rargs[0].downcast_definition::<DCompoundType>(),
                    rargs[1].downcast_definition::<DCompoundType>(),
                ) {
                    return subtype_0.union(&subtype_1).into_ptr();
                }
            }
        }
        if rargs == self.args {
            this.ptr_clone()
        } else {
            Self {
                args: rargs,
                builtin: self.builtin,
            }
            .into_ptr()
        }
    }
}

impl DBuiltin {
    pub fn new_user_facing(builtin: Builtin, env: &Environment) -> Result<Self, Diagnostic> {
        let args = builtin
            .default_arg_names()
            .iter()
            .map(|name| env.get_language_item(name).map(ItemPtr::ptr_clone))
            .collect::<Result<_, _>>()?;
        let r#true = Some(env.r#true());
        Ok(Self { builtin, args })
    }

    pub fn r#type() -> Self {
        Self {
            builtin: Builtin::Type,
            args: vec![],
        }
    }

    pub fn is_type(candidate: ItemPtr) -> Self {
        Self::is_subtype_of(
            candidate
                .query_type(&mut Environment::root_query())
                .unwrap(),
            Self::r#type().into_ptr(),
        )
    }

    pub fn is_subtype_of(subtype: ItemPtr, supertype: ItemPtr) -> Self {
        Self {
            builtin: Builtin::IsSubtypeOf,
            args: vec![subtype, supertype],
        }
    }

    pub fn union(subtype_0: ItemPtr, subtype_1: ItemPtr) -> Self {
        Self {
            builtin: Builtin::Union,
            args: vec![subtype_0, subtype_1],
        }
    }

    pub fn get_builtin(&self) -> Builtin {
        self.builtin
    }

    pub fn get_args(&self) -> &Vec<ItemPtr> {
        &self.args
    }
}
