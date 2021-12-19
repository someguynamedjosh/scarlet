use crate::{
    parser::rule::{Component, Rule},
    rules,
};

pub fn rules() -> Vec<Rule> {
    rules![
        (Expr64 -> Expr63 W := W Expr63)
    ]
}
