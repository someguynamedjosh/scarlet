#![cfg(test)]

use std::assert_matches::assert_matches;

use nom::multi;

use crate::{
    item::{
        definitions::{
            decision::DDecision,
            other::DOther,
            substitution::{DSubstitution, Substitutions},
        },
        equality::Equal,
        test_util::*,
        util::*,
        Item,
    },
    scope::SRoot,
    util::PtrExtension,
};

#[test]
fn something_equals_itself() {
    let mut env = env();
    let thing = unique();
    assert_eq!(thing.get_equality(&thing, 0), Ok(Equal::yes()));
}

#[test]
fn something_equals_variable() {
    let mut env = env();
    let thing = unique();
    let (var_con, var_id) = variable_full();
    let expected = subs(vec![(var_id, thing.ptr_clone())]);
    let left = Equal::Yes(expected.clone(), Default::default());
    assert_eq!(var_con.get_equality(&thing, 1), Ok(left));
    assert_eq!(var_con.get_equality(&thing, 0), Ok(Equal::NeedsHigherLimit));
    assert_eq!(thing.get_equality(&var_con, 0), Ok(Equal::NeedsHigherLimit));
}

#[test]
fn variable_equals_variable() {
    let mut env = env();
    let x = variable_full();
    let y = variable_full();
    let expected = subs(vec![(x.1.ptr_clone(), y.0.ptr_clone())]);
    let left = Equal::Yes(expected.clone(), Default::default());
    assert_eq!(x.0.get_equality(&y.0, 1), Ok(left));
    assert_eq!(x.0.get_equality(&y.0, 0), Ok(Equal::NeedsHigherLimit));
    assert_eq!(y.0.get_equality(&x.0, 0), Ok(Equal::NeedsHigherLimit));
}

#[test]
fn var_sub_something_equals_something() {
    let thing = unique();
    thing.set_name("thing".to_owned());
    let another = unique();
    another.set_name("another".to_owned());
    let (var_con, var_id) = variable_full();
    var_con.set_name("var".to_owned());
    let var_sub_thing = unchecked_substitution(var_con, &subs(vec![(var_id, thing.ptr_clone())]));
    var_sub_thing.set_name("var_sub_thing".to_owned());
    assert_eq!(var_sub_thing.get_equality(&thing, 2), Ok(Equal::yes()));
    assert_eq!(thing.get_equality(&var_sub_thing, 2), Ok(Equal::yes()));
    assert_eq!(var_sub_thing.get_equality(&another, 2), Ok(Equal::No));
    assert_eq!(another.get_equality(&var_sub_thing, 2), Ok(Equal::No));
}

#[test]
fn decision_equals_identical_decision() {
    let mut env = env();
    let a = variable();
    let b = variable();
    let c = variable();
    let d = variable();
    let dec1 = decision(a.ptr_clone(), b.ptr_clone(), c.ptr_clone(), d.ptr_clone());
    let dec2 = decision(a.ptr_clone(), b.ptr_clone(), c.ptr_clone(), d.ptr_clone());
    assert_eq!(dec1.get_equality(&dec2, 2), Ok(Equal::yes()));
    assert_eq!(dec2.get_equality(&dec1, 2), Ok(Equal::yes()));
}

#[test]
fn aabc_is_ddef() {
    let mut env = env();
    let a = variable_full();
    let b = variable_full();
    let c = variable_full();
    let d = variable_full();
    let e = variable_full();
    let f = variable_full();
    let dec1 = decision(
        a.0.ptr_clone(),
        a.0.ptr_clone(),
        b.0.ptr_clone(),
        c.0.ptr_clone(),
    );
    let dec2 = decision(
        d.0.ptr_clone(),
        d.0.ptr_clone(),
        e.0.ptr_clone(),
        f.0.ptr_clone(),
    );
    let left_subs = subs(vec![
        (a.1.ptr_clone(), d.0.ptr_clone()),
        (b.1.ptr_clone(), e.0.ptr_clone()),
        (c.1.ptr_clone(), f.0.ptr_clone()),
    ]);
    assert_eq!(
        dec1.get_equality(&dec2, 3),
        Ok(Equal::Yes(left_subs, Default::default()))
    );
    let right_subs = subs(vec![
        (d.1.ptr_clone(), a.0.ptr_clone()),
        (e.1.ptr_clone(), b.0.ptr_clone()),
        (f.1.ptr_clone(), c.0.ptr_clone()),
    ]);
    assert_eq!(
        dec2.get_equality(&dec1, 3),
        Ok(Equal::Yes(right_subs, Default::default()))
    );
}

