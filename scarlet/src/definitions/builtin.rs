#[cfg(not(feature = "trace_borrows"))]
use std::cell::{Ref, RefCell};
use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

#[cfg(feature = "trace_borrows")]
use debug_cell::{Ref, RefCell, RefMut};
use itertools::Itertools;
use owning_ref::OwningRef;

use super::{compound_type::DCompoundType, parameter::ParameterPtr};
use crate::{
    diagnostic::Diagnostic,
    environment::{r#false, r#true, Environment},
    item::{
        parameters::Parameters,
        query::{
            no_type_check_errors, ParametersQuery, Query, QueryContext, ResolveQuery,
            TypeCheckQuery, TypeQuery,
        },
        CddContext, CycleDetectingDebug, IntoItemPtr, Item, ItemDefinition, ItemPtr,
    },
    shared::TripleBool,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Builtin {
    IsExactly,
    IsSubtypeOf,
    IfThenElse,
    Union,
}

impl Builtin {
    pub fn name(&self) -> &'static str {
        match self {
            Self::IsExactly => "is_exactly",
            Self::IsSubtypeOf => "is_subtype_of",
            Self::IfThenElse => "if_then_else",
            Self::Union => "Union",
        }
    }

    pub fn default_arg_names(&self) -> &'static [&'static str] {
        match self {
            Builtin::IsExactly => &["Comparee", "Comparand", "comparee", "comparand"][..],
            Builtin::IsSubtypeOf => &["Subtype", "Supertype"][..],
            Builtin::IfThenElse => &["Result", "condition", "true_result", "false_result"],
            Builtin::Union => &["Subtype0", "Subtype1"],
        }
    }
}

#[derive(Clone, Debug)]
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

    fn recompute_resolved(
        &self,
        this: &ItemPtr,
        ctx: &mut QueryContext<ResolveQuery>,
    ) -> <ResolveQuery as Query>::Result {
        let rargs = self.args.iter().map(|arg| arg.resolved()).collect();
        Ok(Self {
            args: rargs,
            builtin: self.builtin,
        }
        .into_ptr_mimicking(this))
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
            Builtin::Union => DCompoundType::r#type().into_ptr(),
        })
    }

    fn recompute_type_check(
        &self,
        _ctx: &mut QueryContext<TypeCheckQuery>,
    ) -> <TypeCheckQuery as Query>::Result {
        no_type_check_errors()
    }

    fn reduce(&self, this: &ItemPtr, args: &HashMap<ParameterPtr, ItemPtr>) -> ItemPtr {
        let rargs = self
            .args
            .iter()
            .map(|arg| arg.reduced(args, true))
            .collect_vec();
        match self.builtin {
            Builtin::IsExactly => {
                match rargs[2]
                    .dereference()
                    .unwrap()
                    .is_equal_to(&rargs[3].dereference().unwrap())
                {
                    TripleBool::True => return r#true().into_ptr_mimicking(this),
                    TripleBool::False => return r#false().into_ptr_mimicking(this),
                    TripleBool::Unknown => (),
                }
            }
            Builtin::IsSubtypeOf => {
                let subtype = rargs[0].dereference().unwrap();
                let supertype = rargs[1].dereference().unwrap();
                if supertype.is_same_instance_as(&subtype) {
                    return r#true().into_ptr_mimicking(this);
                } else if supertype.is_exactly_type() && subtype.is_exactly_type() {
                    return r#true().into_ptr_mimicking(this);
                } else if let Some((supertype, subtype)) = both_compound_types(&supertype, &subtype)
                {
                    if subtype.is_subtype_of(&*supertype) {
                        return r#true().into_ptr_mimicking(this);
                    }
                }
            }
            Builtin::IfThenElse => {
                if rargs[1].dereference().unwrap().is_true() {
                    return rargs[2].ptr_clone();
                } else if rargs[1].dereference().unwrap().is_false() {
                    return rargs[3].ptr_clone();
                }
            }
            Builtin::Union => {
                if let Some((subtype_0, subtype_1)) = both_compound_types(
                    &rargs[0].dereference().unwrap(),
                    &rargs[1].dereference().unwrap(),
                ) {
                    return subtype_0.union(&subtype_1).into_ptr_mimicking(this);
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
            .into_ptr_mimicking(this)
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

    pub fn is_type(candidate: ItemPtr) -> Self {
        Self::is_subtype_of(
            candidate
                .query_type(&mut Environment::root_query())
                .unwrap(),
            DCompoundType::r#type().into_ptr(),
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
