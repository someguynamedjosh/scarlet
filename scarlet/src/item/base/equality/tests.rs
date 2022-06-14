#![cfg(test)]

use std::assert_matches::assert_matches;

use crate::{
    item::{
        definitions::substitution::{DSubstitution, Substitutions},
        equality::{Equal, EqualResult, EqualSuccess},
        test_util::*,
        util::*,
    },
    util::PtrExtension,
};

#[test]
fn something_equals_itself() {
    let thing = unique();
    assert_eq!(thing.get_trimmed_equality(&thing), Ok(Equal::yes()));
}

#[test]
fn variable_equals_something() {
    let (var_con, var_id) = variable_full();
    let thing = unique();
    let expected = subs(vec![(var_id, thing.ptr_clone())]);
    let result = Equal::yes1(expected.clone(), Default::default());
    assert_eq!(var_con.get_trimmed_equality(&thing), Ok(result));
}

#[test]
fn variable_equals_variable() {
    let x = variable_full();
    let y = variable_full();
    let expected = subs(vec![(x.1.ptr_clone(), y.0.ptr_clone())]);
    let left = Equal::yes1(expected.clone(), Default::default());
    assert_eq!(x.0.get_trimmed_equality(&y.0), Ok(left));
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
    assert_eq!(var_sub_thing.get_trimmed_equality(&thing), Ok(Equal::yes()));
    assert_eq!(thing.get_trimmed_equality(&var_sub_thing), Ok(Equal::yes()));
    assert_eq!(var_sub_thing.get_trimmed_equality(&another), Ok(Equal::No));
    assert_eq!(another.get_trimmed_equality(&var_sub_thing), Ok(Equal::No));
}

#[test]
fn decision_equals_identical_decision() {
    let a = unique();
    let b = unique();
    let c = unique();
    let d = unique();
    let dec1 = decision(a.ptr_clone(), b.ptr_clone(), c.ptr_clone(), d.ptr_clone());
    let dec2 = decision(a.ptr_clone(), b.ptr_clone(), c.ptr_clone(), d.ptr_clone());
    assert_eq!(dec1.get_trimmed_equality(&dec2), Ok(Equal::yes()));
    assert_eq!(dec2.get_trimmed_equality(&dec1), Ok(Equal::yes()));
}

#[test]
fn aabc_is_ddef() {
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
        dec1.get_trimmed_equality(&dec2),
        Ok(Equal::yes1(left_subs, Default::default()))
    );
    let right_subs = subs(vec![
        (d.1.ptr_clone(), a.0.ptr_clone()),
        (e.1.ptr_clone(), b.0.ptr_clone()),
        (f.1.ptr_clone(), c.0.ptr_clone()),
    ]);
    assert_eq!(
        dec2.get_trimmed_equality(&dec1),
        Ok(Equal::yes1(right_subs, Default::default()))
    );
}

#[test]
fn xxbc_is_aabc() {
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
        dec1.get_trimmed_equality(&dec2),
        Ok(Equal::yes1(left_subs, Default::default()))
    );
}

#[test]
fn aabc_eq_b_is_ddef_eq_e() {
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
        dec1.get_trimmed_equality(&dec2),
        Ok(Equal::yes1(left_subs, Default::default()))
    );
    let right_subs = subs(vec![
        (d.1.ptr_clone(), a.0.ptr_clone()),
        (e.1.ptr_clone(), b.0.ptr_clone()),
        (f.1.ptr_clone(), c.0.ptr_clone()),
    ]);
    assert_eq!(
        dec2.get_trimmed_equality(&dec1),
        Ok(Equal::yes1(right_subs, Default::default()))
    );
}

#[test]
fn decision_equals_decision_with_subs() {
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
        dec1.get_trimmed_equality(&dec2),
        Ok(Equal::yes1(subs.clone(), Default::default()))
    );
}

#[test]
fn fx_is_gy() {
    let x = variable_full();
    let y = variable_full();
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    assert_matches!(f.0.get_trimmed_equality(&g.0), Ok(Equal::Yes(..)));
    assert_matches!(g.0.get_trimmed_equality(&f.0), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(mut tsubs)) = f.0.get_trimmed_equality(&g.0) {
        assert_eq!(tsubs.len(), 1);
        let (lsubs, _) = tsubs.pop().unwrap();
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
        drop(last);
    } else {
        unreachable!()
    }
}

#[test]
fn a_is_y_sub_a() {
    let a = unique();
    let y = variable_full();

    let y_sub_a = unchecked_substitution_without_shortcuts(
        y.0,
        &subs(vec![(y.1.ptr_clone(), a.ptr_clone())]),
    );

    assert_eq!(a.get_equality_left(&y_sub_a), Ok(Equal::yes()));
    assert_eq!(a.get_trimmed_equality(&y_sub_a), Ok(Equal::yes()));
}

