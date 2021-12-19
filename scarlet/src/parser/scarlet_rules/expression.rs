use crate::{
    parser::rule::{Component, Rule},
    rules,
};

pub fn rules() -> Vec<Rule> {
    let mut result = rules![(Expr -> Expr127)];
    for prec in 0..127 {
        result.push(Rule {
            produced_nonterminal: format!("Expr{}", prec + 1),
            components: vec![Component::Nonterminal(format!("Expr{}", prec))],
            preferred: true,
        })
    }
    result
}
