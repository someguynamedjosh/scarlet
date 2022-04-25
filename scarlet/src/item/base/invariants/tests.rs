#![cfg(test)]

use crate::item::test_util::*;

#[test]
fn basic_invariant() {
    let code = r"
    a IS UNIQUE
    y IS VAR[SELF = a]
    y_statement IS y = a
    ";
    with_env_from_code(code, |mut env, root| {
        let y_statement = root.lookup_ident("y_statement").unwrap().unwrap();
        env.justify(&root, &y_statement, &y_statement, 2).unwrap();
        root.check_all();
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
    .AS_LANGUAGE_ITEM[t_eq_ext_rev_statement]

    t IS VAR[statement[x IS SELF]]

    justify_this IS
    statement[x IS t]
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 10).unwrap();
        root.check_all();
    });
}

#[test]
fn moderate_invariant() {
    let code = r"
    a IS VAR[]
    b IS VAR[a = SELF]

    x IS VAR[]
    fx IS VAR[DEP x]

    statement IS 
    (fx[b] = fx[a])

    invariant IS statement[t u]

    t IS VAR[]
    u IS VAR[invariant]

    justify_this IS invariant
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 10).unwrap();
        root.check_all();
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
        let z_statement = root.lookup_ident("z_statement").unwrap().unwrap();
        env.justify(&root, &z_statement, &z_statement, 1).unwrap_err();
    });
}

#[test]
fn basic_theorem_invariant() {
    let code = r"
    statement IS 
    UNIQUE
    .AS_LANGUAGE_ITEM[t_eq_ext_rev_statement]

    t_eq_ext_rev IS AXIOM[t_eq_ext_rev]

    justify_this IS statement
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 5).unwrap();
        root.check_all();
    });
}

#[test]
fn subbed_theorem_invariant() {
    let code = r"
    x IS VAR[]

    statement IS 
    x.AS_LANGUAGE_ITEM[t_eq_ext_rev_statement]

    t_eq_ext_rev IS AXIOM[t_eq_ext_rev]

    a IS UNIQUE
    t_eq_ext_rev[a]
    justify_this IS a
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 5).unwrap();
        root.check_all();
    });
}

#[test]
fn function_invariant() {
    let code = r"
    x IS VAR[]
    fx IS VAR[DEP x]

    statement IS 
    fx.AS_LANGUAGE_ITEM[t_eq_ext_rev_statement]

    t_eq_ext_rev IS AXIOM[t_eq_ext_rev]

    identity IS VAR[]

    a IS VAR[]
    t_eq_ext_rev[identity a]
    justify_this IS statement[identity a]
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 5).unwrap();
        root.check_all();
    });
}

#[test]
fn equality_theorem_invariant() {
    let code = r"
    x IS VAR[]
    y IS VAR[]

    statement IS 
    (x = y).AS_LANGUAGE_ITEM[t_eq_ext_rev_statement]

    t_eq_ext_rev IS AXIOM[t_eq_ext_rev]

    a IS UNIQUE
    b IS UNIQUE
    t_eq_ext_rev[a b]
    justify_this IS statement[a b]
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 5).unwrap();
        root.check_all();
    });
}

#[test]
fn real_theorem_invariant() {
    let code = r"
    a IS VAR[]
    b IS VAR[a = SELF]

    x IS VAR[].AS_LANGUAGE_ITEM[x]
    fx IS VAR[DEP x]

    statement IS 
    (fx[b] = fx[a])
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
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 5).unwrap();
        root.check_all();
    });
}

#[test]
fn simpler_auto_theorem_invariant() {
    let code = r"
    # Abusing the axiom feature to introduce a theorem that can be proven from
    # other axioms without doing the full proof.
    statement IS UNIQUE.AS_LANGUAGE_ITEM[t_decision_eq_statement]

    AXIOM[t_decision_eq].AS_AUTO_THEOREM

    justify_this IS statement
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 5).unwrap();
        root.check_all();
    });
}

