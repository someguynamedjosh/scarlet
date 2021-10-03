use super::helpers;
use crate::{stage1::structure::expression::Expression, stage2::structure::Item};

pub fn vomit(base: &Item, name: &String) -> Expression {
    let mut base = super::vomit(base);
    let ident = helpers::just_root_expression(helpers::identifier(name));
    let member = helpers::single_expr_construct("member", ident);
    base.others.push(member);
    base
}
