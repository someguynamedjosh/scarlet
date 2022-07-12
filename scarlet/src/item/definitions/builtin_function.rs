use itertools::Itertools;

use super::substitution::DSubstitution;
use crate::{
    diagnostic::Position,
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        dependencies::{Dcc, DepResult, Dependencies, DependenciesFeature, OnlyCalledByDcc},
        equality::{Ecc, Equal, EqualResult, EqualityFeature, OnlyCalledByEcc},
        invariants::{Icc, InvariantSet, InvariantsFeature, InvariantsResult, OnlyCalledByIcc},
        resolvable::RSubstitution,
        ContainmentType, Item, ItemDefinition, ItemPtr,
    },
    scope::Scope,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuiltinFunction {
    Decision,
    Body,
    TailLabel,
    TailValue,
    HasTail,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DBuiltinFunction {
    func: BuiltinFunction,
    args: Vec<ItemPtr>,
}

impl DBuiltinFunction {
    pub fn from_name(env: &Environment, name: &str) -> Option<Self> {
        Some(Self {
            func: match name {
                "decision" => BuiltinFunction::Decision,
                "body" => BuiltinFunction::Body,
                "tail_label" => BuiltinFunction::TailLabel,
                "tail_value" => BuiltinFunction::TailValue,
                "has_tail" => BuiltinFunction::HasTail,
                _ => return None,
            },
            args: match name {
                "decision" => vec!["x", "y", "when_equal", "when_not_equal"],
                "body" => vec!["x"],
                "tail_label" => vec!["x"],
                "tail_value" => vec!["x"],
                "has_tail" => vec!["x"],
                _ => return None,
            }
            .into_iter()
            .map(|name| env.get_language_item(name).unwrap().ptr_clone())
            .collect_vec(),
        })
    }

    fn with_args(
        env: &Environment,
        name: &str,
        args: Vec<ItemPtr>,
        scope: Box<dyn Scope>,
        position: Position,
    ) -> RSubstitution {
        let base = Self::from_name(env, name).unwrap();
        let base = Item::new_boxed(Box::new(base), scope);
        RSubstitution {
            base,
            position,
            named_subs: Default::default(),
            anonymous_subs: args,
        }
    }

    pub fn decision(
        env: &Environment,
        x: ItemPtr,
        y: ItemPtr,
        when_equal: ItemPtr,
        when_not_equal: ItemPtr,
        scope: Box<dyn Scope>,
        position: Position,
    ) -> RSubstitution {
        let args = vec![x, y, when_equal, when_not_equal];
        Self::with_args(env, "decision", args, scope, position)
    }

    pub fn body(
        env: &Environment,
        x: ItemPtr,
        scope: Box<dyn Scope>,
        position: Position,
    ) -> RSubstitution {
        let args = vec![x];
        Self::with_args(env, "body", args, scope, position)
    }

    pub fn tail_label(
        env: &Environment,
        x: ItemPtr,
        scope: Box<dyn Scope>,
        position: Position,
    ) -> RSubstitution {
        let args = vec![x];
        Self::with_args(env, "tail_label", args, scope, position)
    }

    pub fn tail_value(
        env: &Environment,
        x: ItemPtr,
        scope: Box<dyn Scope>,
        position: Position,
    ) -> RSubstitution {
        let args = vec![x];
        Self::with_args(env, "tail_value", args, scope, position)
    }

    pub fn has_tail(
        env: &Environment,
        x: ItemPtr,
        scope: Box<dyn Scope>,
        position: Position,
    ) -> RSubstitution {
        let args = vec![x];
        Self::with_args(env, "has_tail", args, scope, position)
    }

    pub fn get_name(&self) -> &'static str {
        match self.func {
            BuiltinFunction::Decision => "decision",
            BuiltinFunction::Body => "body",
            BuiltinFunction::TailLabel => "tail_label",
            BuiltinFunction::TailValue => "tail_value",
            BuiltinFunction::HasTail => "has_tail",
        }
    }

    pub fn get_function(&self) -> BuiltinFunction {
        self.func
    }
}

impl_any_eq_from_regular_eq!(DBuiltinFunction);

impl ItemDefinition for DBuiltinFunction {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<(ContainmentType, ItemPtr)> {
        self.args
            .iter()
            .map(|arg| (ContainmentType::Definitional, arg.ptr_clone()))
            .collect()
    }
}

impl CheckFeature for DBuiltinFunction {}

impl DependenciesFeature for DBuiltinFunction {
    fn get_dependencies_using_context(
        &self,
        _this: &ItemPtr,
        ctx: &mut Dcc,
        affects_return_value: bool,
        _: OnlyCalledByDcc,
    ) -> DepResult {
        let mut deps = Dependencies::new();
        for dep in &self.args {
            deps.append(ctx.get_dependencies(dep, affects_return_value));
        }
        deps
    }
}

impl EqualityFeature for DBuiltinFunction {
    fn get_equality_using_context(&self, ctx: &mut Ecc, _: OnlyCalledByEcc) -> EqualResult {
        if let Some(other) = ctx.other().downcast_definition::<Self>() {
            let self_statement = self.func;
            let other_statement = other.func;
            drop(other);
            Ok(if self_statement == other_statement {
                let mut args_equal = Vec::new();
                for arg in &self.args {
                    args_equal.push(
                        ctx.with_primary_and_other(arg.ptr_clone(), arg.ptr_clone())
                            .get_equality_left()?,
                    );
                }
                Equal::and(args_equal)
            } else {
                Equal::Unknown
            })
        } else {
            Ok(Equal::Unknown)
        }
    }
}

impl InvariantsFeature for DBuiltinFunction {}