#[test]
fn xxbc_is_aabc() {
    let mut env = env();
    let a = unique();
    let b = unique();
    let c = unique();
    let x = variable_full();
    let dec1 = decision(
        x.0.ptr_clone(),
        x.0.ptr_clone(),
        b.ptr_clone(),
        c.ptr_clone(),
    );
    let dec2 = decision(a.ptr_clone(), a.ptr_clone(), b.ptr_clone(), c.ptr_clone());
    let left_subs = subs(vec![(x.1.ptr_clone(), a.ptr_clone())]);
    assert_eq!(
        dec1.get_equality(&dec2, 3),
        Ok(Equal::Yes(left_subs, Default::default()))
    );
}

#[test]
fn aabc_eq_b_is_ddef_eq_e() {
    let mut env = env();
    let truee = unique();
    let falsee = unique();
    let a = variable_full();
    let b = variable_full();
    let c = variable_full();
    let d = variable_full();
    let e = variable_full();
    let f = variable_full();
    let dec1 = decision(
        a.0.ptr_clone(),
        a.0.ptr_clone(),
        b.0.ptr_clone(),
        c.0.ptr_clone(),
    );
    let dec1 = decision(dec1, b.0.ptr_clone(), truee.ptr_clone(), falsee.ptr_clone());
    let dec2 = decision(
        d.0.ptr_clone(),
        d.0.ptr_clone(),
        e.0.ptr_clone(),
        f.0.ptr_clone(),
    );
    let dec2 = decision(dec2, e.0.ptr_clone(), truee.ptr_clone(), falsee);
    let left_subs = subs(vec![
        (a.1.ptr_clone(), d.0.ptr_clone()),
        (b.1.ptr_clone(), e.0.ptr_clone()),
        (c.1.ptr_clone(), f.0.ptr_clone()),
    ]);
    assert_eq!(
        dec1.get_equality(&dec2, 3),
        Ok(Equal::Yes(left_subs, Default::default()))
    );
    let right_subs = subs(vec![
        (d.1.ptr_clone(), a.0.ptr_clone()),
        (e.1.ptr_clone(), b.0.ptr_clone()),
        (f.1.ptr_clone(), c.0.ptr_clone()),
    ]);
    assert_eq!(
        dec2.get_equality(&dec1, 3),
        Ok(Equal::Yes(right_subs, Default::default()))
    );
}

#[test]
fn decision_equals_decision_with_subs() {
    let mut env = env();
    let a = variable_full();
    let b = variable_full();
    let c = variable_full();
    let d = variable_full();
    let w = unique();
    let x = unique();
    let y = unique();
    let z = unique();
    let dec1 = decision(
        a.0.ptr_clone(),
        b.0.ptr_clone(),
        c.0.ptr_clone(),
        d.0.ptr_clone(),
    );
    let dec2 = decision(w.ptr_clone(), x.ptr_clone(), y.ptr_clone(), z.ptr_clone());
    let subs = subs(vec![
        (a.1.ptr_clone(), w.ptr_clone()),
        (b.1.ptr_clone(), x.ptr_clone()),
        (c.1.ptr_clone(), y.ptr_clone()),
        (d.1.ptr_clone(), z.ptr_clone()),
    ]);
    assert_eq!(
        dec1.get_equality(&dec2, 2),
        Ok(Equal::Yes(subs.clone(), Default::default()))
    );
}

#[test]
fn fx_is_gy() {
    let mut env = env();
    let x = variable_full();
    let y = variable_full();
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    assert_matches!(f.0.get_equality(&g.0, 2), Ok(Equal::Yes(..)));
    assert_matches!(g.0.get_equality(&f.0, 2), Ok(Equal::Yes(..)));
    assert_matches!(g.0.get_equality(&f.0, 1), Ok(Equal::NeedsHigherLimit));
    if let Ok(Equal::Yes(lsubs, _)) = f.0.get_equality(&g.0, 2) {
        assert_eq!(lsubs.len(), 2);
        let mut entries = lsubs.iter();
        let next = entries.next().unwrap();
        assert_eq!(next, &(x.1.ptr_clone(), y.0.ptr_clone()));
        let last = entries.next().unwrap();
        assert_eq!(last.0, f.1.ptr_clone());
        if let Some(sub) = last.1.downcast_definition::<DSubstitution>() {
            assert_eq!(sub.base(), &g.0);
            assert_eq!(
                sub.substitutions(),
                &subs(vec![(y.1.ptr_clone(), x.0.ptr_clone())])
            )
        } else {
            panic!("Expected second substitution to be itself another substitution");
        }
    } else {
        unreachable!()
    }
}

