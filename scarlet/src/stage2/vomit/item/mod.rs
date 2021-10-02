use super::{helpers, value};
use crate::{
    stage1::structure::expression::Expression,
    stage2::structure::{Environment, Item, Namespace, Value},
};

mod defining;
mod member;
mod replacing;

pub fn vomit(env: &Environment, item: Item) -> Expression {
    let namespace = env[item.namespace].as_ref().expect("TODO: Nice error");
    let value = env[item.value].as_ref().expect("TODO: Nice error");
    vomit_impl(namespace, env, item, value)
}

fn vomit_impl(namespace: &Namespace, env: &Environment, item: Item, value: &Value) -> Expression {
    match namespace {
        Namespace::Defining {
            base, definitions, ..
        } => defining::vomit(definitions, env, base, item),
        Namespace::Empty => value::vomit_value_impl(env, value),
        Namespace::Identifier { name, .. } => vomit_identifier(value, name),
        Namespace::Member { base, name } => member::vomit(value, base, name, env),
        Namespace::Root(item) => vomit(env, *item),
        Namespace::Replacing { base, replacements } => {
            replacing::vomit(value, replacements, base, env)
        }
    }
}

fn vomit_identifier(value: &Value, name: &String) -> Expression {
    if let Value::Identifier { name: vname, .. } = value {
        debug_assert_eq!(name, vname);
        helpers::just_root_expression(helpers::identifier(name))
    } else {
        unreachable!("ICE")
    }
}
