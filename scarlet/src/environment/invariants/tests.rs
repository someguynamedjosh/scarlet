#![cfg(test)]

use crate::environment::test_util::*;

#[test]
fn basic_invariant() {
    let code = r"
    a IS UNIQUE
    y IS VAR[SELF = a]
    y_statement IS y = a
    ";
    with_env_from_code(code, |mut env, root| {
        let y_statement = env.lookup_ident(root, "y_statement").unwrap().unwrap();
        env.justify(y_statement, y_statement, 1).unwrap();
    });
}

#[test]
fn sub_invariant() {
    let code = r"
    a IS UNIQUE

    x IS VAR[].AS_LANGUAGE_ITEM[x]
    fx IS VAR[DEP x]

    statement IS 
    (fx = a)
    .WITH_DEPENDENCIES[x fx]
    .AS_LANGUAGE_ITEM[t_eq_ext_rev_statement]

    t IS VAR[statement[x IS SELF]]

    justify_this IS
    statement[x IS t]
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = env.lookup_ident(root, "justify_this").unwrap().unwrap();
        env.justify(justify_this, justify_this, 10).unwrap();
    });
}

#[test]
fn moderate_invariant() {
    let code = r"
    x IS VAR[].AS_LANGUAGE_ITEM[x]
    fx IS VAR[DEP x]

    a IS VAR[]
    b IS VAR[a = SELF]

    statement IS 
    (fx[b] = fx[a])
    .WITH_DEPENDENCIES[a b fx]
    .AS_LANGUAGE_ITEM[t_eq_ext_rev_statement]

    t IS VAR[]
    u IS VAR[t = u   statement[t u]]

    justify_this IS
    statement[t u]
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = env.lookup_ident(root, "justify_this").unwrap().unwrap();
        env.justify(justify_this, justify_this, 10).unwrap();
    });
}

#[test]
fn nonexistant_invariant() {
    let code = r"
    a IS UNIQUE
    b IS UNIQUE
    y IS VAR[SELF = a]
    z_statement IS y = b
    ";
    with_env_from_code(code, |mut env, root| {
        let z_statement = env.lookup_ident(root, "z_statement").unwrap().unwrap();
        env.justify(z_statement, z_statement, 1).unwrap_err();
    });
}

#[test]
fn theorem_invariant() {
    let code = r"
    x IS VAR[].AS_LANGUAGE_ITEM[x]
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
        env.justify(justify_this, justify_this, 5).unwrap_err();
    });
}