#[test]
fn fx_sub_a_is_gy_sub_a() {
    let mut env = env();
    let a = unique();
    a.set_name("a".to_owned());
    let x = variable_full();
    x.0.set_name("x".to_owned());
    let y = variable_full();
    y.0.set_name("y".to_owned());
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    f.0.set_name("f".to_owned());
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    g.0.set_name("g".to_owned());
    let f_sub_a = unchecked_substitution(
        f.0.ptr_clone(),
        &subs(vec![(x.1.ptr_clone(), a.ptr_clone())]),
    );
    f_sub_a.set_name("f[a]".to_owned());
    let g_sub_a = unchecked_substitution(
        g.0.ptr_clone(),
        &subs(vec![(y.1.ptr_clone(), a.ptr_clone())]),
    );
    g_sub_a.set_name("g[a]".to_owned());
    assert_matches!(f_sub_a.get_equality(&g_sub_a, 2), Ok(Equal::Yes(..)));
    assert_matches!(g_sub_a.get_equality(&f_sub_a, 2), Ok(Equal::Yes(..)));
    assert_matches!(
        g_sub_a.get_equality(&f_sub_a, 1),
        Ok(Equal::NeedsHigherLimit)
    );
    if let Ok(Equal::Yes(lsubs, _)) = f_sub_a.get_equality(&g_sub_a, 2) {
        assert_eq!(lsubs.len(), 1);
        let mut entries = lsubs.iter();
        let last = entries.next().unwrap();
        assert_eq!(last.0.ptr_clone(), f.1.ptr_clone());
        if let Some(sub) = last.1.downcast_definition::<DSubstitution>() {
            assert_eq!(sub.base(), &g.0);
            assert_eq!(
                sub.substitutions(),
                &subs(vec![(y.1.ptr_clone(), x.0.ptr_clone())])
            )
        } else {
            panic!("Expected substitution to be itself another substitution");
        }
    } else {
        unreachable!()
    }
}

#[test]
fn fx_sub_gy_is_gy_sub_x() {
    let x = variable_full();
    x.0.set_name(format!("x"));
    let y = variable_full();
    y.0.set_name(format!("y"));
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    f.0.set_name(format!("f"));
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    g.0.set_name(format!("g"));

    let gx = unchecked_substitution(
        g.0.ptr_clone(),
        &subs(vec![(y.1.ptr_clone(), x.0.ptr_clone())]),
    );
    gx.set_name(format!("g[x]"));
    let fx_sub_gy = unchecked_substitution(
        f.0.ptr_clone(),
        &subs(vec![(f.1.ptr_clone(), gx.ptr_clone())]),
    );
    fx_sub_gy.set_name(format!("f[g[x]]"));

    assert_eq!(gx.get_equality(&fx_sub_gy, 6), Ok(Equal::yes()));
    assert_eq!(fx_sub_gy.get_equality(&gx, 6), Ok(Equal::yes()));
}

#[test]
fn fx_sub_nothing_is_gy_sub_nothing() {
    let mut env = env();
    let x = variable_full();
    let y = variable_full();
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    let f_sub = unchecked_substitution(f.0.ptr_clone(), &Default::default());
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    let g_sub = unchecked_substitution(g.0.ptr_clone(), &Default::default());
    assert_matches!(f_sub.get_equality(&g_sub, 3), Ok(Equal::Yes(..)));
    assert_matches!(g_sub.get_equality(&f_sub, 3), Ok(Equal::Yes(..)));
    assert_matches!(g_sub.get_equality(&f_sub, 0), Ok(Equal::NeedsHigherLimit));
    if let Ok(Equal::Yes(lsubs, _)) = f_sub.get_equality(&g_sub, 3) {
        assert_eq!(lsubs.len(), 2);
        let mut entries = lsubs.iter();
        assert_eq!(entries.next(), Some(&(x.1.ptr_clone(), y.0.ptr_clone())));
        let last = entries.next().unwrap();
        assert_eq!(last.0, f.1.ptr_clone());
        if let Some(sub) = last.1.downcast_definition::<DSubstitution>() {
            assert_eq!(sub.base(), &g.0);
            assert_eq!(
                sub.substitutions(),
                &subs(vec![(y.1.ptr_clone(), x.0.ptr_clone())])
            )
        } else {
            panic!("Expected second substitution to be itself another substitution");
        }
    } else {
        unreachable!()
    }
}

