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
    d IS VAR[c = SELF]
    identity IS VAR[]

    t_eq_ext_rev[c d identity]

    justify_this IS 
    statement[c d identity]
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = env.lookup_ident(root, "justify_this").unwrap().unwrap();
        env.justify(justify_this, justify_this, 5).unwrap();
    });
}

#[test]
fn auto_theorem_invariant() {
    let code = r"
    # Abusing the axiom feature to introduce a theorem that can be proven from
    # other axioms without doing the full proof.
    t_decision_eq IS
    { 
        AXIOM[t_decision_eq]

        (DECISION[a a b c] = b)
        .AS_LANGUAGE_ITEM[t_decision_eq_statement]

        a IS VAR[]
        b IS VAR[]
        c IS VAR[]
    }
    .VALUE
    .AS_AUTO_THEOREM

    d IS VAR[]
    e IS VAR[]
    f IS VAR[]

    justify_this IS
    DECISION[d d e f] = e
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = env.lookup_ident(root, "justify_this").unwrap().unwrap();
        env.justify(justify_this, justify_this, 5).unwrap();
    });
}

#[test]
fn t_just_after_theorem() {
    let code = r"
    x IS VAR[].AS_LANGUAGE_ITEM[x]
    fx IS VAR[DEP x]

    t_eq_ext_rev IS 
    {
        AXIOM[t_eq_ext_rev]

        statement IS 
        (fx[b] = fx[a])
        .WITH_DEPENDENCIES[a b fx]
        .AS_LANGUAGE_ITEM[t_eq_ext_rev_statement]

        a IS VAR[]
        b IS VAR[a = SELF]
    }
    .VALUE

    t_just IS VAR[SELF]

    justify_this IS b = a

    t_eq_ext_rev[a b x]

    a IS VAR[]
    b IS VAR[a = SELF]
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = env.lookup_ident(root, "justify_this").unwrap().unwrap();
        println!("Justifying...");
        env.justify(justify_this, justify_this, 5).unwrap();
    });
}