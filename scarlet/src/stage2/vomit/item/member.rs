use super::helpers;
use crate::{
    stage1::structure::expression::Expression,
    stage2::structure::{Environment, ItemId},
};

pub fn vomit(env: &Environment, base: ItemId, name: &String) -> Expression {
    let mut base = super::vomit(env, base);
    let ident = helpers::just_root_expression(helpers::identifier(name));
    let member = helpers::single_expr_construct("member", ident);
    base.posts.push(member);
    base
}