#[test]
fn fx_sub_z_is_gy_sub_nothing() {
    let mut env = env();
    let x = variable_full();
    let y = variable_full();
    let z = variable_full();
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    let f_sub = unchecked_substitution(
        f.0.ptr_clone(),
        &subs(vec![(x.1.ptr_clone(), z.0.ptr_clone())]),
    );
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    let g_sub = unchecked_substitution(g.0.ptr_clone(), &Default::default());
    assert_matches!(f_sub.get_equality(&g_sub, 3), Ok(Equal::Yes(..)));
    assert_matches!(g_sub.get_equality(&f_sub, 3), Ok(Equal::Yes(..)));
    assert_matches!(g_sub.get_equality(&f_sub, 0), Ok(Equal::NeedsHigherLimit));
    if let Ok(Equal::Yes(lsubs, _)) = f_sub.get_equality(&g_sub, 3) {
        assert_eq!(lsubs.len(), 2);
        let mut entries = lsubs.iter();
        assert_eq!(entries.next(), Some(&(z.1.ptr_clone(), y.0.ptr_clone())));
        let last = entries.next().unwrap();
        assert_eq!(last.0, f.1.ptr_clone());
        if let Some(sub) = last.1.downcast_definition::<DSubstitution>() {
            assert_eq!(sub.base(), &g.0);
            assert_eq!(
                sub.substitutions(),
                &subs(vec![(y.1.ptr_clone(), x.0.ptr_clone())])
            )
        } else {
            panic!("Expected second substitution to be itself another substitution");
        }
    } else {
        unreachable!()
    }
}

#[test]
fn fx_sub_decision_is_gy_sub_decision() {
    let mut env = env();
    let a = unique();
    let b = unique();
    let c = unique();
    let d = unique();

    let dec = decision(a.ptr_clone(), b.ptr_clone(), c.ptr_clone(), d.ptr_clone());
    let x = variable_full();
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    let f_dec = unchecked_substitution(f.0.ptr_clone(), &subs(vec![(x.1.ptr_clone(), dec)]));

    let dec = decision(a.ptr_clone(), b.ptr_clone(), c.ptr_clone(), d.ptr_clone());
    let y = variable_full();
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    let g_dec = unchecked_substitution(g.0.ptr_clone(), &subs(vec![(y.1.ptr_clone(), dec)]));

    assert_matches!(f_dec.get_equality(&g_dec, 3), Ok(Equal::Yes(..)));
    assert_matches!(g_dec.get_equality(&f_dec, 3), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(lsubs, _)) = f_dec.get_equality(&g_dec, 3) {
        assert_eq!(lsubs.len(), 1);
        let mut entries = lsubs.iter();
        let last = entries.next().unwrap();
        assert_eq!(last.0, f.1.ptr_clone());
        if let Some(sub) = last.1.downcast_definition::<DSubstitution>() {
            assert_eq!(sub.base(), &g.0);
            assert_eq!(
                sub.substitutions(),
                &subs(vec![(y.1.ptr_clone(), x.0.ptr_clone())])
            )
        } else {
            panic!("Expected second substitution to be itself another substitution");
        }
    } else {
        unreachable!()
    }
}

#[test]
fn dex_sub_decision_is_gy_sub_decision() {
    let mut env = env();
    let a = unique();
    let b = unique();
    let c = unique();
    let d = unique();

    let s = variable_full();
    let t = variable_full();
    let u = variable_full();
    let v = variable_full();

    let dec_for_dex = decision(a.ptr_clone(), b.ptr_clone(), c.ptr_clone(), d.ptr_clone());
    let x = variable_full();
    let dex = decision(x.0.ptr_clone(), d.ptr_clone(), c.ptr_clone(), b.ptr_clone());
    let dex_dec =
        unchecked_substitution(dex.ptr_clone(), &subs(vec![(x.1.ptr_clone(), dec_for_dex)]));

    let dec_for_g = decision(
        s.0.ptr_clone(),
        t.0.ptr_clone(),
        u.0.ptr_clone(),
        v.0.ptr_clone(),
    );
    let y = variable_full();
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    let g_dec = unchecked_substitution(g.0.ptr_clone(), &subs(vec![(y.1.ptr_clone(), dec_for_g)]));

    assert_matches!(g_dec.get_equality(&dex_dec, 3), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(lsubs, _)) = g_dec.get_equality(&dex_dec, 3) {
        assert_eq!(lsubs.len(), 5);
        let mut entries = lsubs.iter();
        assert_eq!(entries.next().unwrap(), &(s.1.ptr_clone(), a.ptr_clone()));
        assert_eq!(entries.next().unwrap(), &(t.1.ptr_clone(), b.ptr_clone()));
        assert_eq!(entries.next().unwrap(), &(u.1.ptr_clone(), c.ptr_clone()));
        assert_eq!(entries.next().unwrap(), &(v.1.ptr_clone(), d.ptr_clone()));
        let first = entries.next().unwrap();
        assert_eq!(first.0, g.1.ptr_clone());
        if let Some(sub) = first.1.downcast_definition::<DSubstitution>() {
            assert_eq!(sub.base(), &dex);
            assert_eq!(
                sub.substitutions(),
                &subs(vec![(x.1.ptr_clone(), y.0.ptr_clone())])
            )
        } else {
            panic!("Expected last substitution to be itself another substitution");
        }
    } else {
        unreachable!()
    }
}

