use std::{
    cell::Ref,
    collections::HashMap,
    fmt::{self, Formatter},
};

use itertools::Itertools;
use owning_ref::OwningRef;

use super::{compound_type::DCompoundType, parameter::ParameterPtr};
use crate::{
    diagnostic::Diagnostic,
    environment::{r#true, Environment},
    item::{
        parameters::Parameters,
        query::{
            no_type_check_errors, ParametersQuery, Query, QueryContext, TypeCheckQuery, TypeQuery,
        },
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
            Builtin::IfThenElse => &["Result", "condition", "true_result", "false_result"],
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

fn both_compound_types<'a>(
    a: &'a ItemPtr,
    b: &'a ItemPtr,
) -> Option<(
    OwningRef<Ref<'a, Item>, DCompoundType>,
    OwningRef<Ref<'a, Item>, DCompoundType>,
)> {
    a.downcast_definition::<DCompoundType>()
        .map(|def_a| {
            b.downcast_definition::<DCompoundType>()
                .map(|def_b| (def_a, def_b))
        })
        .flatten()
}

impl ItemDefinition for DBuiltin {
    fn children(&self) -> Vec<ItemPtr> {
        vec![]
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
        for arg in &self.args {
            result.append(arg.query_parameters(ctx));
        }
        result
    }

    fn recompute_type(&self, _ctx: &mut QueryContext<TypeQuery>) -> <TypeQuery as Query>::Result {
        Some(match self.builtin {
            Builtin::IsExactly => todo!(),
            Builtin::IsSubtypeOf => todo!(),
            Builtin::IfThenElse => self.args[0].ptr_clone(),
            Builtin::Type | Builtin::Union => Self::r#type().into_ptr(),
        })
    }

    fn recompute_type_check(
        &self,
        _ctx: &mut QueryContext<TypeCheckQuery>,
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
                } else if let Some((supertype, subtype)) = both_compound_types(supertype, subtype) {
                    if subtype.is_subtype_of(&*supertype) {
                        return r#true();
                    }
                }
            }
            Builtin::IfThenElse => {
                if rargs[1].is_true() {
                    return rargs[2].ptr_clone();
                } else if rargs[1].is_false() {
                    return rargs[3].ptr_clone();
                }
            }
            Builtin::Type => return DCompoundType::new(this.ptr_clone(), 0).into_ptr(),
            Builtin::Union => {
                if let Some((subtype_0, subtype_1)) = both_compound_types(&rargs[0], &rargs[1]) {
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
        let _true = Some(env.r#true());
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