#[test]
fn fx_is_gy_sub_a() {
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
    let g_sub_a = unchecked_substitution(
        g.0.ptr_clone(),
        &subs(vec![(y.1.ptr_clone(), a.ptr_clone())]),
    );
    g_sub_a.set_name("g(a)".to_owned());
    assert_matches!(f.0.get_trimmed_equality(&g_sub_a), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(mut tsubs)) = f.0.get_equality_left(&g_sub_a) {
        assert_eq!(tsubs.len(), 1);
        let (lsubs, _) = tsubs.pop().unwrap();
        assert_eq!(lsubs.len(), 2);
        let mut entries = lsubs.iter();
        let first = entries.next().unwrap();
        assert_eq!(first.0.ptr_clone(), x.1.ptr_clone());
        assert_eq!(first.1.ptr_clone(), a.ptr_clone());
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
        drop(last);
    } else {
        unreachable!()
    }
}

#[test]
fn fx_sub_a_is_gy_sub_a() {
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
    f_sub_a.set_name("f(a)".to_owned());
    let g_sub_a = unchecked_substitution(
        g.0.ptr_clone(),
        &subs(vec![(y.1.ptr_clone(), a.ptr_clone())]),
    );
    g_sub_a.set_name("g(a)".to_owned());
    assert_matches!(f_sub_a.get_trimmed_equality(&g_sub_a), Ok(Equal::Yes(..)));
    assert_matches!(g_sub_a.get_trimmed_equality(&f_sub_a), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(mut tsubs)) = f_sub_a.get_trimmed_equality(&g_sub_a) {
        assert_eq!(tsubs.len(), 1);
        let (lsubs, _) = tsubs.pop().unwrap();
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
        drop(last);
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

    assert_eq!(
        gx.get_trimmed_equality(&fx_sub_gy),
        Ok(Equal::Yes(vec![
            (Default::default(), Default::default()),
            (Default::default(), Default::default())
        ]))
    );
    assert_eq!(
        fx_sub_gy.get_trimmed_equality(&gx),
        Ok(Equal::Yes(vec![
            (Default::default(), Default::default()),
            (Default::default(), Default::default()),
        ]))
    );
}

#[test]
fn fx_sub_nothing_is_gy_sub_nothing() {
    let x = variable_full();
    let y = variable_full();
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    let f_sub = unchecked_substitution(f.0.ptr_clone(), &Default::default());
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    let g_sub = unchecked_substitution(g.0.ptr_clone(), &Default::default());
    assert_matches!(f_sub.get_trimmed_equality(&g_sub), Ok(Equal::Yes(..)));
    assert_matches!(g_sub.get_trimmed_equality(&f_sub), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(mut tsubs)) = f_sub.get_trimmed_equality(&g_sub) {
        assert_eq!(tsubs.len(), 1);
        let (lsubs, _) = tsubs.pop().unwrap();
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
        drop(last);
    } else {
        unreachable!()
    }
}

#[test]
fn fx_sub_z_is_gy_sub_nothing() {
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
    assert_matches!(f_sub.get_trimmed_equality(&g_sub), Ok(Equal::Yes(..)));
    assert_matches!(g_sub.get_trimmed_equality(&f_sub), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(mut tsubs)) = f_sub.get_trimmed_equality(&g_sub) {
        assert_eq!(tsubs.len(), 1);
        let (lsubs, _) = tsubs.pop().unwrap();
        assert_eq!(lsubs.len(), 2);
        let mut entries = lsubs.iter();
        let first = entries.next().unwrap();
        assert_eq!(first.0, f.1.ptr_clone());
        if let Some(sub) = first.1.downcast_definition::<DSubstitution>() {
            assert_eq!(sub.base(), &g.0);
            assert_eq!(
                sub.substitutions(),
                &subs(vec![(y.1.ptr_clone(), x.0.ptr_clone())])
            )
        } else {
            panic!("Expected second substitution to be itself another substitution");
        }
        assert_eq!(entries.next(), Some(&(z.1.ptr_clone(), y.0.ptr_clone())));
    } else {
        unreachable!()
    }
}

#[test]
fn fx_sub_decision_is_gy_sub_decision() {
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

    assert_matches!(f_dec.get_trimmed_equality(&g_dec), Ok(Equal::Yes(..)));
    assert_matches!(g_dec.get_trimmed_equality(&f_dec), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(mut tsubs)) = f_dec.get_trimmed_equality(&g_dec) {
        assert_eq!(tsubs.len(), 1);
        let (lsubs, _) = tsubs.pop().unwrap();
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
        drop(last);
    } else {
        unreachable!()
    }
}

#[test]
fn dex_sub_decision_is_gy_sub_decision() {
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

    assert_matches!(g_dec.get_trimmed_equality(&dex_dec), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(mut tsubs)) = g_dec.get_trimmed_equality(&dex_dec) {
        assert_eq!(tsubs.len(), 1);
        let (lsubs, _) = tsubs.pop().unwrap();
        assert_eq!(lsubs.len(), 5);
        let mut entries = lsubs.iter();
        let first = entries.next().unwrap();
        assert_eq!(first.0, g.1.ptr_clone());
        if let Some(sub) = first.1.downcast_definition::<DSubstitution>() {
            assert_eq!(sub.base(), &dex);
            assert_eq!(
                sub.substitutions(),
                &subs(vec![(x.1.ptr_clone(), y.0.ptr_clone())])
            )
        } else {
            panic!("Expected first substitution to be itself another substitution");
        }
        assert_eq!(entries.next().unwrap(), &(s.1.ptr_clone(), a.ptr_clone()));
        assert_eq!(entries.next().unwrap(), &(t.1.ptr_clone(), b.ptr_clone()));
        assert_eq!(entries.next().unwrap(), &(u.1.ptr_clone(), c.ptr_clone()));
        assert_eq!(entries.next().unwrap(), &(v.1.ptr_clone(), d.ptr_clone()));
    } else {
        unreachable!()
    }
}

#[test]
fn fx_sub_decision_with_var_is_gy_sub_decision() {
    let z = variable_full();
    z.0.set_name(format!("z"));

    let a = unique();
    a.set_name(format!("a"));
    let b = unique();
    b.set_name(format!("b"));
    let c = unique();
    c.set_name(format!("c"));
    let d = unique();
    d.set_name(format!("d"));

    let dec = decision(z.0.ptr_clone(), b.ptr_clone(), c.ptr_clone(), d.ptr_clone());
    let x = variable_full();
    x.0.set_name(format!("x"));
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    f.0.set_name(format!("f"));
    let f_dec = unchecked_substitution(f.0.ptr_clone(), &subs(vec![(x.1.ptr_clone(), dec)]));

    let dec = decision(a.ptr_clone(), b.ptr_clone(), c.ptr_clone(), d.ptr_clone());
    let y = variable_full();
    y.0.set_name(format!("y"));
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    g.0.set_name(format!("g"));
    let g_dec = unchecked_substitution(g.0.ptr_clone(), &subs(vec![(y.1.ptr_clone(), dec)]));

    assert_matches!(f_dec.get_trimmed_equality(&g_dec), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(mut tsubs)) = f_dec.get_trimmed_equality(&g_dec) {
        assert_eq!(tsubs.len(), 1);
        let (lsubs, _) = tsubs.pop().unwrap();
        assert_eq!(lsubs.len(), 2);
        let mut entries = lsubs.iter();
        let next = entries.next().unwrap();
        assert_eq!(next.0, f.1.ptr_clone());
        if let Some(sub) = next.1.downcast_definition::<DSubstitution>() {
            assert_eq!(sub.base(), &g.0);
            assert_eq!(
                sub.substitutions(),
                &subs(vec![(y.1.ptr_clone(), x.0.ptr_clone())])
            )
        } else {
            panic!("Expected second substitution to be itself another substitution");
        }
        assert_eq!(Some(&(z.1.ptr_clone(), a.ptr_clone())), entries.next());
    } else {
        unreachable!()
    }
}

#[test]
fn fx_sub_gy_sub_a_is_gy_sub_a() {
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
    let f_sub_gx = unchecked_substitution(f.0.ptr_clone(), &subs(vec![(f.1.ptr_clone(), gx)])); // 20
    let f_sub_gx_sub_a =
        unchecked_substitution(f_sub_gx, &subs(vec![(x.1.ptr_clone(), a.ptr_clone())]));

    // 21
    let gy_sub_a = unchecked_substitution(
        g.0.ptr_clone(),
        &subs(vec![(y.1.ptr_clone(), a.ptr_clone())]),
    );

    assert_eq!(
        f_sub_gx_sub_a.get_trimmed_equality(&gy_sub_a),
        Ok(Equal::yes())
    );
    assert_eq!(
        gy_sub_a.get_trimmed_equality(&f_sub_gx_sub_a),
        Ok(Equal::Yes(vec![
            (Default::default(), Default::default()),
            (Default::default(), Default::default())
        ]))
    );
}

#[test]
fn fx_sub_a_sub_gy_is_gy_sub_a() {
    let a = unique();
    a.set_name("a".to_owned());

    let x = variable_full();
    x.0.set_name("x".to_owned());
    let y = variable_full();
    y.0.set_name("y".to_owned());
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    g.0.set_name("gy".to_owned());
    let gx = unchecked_substitution(
        g.0.ptr_clone(),
        &subs(vec![(y.1.ptr_clone(), x.0.ptr_clone())]),
    );
    gx.set_name("gy(x)".to_owned());

    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    f.0.set_name("fx".to_owned());
    let f_sub_a = unchecked_substitution(
        f.0.ptr_clone(),
        &subs(vec![(x.1.ptr_clone(), a.ptr_clone())]),
    );
    f_sub_a.set_name("fx(a)".to_owned());
    let f_sub_a_sub_gy = unchecked_substitution(f_sub_a, &subs(vec![(f.1.ptr_clone(), gx)]));
    f_sub_a_sub_gy.set_name("fx(a)(fx IS gy(x))".to_owned());

    let gy_sub_a = unchecked_substitution(
        g.0.ptr_clone(),
        &subs(vec![(y.1.ptr_clone(), a.ptr_clone())]),
    );
    gy_sub_a.set_name("gy(a)".to_owned());

    // assert_eq!(
    //     f_sub_a_sub_gy.get_trimmed_equality(&gy_sub_a),
    //     Ok(Equal::Yes(vec![
    //         (Default::default(), Default::default()),
    //         (Default::default(), Default::default())
    //     ]))
    // );
    println!("{:#?}", gy_sub_a.get_trimmed_equality(&f_sub_a_sub_gy));
    // gy IS fx(y)(fx IS gy(x))
    if let Ok(Equal::Yes(cases)) = gy_sub_a.get_trimmed_equality(&f_sub_a_sub_gy) {
        assert!(cases.iter().any(|x| x.0.len() == 0 && x.1.len() == 0));
    } else {
        panic!("Expected a positive result.");
    }
}

#[test]
fn fx_sub_y_sub_gx_is_gy() {
    let a = unique();
    a.set_name("a".to_owned());

    let x = variable_full();
    x.0.set_name("x".to_owned());
    let y = variable_full();
    y.0.set_name("y".to_owned());
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    g.0.set_name("gy".to_owned());
    let gx = unchecked_substitution(
        g.0.ptr_clone(),
        &subs(vec![(y.1.ptr_clone(), x.0.ptr_clone())]),
    );
    gx.set_name("gy(x)".to_owned());

    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    f.0.set_name("fx".to_owned());
    let f_sub_y = unchecked_substitution(
        f.0.ptr_clone(),
        &subs(vec![(x.1.ptr_clone(), y.0.ptr_clone())]),
    );
    f_sub_y.set_name("fx(y)".to_owned());
    let f_sub_y_sub_gx = unchecked_substitution(f_sub_y, &subs(vec![(f.1.ptr_clone(), gx)]));
    f_sub_y_sub_gx.set_name("fx(y)(fx IS gy(x))".to_owned());

    println!("{:#?}", f_sub_y_sub_gx.get_equality_left(&g.0));
    assert_eq!(
        f_sub_y_sub_gx.get_trimmed_equality(&g.0),
        Ok(Equal::Yes(vec![
            (Default::default(), Default::default()),
            (Default::default(), Default::default())
        ]))
    );
}

// This case cannot be detected with the new system.
// #[test]
// fn z_equal_z_is_x_eq_y_sub_true_true() {
//     let truee = unique();
//     truee.set_name("true".to_owned());
//     let falsee = unique();
//     truee.set_name("false".to_owned());

//     let x = variable_full();
//     x.0.set_name("x".to_owned());
//     let y = variable_full();
//     y.0.set_name("y".to_owned());
//     let z = variable_full();
//     z.0.set_name("z".to_owned());

//     let x_eq_y = decision(
//         x.0.ptr_clone(),
//         y.0.ptr_clone(),
//         truee.ptr_clone(),
//         falsee.ptr_clone(),
//     );
//     x_eq_y.set_name("x=y".to_owned());
//     let true_eq_true = unchecked_substitution(
//         x_eq_y,
//         &subs(vec![
//             (x.1.ptr_clone(), truee.ptr_clone()),
//             (y.1.ptr_clone(), truee.ptr_clone()),
//         ]),
//     );
//     true_eq_true.set_name("true=true".to_owned());
//     let z_eq_z = decision(
//         z.0.ptr_clone(),
//         z.0.ptr_clone(),
//         truee.ptr_clone(),
//         falsee.ptr_clone(),
//     );
//     z_eq_z.set_name("z=z".to_owned());

//     assert_matches!(
//         z_eq_z.get_trimmed_equality(&true_eq_true),
//         Ok(Equal::Yes(..))
//     );
//     if let Ok(Equal::Yes(lsubs, _)) =
// z_eq_z.get_trimmed_equality(&true_eq_true) {
// assert_eq!(lsubs.len(), // 1);         let mut entries = lsubs.iter();
//         let last = entries.next().unwrap();
//         assert_eq!(last, &(z.1.ptr_clone(), truee.ptr_clone()));
//     } else {
//         unreachable!()
//     }
// }

#[test]
fn is_bool_sub_y_is_y_is_bool() {
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
        y_is_bool.get_trimmed_equality(&x_sub_y_is_bool),
        Ok(Equal::yes())
    );
}