#[test]
fn fx_sub_decision_with_var_is_gy_sub_decision() {
    let mut env = env();

    let aa = variable_full();

    let a = unique();
    let b = unique();
    let c = unique();
    let d = unique();

    let dec = decision(
        aa.0.ptr_clone(),
        b.ptr_clone(),
        c.ptr_clone(),
        d.ptr_clone(),
    );
    let x = variable_full();
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    let f_dec = unchecked_substitution(f.0.ptr_clone(), &subs(vec![(x.1.ptr_clone(), dec)]));

    let dec = decision(a.ptr_clone(), b.ptr_clone(), c.ptr_clone(), d.ptr_clone());
    let y = variable_full();
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    let g_dec = unchecked_substitution(g.0.ptr_clone(), &subs(vec![(y.1.ptr_clone(), dec)]));

    assert_matches!(f_dec.get_equality(&g_dec, 4), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(lsubs, _)) = f_dec.get_equality(&g_dec, 4) {
        assert_eq!(lsubs.len(), 2);
        let mut entries = lsubs.iter();
        assert_eq!(Some(&(aa.1.ptr_clone(), a.ptr_clone())), entries.next());
        let last = entries.next().unwrap();
        assert_eq!(last.0, f.1.ptr_clone());
        if let Some(sub) = last.1.downcast_definition::<DSubstitution>() {
            assert_eq!(sub.base(), &g.0);
            assert_eq!(
                sub.substitutions(),
                &subs(vec![(y.1.ptr_clone(), x.0.ptr_clone())])
            )
        } else {
            panic!("Expected second substitution to be itself another substitution");
        }
    } else {
        unreachable!()
    }
}

#[test]
fn fx_sub_gy_sub_a_is_gy_sub_a() {
    let mut env = env();

    // 13
    let a = unique();
    // 14
    let x = variable_full();
    // 15
    let y = variable_full();
    // 16
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    // 17
    let gx = unchecked_substitution(
        g.0.ptr_clone(),
        &subs(vec![(y.1.ptr_clone(), x.0.ptr_clone())]),
    );

    // 18
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    // 19
    let f_sub_gx = unchecked_substitution(f.0.ptr_clone(), &subs(vec![(f.1.ptr_clone(), gx)]));
    // 20
    let f_sub_gx_sub_a =
        unchecked_substitution(f_sub_gx, &subs(vec![(x.1.ptr_clone(), a.ptr_clone())]));

    // 21
    let gy_sub_a = unchecked_substitution(
        g.0.ptr_clone(),
        &subs(vec![(y.1.ptr_clone(), a.ptr_clone())]),
    );

    assert_eq!(f_sub_gx_sub_a.get_equality(&gy_sub_a, 5), Ok(Equal::yes()));
    assert_eq!(gy_sub_a.get_equality(&f_sub_gx_sub_a, 5), Ok(Equal::yes()));
}