#[test]
fn single_sub_auto_theorem_invariant() {
    let code = r"
    # Abusing the axiom feature to introduce a theorem that can be proven from
    # other axioms without doing the full proof.
    x IS VAR[]
    y IS VAR[]

    statement IS x.AS_LANGUAGE_ITEM[t_decision_eq_statement]
    AXIOM[t_decision_eq].AS_AUTO_THEOREM

    justify_this IS y
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 5).unwrap();
        root.check_all();
    });
}

#[test]
fn single_sub_in_struct_auto_theorem_invariant() {
    let code = r"
    # Abusing the axiom feature to introduce a theorem that can be proven from
    # other axioms without doing the full proof.
    x IS VAR[]
    y IS VAR[]

    statement IS {x}.AS_LANGUAGE_ITEM[t_decision_eq_statement]
    AXIOM[t_decision_eq].AS_AUTO_THEOREM

    justify_this IS {y}
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 5).unwrap();
        root.check_all();
    });
}

#[test]
fn double_sub_auto_theorem_invariant() {
    let code = r"
    # Abusing the axiom feature to introduce a theorem that can be proven from
    # other axioms without doing the full proof.
    a IS VAR[]
    b IS VAR[]
    x IS VAR[]
    y IS VAR[]

    statement IS {x y}.AS_LANGUAGE_ITEM[t_decision_eq_statement]
    AXIOM[t_decision_eq].AS_AUTO_THEOREM

    justify_this IS {a b}
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 5).unwrap();
        root.check_all();
    });
}

#[test]
fn repeated_single_sub_auto_theorem_invariant() {
    let code = r"
    # Abusing the axiom feature to introduce a theorem that can be proven from
    # other axioms without doing the full proof.
    a IS VAR[]
    x IS VAR[]

    statement IS {x x}.AS_LANGUAGE_ITEM[t_decision_eq_statement]
    AXIOM[t_decision_eq].AS_AUTO_THEOREM

    justify_this IS {a a}
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 2).unwrap();
        root.check_all();
    });
}

#[test]
fn repeated_double_sub_auto_theorem_invariant() {
    let code = r"
    # Abusing the axiom feature to introduce a theorem that can be proven from
    # other axioms without doing the full proof.
    a IS VAR[]
    b IS VAR[]
    x IS VAR[]
    y IS VAR[]

    statement IS {x y x y}.AS_LANGUAGE_ITEM[t_decision_eq_statement]
    AXIOM[t_decision_eq].AS_AUTO_THEOREM

    justify_this IS {a b a b}
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 5).unwrap();
        root.check_all();
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
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 5).unwrap();
        root.check_all();
    });
}

#[test]
fn t_just_after_theorem() {
    let code = r"
    t_eq_ext_rev IS 
    {
        AXIOM[t_eq_ext_rev]

        a IS VAR[]
        b IS VAR[a = SELF]

        statement IS 
        (fx[b] = fx[a])
        .AS_LANGUAGE_ITEM[t_eq_ext_rev_statement]
    }
    .VALUE

    x IS VAR[].AS_LANGUAGE_ITEM[x]
    fx IS VAR[DEP x]

    t_just IS VAR[SELF]

    justify_this IS b = a

    t_eq_ext_rev[a b x]

    a IS VAR[]
    b IS VAR[a = SELF]
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 5).unwrap();
        root.check_all();
    });
}

#[test]
fn justify_auto_refl() {
    let code = r"
    t_refl IS
    {
        AXIOM[t_refl]

        (a = a)
        .AS_LANGUAGE_ITEM[t_refl_statement]

        a IS VAR[]
    }
    .VALUE
    .AS_AUTO_THEOREM

    t_just IS VAR[SELF]

    asdf IS UNIQUE

    justify_this IS asdf = asdf
    ";
    with_env_from_code(code, |mut env, root| {
        let justify_this = root.lookup_ident("justify_this").unwrap().unwrap();
        env.justify(&root, &justify_this, &justify_this, 5).unwrap();
        root.check_all();
    });
}