/// f[z] <=> DECISION[x y a b]
#[test]
fn multi_variable_dex_is_single_variable_dex() {
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

    if let Equal::Yes(mut subs) =
        fz.0.ptr_clone()
            .get_trimmed_equality(&multi_variable_dex)
            .unwrap()
    {
        assert_eq!(subs.len(), 1);
        let (lsubs, _) = subs.pop().unwrap();
        assert_eq!(lsubs.len(), 2);
        let sub = lsubs.get(&z.1.ptr_clone()).unwrap().ptr_clone();
        assert_eq!(sub, x.0.ptr_clone());
        let sub = lsubs.get(&fz.1.ptr_clone()).unwrap().ptr_clone();
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
    let a = unique();
    a.set_name("a".to_owned());
    let b = unique();
    b.set_name("b".to_owned());
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
    multi_variable_dex.set_name("DECISION(x y a b)".to_owned());
    let subbed_multi_variable_dex = unchecked_substitution(
        multi_variable_dex.ptr_clone(),
        &subs(vec![(x.1.ptr_clone(), a.ptr_clone())]),
    );
    subbed_multi_variable_dex.set_name("DECISION(x y a b)(x IS a)".to_owned());

    if let Equal::Yes(mut subs) =
        fz.0.ptr_clone()
            .get_trimmed_equality(&subbed_multi_variable_dex)
            .unwrap()
    {
        assert_eq!(subs.len(), 2);

        let (lsubs, _) = subs.pop().unwrap();
        assert_eq!(lsubs.len(), 2);
        let sub = lsubs.get(&z.1).unwrap().ptr_clone();
        assert_eq!(sub, a.ptr_clone());
        let sub = lsubs.get(&fz.1).unwrap();
        if let Some(def) = sub.downcast_definition::<DSubstitution>() {
            let def = def.clone();
            let mut expected = Substitutions::new();
            expected.insert_no_replace(x.1.ptr_clone(), z.0.ptr_clone());
            assert_eq!(def.substitutions(), &expected);
            assert_eq!(
                def.base().get_trimmed_equality(&multi_variable_dex),
                Ok(Equal::yes())
            );
        } else {
            panic!("Substituted value is not itself a substitution!");
        }
        drop(sub);

        let (lsubs, _) = subs.pop().unwrap();
        assert_eq!(lsubs.len(), 2);
        let sub = lsubs.get(&z.1).unwrap().ptr_clone();
        assert_eq!(sub, y.0.ptr_clone());
        let sub = lsubs.get(&fz.1).unwrap();
        if let Some(def) = sub.downcast_definition::<DSubstitution>() {
            let def = def.clone();
            let mut expected = Substitutions::new();
            expected.insert_no_replace(y.1.ptr_clone(), z.0.ptr_clone());
            assert_eq!(def.substitutions(), &expected);
            assert_eq!(
                def.base().get_trimmed_equality(&subbed_multi_variable_dex),
                Ok(Equal::yes())
            );
        } else {
            panic!("Substituted value is not itself a substitution!");
        }
        drop(sub);
    } else {
        panic!("Not equal!");
    }
}

/// f[z] <=> DECISION[x y a b][x2 y2]
#[test]
fn multi_variable_dex_sub_two_vars_is_single_variable_dex() {
    let a = unique();
    a.set_name("a".to_owned());
    let b = unique();
    b.set_name("b".to_owned());
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
    multi_variable_dex.set_name("DECISION(x y a b)".to_owned());
    let subbed_multi_variable_dex = unchecked_substitution(
        multi_variable_dex.ptr_clone(),
        &subs(vec![
            (x.1.ptr_clone(), x2.0.ptr_clone()),
            (y.1.ptr_clone(), y2.0.ptr_clone()),
        ]),
    );
    subbed_multi_variable_dex.set_name("DECISION(x y a b)(x IS x2   y IS y2)".to_owned());

    if let Equal::Yes(mut subs) =
        fz.0.ptr_clone()
            .get_trimmed_equality(&subbed_multi_variable_dex)
            .unwrap()
    {
        println!("{:#?}", subs);
        assert_eq!(subs.len(), 2);

        let (lsubs, _) = subs.pop().unwrap();
        assert_eq!(lsubs.len(), 2);
        let sub = lsubs.get(&z.1).unwrap();
        assert_eq!(sub, &x2.0);
        let sub = lsubs.get(&fz.1).unwrap();
        println!("{:#?}", sub);
        if let Some(def) = sub.downcast_definition::<DSubstitution>() {
            let def = def.clone();
            let mut expected = Substitutions::new();
            expected.insert_no_replace(y.1.ptr_clone(), y2.0.ptr_clone());
            assert_eq!(def.substitutions(), &expected);
            if let Some(def) = def.base().clone().downcast_definition::<DSubstitution>() {
                let mut expected = Substitutions::new();
                expected.insert_no_replace(x.1.ptr_clone(), z.0.ptr_clone());
                assert_eq!(def.substitutions(), &expected);
                assert_eq!(
                    def.base().get_trimmed_equality(&multi_variable_dex),
                    Ok(Equal::yes())
                );
            } else {
                panic!("Substituted value is not itself a substitution!");
            }
        } else {
            panic!("Substituted value is not itself a substitution!");
        }
        drop(sub);

        let (lsubs, _) = subs.pop().unwrap();
        assert_eq!(lsubs.len(), 2);
        let sub = lsubs.get(&z.1).unwrap();
        assert_eq!(sub, &x2.0);
        let sub = lsubs.get(&fz.1).unwrap();
        if let Some(def) = sub.downcast_definition::<DSubstitution>() {
            let def = def.clone();
            let mut expected = Substitutions::new();
            expected.insert_no_replace(x2.1.ptr_clone(), z.0.ptr_clone());
            assert_eq!(def.substitutions(), &expected);
            assert_eq!(
                def.base().get_trimmed_equality(&subbed_multi_variable_dex),
                Ok(Equal::yes())
            );
        } else {
            panic!("Substituted value is not itself a substitution!");
        }
        drop(sub);
    } else {
        panic!("Not equal!");
    }
}

/// fz <=> DECISION[x y a b][a b]
#[test]
fn single_variable_dex_is_multi_variable_dex_sub_two_uniques() {
    let a = unique();
    a.set_name("a".to_owned());
    let b = unique();
    b.set_name("b".to_owned());
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

    if let Equal::Yes(mut subs) =
        fz.0.ptr_clone()
            .get_trimmed_equality(&subbed_multi_variable_dex)
            .unwrap()
    {
        assert_eq!(subs.len(), 1);
        let (lsubs, _) = subs.pop().unwrap();
        assert_eq!(lsubs.len(), 2);
        let sub = lsubs.get(&z.1).unwrap();
        assert_eq!(sub, &a);
        let sub = lsubs.get(&fz.1).unwrap();
        // DECISION{x y a b}(x IS z)(y IS b)
        println!("{:#?}", sub);
        if let Some(def) = sub.downcast_definition::<DSubstitution>() {
            let def = def.clone();
            let mut expected = Substitutions::new();
            expected.insert_no_replace(y.1.ptr_clone(), b.ptr_clone());
            assert_eq!(def.substitutions(), &expected);
            if let Some(def) = def.base().downcast_definition::<DSubstitution>() {
                let def = def.clone();
                assert_eq!(def.substitutions().len(), 1);

                assert!((*def.substitutions().get(&x.1).unwrap()).is_same_instance_as(&z.0),);
                assert_eq!(
                    def.base().get_trimmed_equality(&multi_variable_dex),
                    Ok(Equal::yes())
                );
            } else {
                panic!("Expected another substitution!");
            }
            drop(def);
        } else {
            panic!("Substituted value is not itself a substitution!");
        }
        drop(sub);
    } else {
        panic!("Not equal!");
    }
}

/// fx(fx IS x = y   x IS a   y IS b) <=/=> a = b
#[test]
fn sneaky_substitution() {
    let a = unique();
    let b = unique();
    let t = unique();
    let f = unique();
    let x = variable_full();
    x.0.set_name("x".to_owned());
    let y = variable_full();
    y.0.set_name("y".to_owned());

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

    // Currently we can't determine that (x IS a   y IS b) solves this
    // equality.     assert_eq!(tricky_sub.get_trimmed_equality(&a_eq_b),
    // Ok(Equal::Unknown)); }
}

#[test]
fn x_x_is_y_y() {
    let x = variable_full();
    x.0.set_name("x".to_owned());
    let y = variable_full();
    y.0.set_name("y".to_owned());
    let void = unique();

    let x_x = structt(vec![("", x.0.ptr_clone()), ("", x.0.ptr_clone())], &void);
    x_x.set_name(format!("{{x x}}"));
    let y_y = structt(vec![("", y.0.ptr_clone()), ("", y.0.ptr_clone())], &void);
    y_y.set_name(format!("{{y y}}"));

    assert_matches!(x_x.get_trimmed_equality(&y_y), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(mut subs)) = x_x.get_trimmed_equality(&y_y) {
        assert_eq!(subs.len(), 1);
        let (lsubs, _) = subs.pop().unwrap();
        assert_eq!(lsubs.len(), 1);
        let mut entries = lsubs.iter();
        let last = entries.next().unwrap();
        assert_eq!(last, &(x.1.ptr_clone(), y.0.ptr_clone()));
    } else {
        unreachable!()
    }
}

#[test]
fn x_x_is_y_other_y_other() {
    let x = variable_full();
    x.0.set_name("x".to_owned());
    let y = variable_full();
    y.0.set_name("y".to_owned());
    let void = unique();

    let x_x = structt(vec![("", x.0.ptr_clone()), ("", x.0.ptr_clone())], &void);
    x_x.set_name(format!("{{x x}}"));
    let y_other_0 = other(y.0.ptr_clone());
    let y_other_1 = other(y.0.ptr_clone());
    let y_y = structt(
        vec![("", y_other_0.ptr_clone()), ("", y_other_1.ptr_clone())],
        &void,
    );
    y_y.set_name(format!("{{y y}}"));

    assert_matches!(x_x.get_trimmed_equality(&y_y), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(mut subs)) = x_x.get_trimmed_equality(&y_y) {
        assert_eq!(subs.len(), 1);
        let (lsubs, _) = subs.pop().unwrap();
        assert_eq!(lsubs.len(), 1);
        let mut entries = lsubs.iter();
        let last = entries.next().unwrap();
        assert_eq!(last, &(x.1.ptr_clone(), y.0.ptr_clone()));
    } else {
        unreachable!()
    }
}

#[test]
fn fx_y_is_self() {
    let y = variable_full();
    let x = variable_full();
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    let fa = unchecked_substitution(
        f.0.ptr_clone(),
        &subs(vec![(x.1.ptr_clone(), y.0.ptr_clone())]),
    );
    let other = other(fa.ptr_clone());
    assert_eq!(fa.get_trimmed_equality(&other), Ok(Equal::yes()));
}

#[test]
fn tricky_fx_y_is_self() {
    let x = variable_full();
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    let y = variable_full();
    let fa = unchecked_substitution(
        f.0.ptr_clone(),
        &subs(vec![(x.1.ptr_clone(), y.0.ptr_clone())]),
    );
    let other = other(fa.ptr_clone());
    assert_eq!(fa.get_trimmed_equality(&other), Ok(Equal::yes()));
}

#[test]
fn other_fx_y_is_self() {
    let y = variable_full();
    let x = variable_full();
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    let fa = unchecked_substitution(
        f.0.ptr_clone(),
        &subs(vec![(x.1.ptr_clone(), y.0.ptr_clone())]),
    );
    let other_base = other(fa.ptr_clone());
    let other_0 = other(other_base.ptr_clone());
    let other_1 = other(other_base.ptr_clone());
    assert_eq!(other_0.get_trimmed_equality(&other_1), Ok(Equal::yes()));
}

#[test]
fn other_fx_is_self() {
    let x = variable_full();
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    let other_base = other(f.0.ptr_clone());
    let other_0 = other(other_base.ptr_clone());
    let other_1 = other(other_base.ptr_clone());
    assert_eq!(other_0.get_trimmed_equality(&other_1), Ok(Equal::yes()));
}

#[test]
fn fx_sub_a_z_is_gy_sub_a_z() {
    let void = unique();
    void.set_name("void".to_owned());
    let a = unique();
    a.set_name("a".to_owned());
    let x = variable_full();
    x.0.set_name("x".to_owned());
    let y = variable_full();
    y.0.set_name("y".to_owned());
    let z = variable_full();
    z.0.set_name("z".to_owned());
    let f = variable_full_with_deps(vec![x.0.ptr_clone()]);
    f.0.set_name("f".to_owned());
    let g = variable_full_with_deps(vec![y.0.ptr_clone()]);
    g.0.set_name("g".to_owned());
    let a_z = structt(vec![("", a.ptr_clone()), ("", z.0.ptr_clone())], &void);
    let f_sub_a_z = unchecked_substitution(
        f.0.ptr_clone(),
        &subs(vec![(x.1.ptr_clone(), a_z.ptr_clone())]),
    );
    f_sub_a_z.set_name("f({a z})".to_owned());
    let g_sub_a_z = unchecked_substitution(
        g.0.ptr_clone(),
        &subs(vec![(y.1.ptr_clone(), a_z.ptr_clone())]),
    );
    g_sub_a_z.set_name("g({a z})".to_owned());
    assert_matches!(
        f_sub_a_z.get_trimmed_equality(&g_sub_a_z),
        Ok(Equal::Yes(..))
    );
    assert_matches!(
        g_sub_a_z.get_trimmed_equality(&f_sub_a_z),
        Ok(Equal::Yes(..))
    );
    if let Ok(Equal::Yes(tsubs)) = f_sub_a_z.get_trimmed_equality(&g_sub_a_z) {
        assert_eq!(tsubs.iter().filter(|x| x.1.len() == 0).count(), 1);
        let (lsubs, _) = tsubs.into_iter().filter(|x| x.1.len() == 0).next().unwrap();
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
        drop(last);
    } else {
        unreachable!()
    }
}

#[test]
fn equality_symmetry() {
    let t = unique();
    t.set_name("true".to_owned());
    let f = unique();
    f.set_name("false".to_owned());
    let identity = variable_full();
    identity.0.set_name("identity".to_owned());
    let y = variable_full();
    y.0.set_name("y".to_owned());
    let z = variable_full();
    z.0.set_name("z".to_owned());
    let x = variable_full();
    x.0.set_name("x".to_owned());
    let fx = variable_full_with_deps(vec![x.0.ptr_clone()]);
    fx.0.set_name("f".to_owned());
    let identity_x = unchecked_substitution_without_shortcuts(
        identity.0.ptr_clone(),
        &subs(vec![(identity.1.ptr_clone(), x.0.ptr_clone())]),
    );
    let fy = unchecked_substitution(
        fx.0.ptr_clone(),
        &subs(vec![(x.1.ptr_clone(), y.0.ptr_clone())]),
    );
    fy.set_name("f(y)".to_owned());
    let fz = unchecked_substitution(
        fx.0.ptr_clone(),
        &subs(vec![(x.1.ptr_clone(), z.0.ptr_clone())]),
    );
    fz.set_name("f(z)".to_owned());
    let fz_eq_fy = decision(fz.ptr_clone(), fy.ptr_clone(), t.ptr_clone(), f.ptr_clone());
    let fz_eq_fy_sub_f_is_identity = unchecked_substitution(
        fz_eq_fy.ptr_clone(),
        &subs(vec![(fx.1.ptr_clone(), identity_x.ptr_clone())]),
    );
    let z_eq_y = decision(
        z.0.ptr_clone(),
        y.0.ptr_clone(),
        t.ptr_clone(),
        f.ptr_clone(),
    );

    println!(
        "{:#?}",
        fz_eq_fy_sub_f_is_identity.get_trimmed_equality(&z_eq_y)
    );
    assert_eq!(
        fz_eq_fy_sub_f_is_identity
            .get_trimmed_equality(&z_eq_y)
            .as_ref()
            .map(Equal::is_trivial_yes),
        Ok(true)
    );
    assert_eq!(
        z_eq_y
            .get_trimmed_equality(&fz_eq_fy_sub_f_is_identity)
            .as_ref()
            .map(Equal::is_trivial_yes),
        Ok(true)
    );
}

#[test]
fn complex_fx_sub_is_arg_env() {
    let code = r"
    y IS VAR[]
    z IS VAR[y = SELF]

    x IS VAR[]
    fx IS VAR[DEP x]

    u IS VAR[]
    v IS VAR[u = SELF]
    identity IS VAR[]

    v1 IS fx[x IS z][z IS v   fx IS identity[identity IS x]]
    v2 IS v
    ";
    with_env_from_code(code, |_, root| {
        let v1 = get_member(&root, "v1");
        let v2 = get_member(&root, "v2");
        assert_eq!(
            v1.get_trimmed_equality(&v2)
                .as_ref()
                .map(Equal::is_trivial_yes),
            Ok(true)
        );
        assert_eq!(
            v2.get_trimmed_equality(&v1)
                .as_ref()
                .map(Equal::is_trivial_yes),
            Ok(true)
        );
    });
}

#[test]
fn advanced_equality_symmetry_env() {
    let code = r"
    y IS VAR[]
    z IS VAR[y = SELF]

    x IS VAR[]
    fx IS VAR[DEP x]

    statement IS fx[z] = fx[y]

    u IS VAR[]
    v IS VAR[u = SELF]
    identity IS VAR[]

    v1 IS statement[u v identity]
    v2 IS v = u
    ";
    with_env_from_code(code, |_, root| {
        let v1 = get_member(&root, "v1");
        let v2 = get_member(&root, "v2");
        assert_eq!(
            v1.get_trimmed_equality(&v2)
                .as_ref()
                .map(Equal::is_trivial_yes),
            Ok(true)
        );
        assert_eq!(
            v2.get_trimmed_equality(&v1)
                .as_ref()
                .map(Equal::is_trivial_yes),
            Ok(true)
        );
    });
}

#[test]
fn fx_eq_a_sub_y_is_self() {
    let a = unique();
    a.set_name("a".to_owned());
    let t = unique();
    t.set_name("true".to_owned());
    let f = unique();
    f.set_name("false".to_owned());
    let y = variable_full();
    y.0.set_name("y".to_owned());
    let x = variable_full();
    x.0.set_name("x".to_owned());
    let fx = variable_full_with_deps(vec![x.0.ptr_clone()]);
    fx.0.set_name("f".to_owned());
    let fx_eq_a = decision(
        fx.0.ptr_clone(),
        a.ptr_clone(),
        t.ptr_clone(),
        f.ptr_clone(),
    );
    fx_eq_a.set_name("f = a".to_owned());
    let fx_eq_a_sub_y_1 = unchecked_substitution(
        other(fx_eq_a.ptr_clone()),
        &subs(vec![(x.1.ptr_clone(), y.0.ptr_clone())]),
    );
    fx_eq_a_sub_y_1.set_name("(f = a)(x IS y) 1".to_owned());
    let fx_eq_a_sub_y_2 = unchecked_substitution(
        other(fx_eq_a.ptr_clone()),
        &subs(vec![(x.1.ptr_clone(), y.0.ptr_clone())]),
    );
    fx_eq_a_sub_y_2.set_name("(f = a)(x IS y) 2".to_owned());

    assert_eq!(
        fx_eq_a_sub_y_1
            .get_trimmed_equality(&fx_eq_a_sub_y_2)
            .as_ref()
            .map(Equal::is_trivial_yes),
        Ok(true)
    );
    assert_eq!(
        fx_eq_a_sub_y_2
            .get_trimmed_equality(&fx_eq_a_sub_y_1)
            .as_ref()
            .map(Equal::is_trivial_yes),
        Ok(true)
    );
}

#[test]
fn fx_eq_a_is_self_sub_y_env() {
    let code = r"
    a IS UNIQUE

    x IS VAR[].AS_LANGUAGE_ITEM[x]
    fx IS VAR[DEP x]

    y IS VAR[]
    gy IS VAR[DEP y]

    statement IS fx = a

    t IS VAR[]

    # x -> t
    # fx -> fx(x IS t)
    v1 IS statement
    v2 IS statement[x IS t]

    # # x -> t
    # # fx -> gy(y IS x)
    # v1 IS fx = a
    # v2 IS (gy = a)[y IS t]
    ";
    with_env_from_code(code, |_, root| {
        let x = get_member(&root, "x");
        let t = get_member(&root, "t");
        let v1 = get_member(&root, "v1");
        let v2 = get_member(&root, "v2");
        if let Ok(Equal::Yes(mut cases)) = v1.get_trimmed_equality(&v2) {
            assert_eq!(cases.len(), 1);
            let (lsubs, _) = cases.pop().unwrap();
            assert_eq!(lsubs.len(), 1);
            let (target, value) = lsubs.into_iter().next().unwrap();
            assert!(target.borrow().item().is_same_instance_as(&x.dereference()));
            assert!(value.is_same_instance_as(&t.dereference()));
        } else {
            panic!("Expected values to be equal.");
        }
    });
}

#[test]
fn fx_eq_a_sub_y_is_self_env() {
    let code = r"
    a IS UNIQUE

    x IS VAR[].AS_LANGUAGE_ITEM[x]
    fx IS VAR[DEP x]

    statement IS fx = a

    t IS VAR[]
    s1 IS statement[x IS t]
    s2 IS statement[x IS t]

    v1 IS statement
    v2 IS statement[x IS t]
    ";
    with_env_from_code(code, |_, root| {
        let v1 = get_member(&root, "v1");
        let v2 = get_member(&root, "v2");
        let fx_eq_a_sub_y_1 = get_member(&root, "s1");
        let fx_eq_a_sub_y_2 = get_member(&root, "s2");
        println!("{:#?}", v1.get_trimmed_equality(&v2));
        assert_eq!(
            fx_eq_a_sub_y_1
                .get_trimmed_equality(&fx_eq_a_sub_y_2)
                .as_ref()
                .map(Equal::is_trivial_yes),
            Ok(true)
        );
        assert_eq!(
            fx_eq_a_sub_y_2
                .get_trimmed_equality(&fx_eq_a_sub_y_1)
                .as_ref()
                .map(Equal::is_trivial_yes),
            Ok(true)
        );
    });
}
