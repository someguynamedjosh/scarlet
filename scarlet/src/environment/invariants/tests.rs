#![cfg(test)]

use std::assert_matches::assert_matches;

use crate::{
    constructs::{
        decision::CDecision,
        structt::CPopulatedStruct,
        substitution::{CSubstitution, Substitutions},
        unique::CUnique,
        variable::{CVariable, Variable, VariableId},
        ConstructId,
    },
    environment::{discover_equality::Equal, test_util::*, Environment},
};

#[test]
fn scratch() {
    let code = r"
    true IS UNIQUE
    a IS VAR[SELF = true]
    a_statement IS a = true
    ";
    with_env_from_code(code, |mut env, root| {
        // let a = env.lookup_ident(root, "a").unwrap().unwrap();
        let a_statement = env.lookup_ident(root, "a_statement").unwrap().unwrap();
        env.justify(a_statement, a_statement, 2).unwrap();
    });
}