#[test]
fn fx_sub_a_sub_gy_is_gy_sub_a() {
    let mut env = env();

    let a = unique();

    let x = variable_full();
    let y = variable_full();
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    let gx = unchecked_substitution(
        g.0.ptr_clone(),
        &subs(vec![(y.1.ptr_clone(), x.0.ptr_clone())]),
    );

    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    let f_sub_a = unchecked_substitution(
        f.0.ptr_clone(),
        &subs(vec![(x.1.ptr_clone(), a.ptr_clone())]),
    );
    let f_sub_a_sub_gy = unchecked_substitution(f_sub_a, &subs(vec![(f.1.ptr_clone(), gx)]));

    let gy_sub_a = unchecked_substitution(
        g.0.ptr_clone(),
        &subs(vec![(y.1.ptr_clone(), a.ptr_clone())]),
    );

    assert_eq!(f_sub_a_sub_gy.get_equality(&gy_sub_a, 4), Ok(Equal::yes()));
    assert_eq!(gy_sub_a.get_equality(&f_sub_a_sub_gy, 4), Ok(Equal::yes()));
}

#[test]
fn x_eq_y_sub_true_true_is_a_equal_a() {
    let mut env = env();
    let truee = unique();
    truee.set_name("true".to_owned());
    let falsee = unique();
    truee.set_name("false".to_owned());

    let a = variable_full();
    a.0.set_name("a".to_owned());
    let x = variable_full();
    x.0.set_name("x".to_owned());
    let y = variable_full();
    y.0.set_name("y".to_owned());

    let x_eq_y = decision(
        x.0.ptr_clone(),
        y.0.ptr_clone(),
        truee.ptr_clone(),
        falsee.ptr_clone(),
    );
    x_eq_y.set_name("x=y".to_owned());
    let true_eq_true = unchecked_substitution(
        x_eq_y,
        &subs(vec![
            (x.1.ptr_clone(), truee.ptr_clone()),
            (y.1.ptr_clone(), truee.ptr_clone()),
        ]),
    );
    true_eq_true.set_name("true=true".to_owned());
    let a_eq_a = decision(
        a.0.ptr_clone(),
        a.0.ptr_clone(),
        truee.ptr_clone(),
        falsee.ptr_clone(),
    );
    a_eq_a.set_name("a=a".to_owned());

    assert_matches!(a_eq_a.get_equality(&true_eq_true, 3), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(lsubs, _)) = a_eq_a.get_equality(&true_eq_true, 3) {
        assert_eq!(lsubs.len(), 1);
        let mut entries = lsubs.iter();
        let last = entries.next().unwrap();
        assert_eq!(last, &(a.1.ptr_clone(), truee.ptr_clone()));
    } else {
        unreachable!()
    }
}

#[test]
fn is_bool_sub_y_is_y_is_bool() {
    let mut env = env();

    let x = variable_full();
    let y = variable_full();
    let t = unique();
    let f = unique();

    let x_is_false = decision(x.0.ptr_clone(), f.ptr_clone(), t.ptr_clone(), f.ptr_clone());
    let x_is_bool = decision(x.0.ptr_clone(), t.ptr_clone(), t.ptr_clone(), x_is_false);

    let y_is_false = decision(y.0.ptr_clone(), f.ptr_clone(), t.ptr_clone(), f.ptr_clone());
    let y_is_bool = decision(y.0.ptr_clone(), t.ptr_clone(), t.ptr_clone(), y_is_false);

    let x_sub_y_is_bool =
        unchecked_substitution(x_is_bool, &subs(vec![(x.1.ptr_clone(), y.0.ptr_clone())]));

    assert_eq!(
        y_is_bool.get_equality(&x_sub_y_is_bool, 4),
        Ok(Equal::yes())
    );
}

/// f[z] <=> DECISION[x y a b]
#[test]
fn multi_variable_dex_is_single_variable_dex() {
    let mut env = env();

    let a = unique();
    let b = unique();
    let x = variable_full();
    x.0.set_name("x".to_owned());
    let y = variable_full();
    y.0.set_name("y".to_owned());
    let z = variable_full();
    z.0.set_name("z".to_owned());

    let fz = variable_full_with_deps(vec![z.0.ptr_clone()]);
    fz.0.ptr_clone().set_name("fz".to_owned());

    let multi_variable_dex = decision(
        x.0.ptr_clone(),
        y.0.ptr_clone(),
        a.ptr_clone(),
        b.ptr_clone(),
    );

    if let Equal::Yes(subs, _) =
        fz.0.ptr_clone()
            .get_equality(&multi_variable_dex, 15)
            .unwrap()
    {
        assert_eq!(subs.len(), 2);
        let sub = subs.get(&z.1.ptr_clone()).unwrap().ptr_clone();
        assert_eq!(sub, x.0.ptr_clone());
        let sub = subs.get(&fz.1.ptr_clone()).unwrap().ptr_clone();
        if let Some(def) = sub.downcast_definition::<DSubstitution>() {
            let mut expected = Substitutions::new();
            expected.insert_no_replace(x.1.ptr_clone(), z.0.ptr_clone());
            assert_eq!(def.substitutions(), &expected);
            assert_eq!(def.base(), &multi_variable_dex);
        } else {
            panic!("Substituted value is not itself a substitution!");
        }
        drop(sub);
    } else {
        panic!("Not equal!");
    }
}

