use crate::{
    parser::rule::{Component, Rule},
    rules,
};

pub fn rules() -> Vec<Rule> {
    rules![
        (Expr0 -> AE)
        (Expr0 -> UNIQUE)
    ]
}
