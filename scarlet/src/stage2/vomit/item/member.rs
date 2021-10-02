use super::{helpers, value};
use crate::{
    stage1::{
        self,
        structure::{
            construct::{Construct, ConstructBody},
            expression::Expression,
            statement::{Is, Replace, Statement},
        },
    },
    stage2::structure::{
        BuiltinOperation, BuiltinValue, Definitions, Environment, Item, Namespace, NamespaceId,
        Value, ValueId,
    },
};

pub fn vomit(value: &Value, base: &NamespaceId, name: &String, env: &Environment) -> Expression {
    if let Value::Member {
        base: vbase,
        name: vname,
        previous_value,
    } = value
    {
        debug_assert_eq!(base, vbase);
        debug_assert_eq!(name, vname);
        vomit_impl(previous_value, base, env, name)
    } else {
        unreachable!("ICE")
    }
}

fn vomit_impl(
    previous_value: &crate::shared::Id<Option<Value>>,
    base: &crate::shared::Id<Option<Namespace>>,
    env: &Environment,
    name: &String,
) -> Expression {
    let base = Item {
        value: *previous_value,
        namespace: *base,
    };
    let mut base = super::vomit(env, base);
    let ident = helpers::just_root_expression(helpers::identifier(name));
    let member = helpers::single_expr_construct("member", ident);
    base.others.push(member);
    base
}