/// f[z] <=> DECISION[x y a b][a]
#[test]
fn multi_variable_dex_sub_something_is_single_variable_dex() {
    let mut env = env();

    let a = unique();
    let b = unique();
    let x = variable_full();
    x.0.set_name("x".to_owned());
    let y = variable_full();
    y.0.set_name("y".to_owned());
    let z = variable_full();
    z.0.set_name("z".to_owned());

    let fz = variable_full_with_deps(vec![z.0.ptr_clone()]);
    fz.0.ptr_clone().set_name("fz".to_owned());

    let multi_variable_dex = decision(
        x.0.ptr_clone(),
        y.0.ptr_clone(),
        a.ptr_clone(),
        b.ptr_clone(),
    );
    let subbed_multi_variable_dex = unchecked_substitution(
        multi_variable_dex,
        &subs(vec![(x.1.ptr_clone(), a.ptr_clone())]),
    );

    if let Equal::Yes(subs, _) =
        fz.0.ptr_clone()
            .get_equality(&subbed_multi_variable_dex, 15)
            .unwrap()
    {
        assert_eq!(subs.len(), 2);
        let sub = subs.get(&z.1).unwrap().ptr_clone();
        assert_eq!(sub, y.0.ptr_clone());
        let sub = subs.get(&fz.1).unwrap();
        if let Some(def) = sub.downcast_definition::<DSubstitution>() {
            let def = def.clone();
            let mut expected = Substitutions::new();
            expected.insert_no_replace(y.1.ptr_clone(), z.0.ptr_clone());
            assert_eq!(def.substitutions(), &expected);
            assert_eq!(
                def.base().get_equality(&subbed_multi_variable_dex, 4),
                Ok(Equal::yes())
            );
        } else {
            panic!("Substituted value is not itself a substitution!");
        }
    } else {
        panic!("Not equal!");
    }
}

/// f[z] <=> DECISION[x y a b][x2 y2]
#[test]
fn multi_variable_dex_sub_two_vars_is_single_variable_dex() {
    let mut env = env();

    let a = unique();
    let b = unique();
    let x = variable_full();
    x.0.set_name("x".to_owned());
    let x2 = variable_full();
    x2.0.set_name("x2".to_owned());
    let y = variable_full();
    y.0.set_name("y".to_owned());
    let y2 = variable_full();
    y2.0.set_name("y2".to_owned());
    let z = variable_full();
    z.0.set_name("z".to_owned());

    let fz = variable_full_with_deps(vec![z.0.ptr_clone()]);
    fz.0.ptr_clone().set_name("fz".to_owned());

    let multi_variable_dex = decision(
        x.0.ptr_clone(),
        y.0.ptr_clone(),
        a.ptr_clone(),
        b.ptr_clone(),
    );
    let subbed_multi_variable_dex = unchecked_substitution(
        multi_variable_dex,
        &subs(vec![
            (x.1.ptr_clone(), x2.0.ptr_clone()),
            (y.1.ptr_clone(), y2.0.ptr_clone()),
        ]),
    );

    if let Equal::Yes(subs, _) =
        fz.0.ptr_clone()
            .get_equality(&subbed_multi_variable_dex, 15)
            .unwrap()
    {
        assert_eq!(subs.len(), 2);
        let sub = subs.get(&z.1).unwrap();
        assert_eq!(sub, &x2.0);
        let sub = subs.get(&fz.1).unwrap();
        if let Some(def) = sub.downcast_definition::<DSubstitution>() {
            let def = def.clone();
            let mut expected = Substitutions::new();
            expected.insert_no_replace(x2.1.ptr_clone(), z.0.ptr_clone());
            assert_eq!(def.substitutions(), &expected);
            assert_eq!(
                def.base().get_equality(&subbed_multi_variable_dex, 4),
                Ok(Equal::yes())
            );
        } else {
            panic!("Substituted value is not itself a substitution!");
        }
    } else {
        panic!("Not equal!");
    }
}

