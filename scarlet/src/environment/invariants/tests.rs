#![cfg(test)]

use crate::environment::test_util::*;

#[test]
fn basic_invariant() {
    let code = r"
    true IS UNIQUE
    a IS VAR[SELF = true]
    a_statement IS a = true
    ";
    with_env_from_code(code, |mut env, root| {
        let a_statement = env.lookup_ident(root, "a_statement").unwrap().unwrap();
        env.justify(a_statement, a_statement, 1).unwrap();
    });
}

#[test]
fn nonexistant_invariant() {
    let code = r"
    true IS UNIQUE
    false IS UNIQUE
    a IS VAR[SELF = true]
    b_statement IS a = false
    ";
    with_env_from_code(code, |mut env, root| {
        let b_statement = env.lookup_ident(root, "b_statement").unwrap().unwrap();
        env.justify(b_statement, b_statement, 1).unwrap_err();
    });
}

#[test]
fn theorem_invariant() {
    let code = r"
    x IS VAR[]
    fx IS VAR[DEP x]

    a IS VAR[]
    b IS VAR[a = SELF]

    statement IS 
    (fx[b] = fx[a])
    .WITH_DEPENDENCIES[a b fx]
    .AS_LANGUAGE_ITEM[t_eq_ext_rev_statement]

    t_eq_ext_rev IS AXIOM[t_eq_ext_rev]

    c IS VAR[]
    d IS VAR[SELF = c]
    identity IS VAR[]

    t_eq_ext_rev[c d identity]

    justify_this IS 
    statement[c d identity]
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = env.lookup_ident(root, "justify_this").unwrap().unwrap();
        env.justify(justify_this, justify_this, 1).unwrap_err();
    });
}