/// f[z] <=> DECISION[x y a b][a b]
#[test]
fn multi_variable_dex_sub_two_uniques_is_single_variable_dex() {
    let mut env = env();

    let a = unique();
    let b = unique();
    let x = variable_full();
    x.0.set_name("x".to_owned());
    let y = variable_full();
    y.0.set_name("y".to_owned());
    let z = variable_full();
    z.0.set_name("z".to_owned());

    let fz = variable_full_with_deps(vec![z.0.ptr_clone()]);
    fz.0.ptr_clone().set_name("fz".to_owned());

    let multi_variable_dex = decision(
        x.0.ptr_clone(),
        y.0.ptr_clone(),
        a.ptr_clone(),
        b.ptr_clone(),
    );
    let subbed_multi_variable_dex = unchecked_substitution(
        multi_variable_dex.ptr_clone(),
        &subs(vec![
            (x.1.ptr_clone(), a.ptr_clone()),
            (y.1.ptr_clone(), b.ptr_clone()),
        ]),
    );

    if let Equal::Yes(subs, _) =
        fz.0.ptr_clone()
            .get_equality(&subbed_multi_variable_dex, 15)
            .unwrap()
    {
        assert_eq!(subs.len(), 2);
        let sub = subs.get(&z.1).unwrap();
        assert_eq!(sub, &a);
        let sub = subs.get(&fz.1).unwrap();
        if let Some(def) = sub.downcast_definition::<DSubstitution>() {
            let def = def.clone();
            let mut expected = Substitutions::new();
            expected.insert_no_replace(x.1.ptr_clone(), z.0.ptr_clone());
            assert_eq!(def.substitutions(), &expected);
            if let Some(def) = def.base().downcast_definition::<DSubstitution>() {
                let def = def.clone();
                assert_eq!(def.substitutions().len(), 1);
                assert_eq!(
                    (*def.substitutions().get(&y.1).unwrap()).get_equality(&b.ptr_clone(), 4),
                    Ok(Equal::yes())
                );
                assert_eq!(
                    def.base().get_equality(&multi_variable_dex, 4),
                    Ok(Equal::yes())
                );
            } else {
                panic!("Expected another substitution!");
            }
            drop(def);
        } else {
            panic!("Substituted value is not itself a substitution!");
        }
    } else {
        panic!("Not equal!");
    }
}

/// fx[fx IS x = y   x IS a   y IS b] <=/=> a = b
#[test]
fn sneaky_substitution() {
    let mut env = env();

    // I13
    let a = unique();
    // I14
    let b = unique();
    // I15
    let t = unique();
    // I16
    let f = unique();
    // I17 V0
    let x = variable_full();
    x.0.set_name("x".to_owned());
    // I18 V1
    let y = variable_full();
    y.0.set_name("y".to_owned());

    // I19 V2
    let fx = variable_full_with_deps(vec![x.0.ptr_clone()]);
    fx.0.set_name("fx".to_owned());
    let x_eq_y = decision(
        x.0.ptr_clone(),
        y.0.ptr_clone(),
        t.ptr_clone(),
        f.ptr_clone(),
    );
    let a_eq_b = decision(a.ptr_clone(), b.ptr_clone(), t.ptr_clone(), f.ptr_clone());

    let this_subs = subs(vec![
        (fx.1.ptr_clone(), x_eq_y),
        (x.1.ptr_clone(), a.ptr_clone()),
        (y.1.ptr_clone(), b.ptr_clone()),
    ]);
    let tricky_sub = unchecked_substitution(fx.0.ptr_clone(), &this_subs);

    assert_eq!(
        tricky_sub.get_equality(&a_eq_b, 5),
        Ok(Equal::Yes(
            subs(vec![(y.1.ptr_clone(), b.ptr_clone())]),
            Default::default()
        ))
    );
}

// DECISION[a b SELF c] = SELF (recursion)
// u should be
#[test]
fn recursion_is_tracked_in_decision() {
    let a = unique();
    let b = unique();
    let c = unique();
    let mut dec_rec = unique();
    let dec = Item::new_self_referencing(
        DDecision::new(a.ptr_clone(), b.ptr_clone(), c.ptr_clone(), c.ptr_clone()),
        Box::new(SRoot),
        |ptr, this| {
            let other = Item::new(DOther::new_recursive(ptr), SRoot);
            dec_rec = other.ptr_clone();
            this.set_when_equal(other);
        },
    );

    assert_eq!(
        dec.get_equality(&dec_rec, 3),
        Ok(Equal::Yes(subs(vec![]), vec![dec]))
    );
}
