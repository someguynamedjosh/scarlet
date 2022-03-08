#![cfg(test)]

use std::assert_matches::assert_matches;

use crate::{
    constructs::substitution::CSubstitution,
    environment::{discover_equality::Equal, test_util::*},
    scope::SRoot,
};

#[test]
fn something_equals_itself() {
    let mut env = env();
    let thing = env.unique();
    assert_eq!(env.discover_equal(thing, thing, 0), Ok(Equal::yes()));
}

#[test]
fn something_equals_variable() {
    let mut env = env();
    let thing = env.unique();
    let (var_con, var_id) = env.variable_full();
    let expected = subs(vec![(var_id, thing)]);
    let left = Equal::Yes(expected.clone());
    assert_eq!(env.discover_equal(var_con, thing, 1), Ok(left));
    assert_eq!(
        env.discover_equal(var_con, thing, 0),
        Ok(Equal::NeedsHigherLimit)
    );
    assert_eq!(
        env.discover_equal(thing, var_con, 0),
        Ok(Equal::NeedsHigherLimit)
    );
}

#[test]
fn var_sub_something_equals_something() {
    let mut env = env();
    let thing = env.unique();
    let another = env.unique();
    let (var_con, var_id) = env.variable_full();
    let var_sub_thing = env.substitute(var_con, &subs(vec![(var_id, thing)]));
    assert_eq!(
        env.discover_equal(var_sub_thing, thing, 2),
        Ok(Equal::yes())
    );
    assert_eq!(
        env.discover_equal(thing, var_sub_thing, 2),
        Ok(Equal::yes())
    );
    assert_eq!(env.discover_equal(var_sub_thing, another, 2), Ok(Equal::No));
    assert_eq!(env.discover_equal(another, var_sub_thing, 2), Ok(Equal::No));
}

#[test]
fn decision_equals_identical_decision() {
    let mut env = env();
    let a = env.variable();
    let b = env.variable();
    let c = env.variable();
    let d = env.variable();
    let dec1 = env.decision(a, b, c, d);
    let dec2 = env.decision(a, b, c, d);
    assert_eq!(env.discover_equal(dec1, dec2, 2), Ok(Equal::yes()));
    assert_eq!(env.discover_equal(dec2, dec1, 2), Ok(Equal::yes()));
}

#[test]
fn aabc_is_ddef() {
    let mut env = env();
    let a = env.variable_full();
    let b = env.variable_full();
    let c = env.variable_full();
    let d = env.variable_full();
    let e = env.variable_full();
    let f = env.variable_full();
    let dec1 = env.decision(a.0, a.0, b.0, c.0);
    let dec2 = env.decision(d.0, d.0, e.0, f.0);
    let left_subs = subs(vec![(a.1, d.0), (b.1, e.0), (c.1, f.0)]);
    assert_eq!(env.discover_equal(dec1, dec2, 3), Ok(Equal::Yes(left_subs)));
    let right_subs = subs(vec![(d.1, a.0), (e.1, b.0), (f.1, c.0)]);
    assert_eq!(
        env.discover_equal(dec2, dec1, 3),
        Ok(Equal::Yes(right_subs))
    );
}

#[test]
fn xxbc_is_aabc() {
    let mut env = env();
    let a = env.unique();
    let b = env.unique();
    let c = env.unique();
    let x = env.variable_full();
    let dec1 = env.decision(x.0, x.0, b, c);
    let dec2 = env.decision(a, a, b, c);
    let left_subs = subs(vec![(x.1, a)]);
    assert_eq!(env.discover_equal(dec1, dec2, 3), Ok(Equal::Yes(left_subs)));
}

#[test]
fn aabc_eq_b_is_ddef_eq_e() {
    let mut env = env();
    let truee = env.unique();
    let falsee = env.unique();
    let a = env.variable_full();
    let b = env.variable_full();
    let c = env.variable_full();
    let d = env.variable_full();
    let e = env.variable_full();
    let f = env.variable_full();
    let dec1 = env.decision(a.0, a.0, b.0, c.0);
    let dec1 = env.decision(dec1, b.0, truee, falsee);
    let dec2 = env.decision(d.0, d.0, e.0, f.0);
    let dec2 = env.decision(dec2, e.0, truee, falsee);
    let left_subs = subs(vec![(a.1, d.0), (b.1, e.0), (c.1, f.0)]);
    assert_eq!(env.discover_equal(dec1, dec2, 3), Ok(Equal::Yes(left_subs)));
    let right_subs = subs(vec![(d.1, a.0), (e.1, b.0), (f.1, c.0)]);
    assert_eq!(
        env.discover_equal(dec2, dec1, 3),
        Ok(Equal::Yes(right_subs))
    );
}

#[test]
fn decision_equals_decision_with_subs() {
    let mut env = env();
    let a = env.variable_full();
    let b = env.variable_full();
    let c = env.variable_full();
    let d = env.variable_full();
    let w = env.unique();
    let x = env.unique();
    let y = env.unique();
    let z = env.unique();
    let dec1 = env.decision(a.0, b.0, c.0, d.0);
    let dec2 = env.decision(w, x, y, z);
    let subs = subs(vec![(a.1, w), (b.1, x), (c.1, y), (d.1, z)]);
    assert_eq!(
        env.discover_equal(dec1, dec2, 2),
        Ok(Equal::Yes(subs.clone()))
    );
}

#[test]
fn fx_is_gy() {
    let mut env = env();
    let x = env.variable_full();
    let y = env.variable_full();
    let f = env.variable_full_with_deps(vec![x.0]);
    let g = env.variable_full_with_deps(vec![y.0]);
    assert_matches!(env.discover_equal(f.0, g.0, 2), Ok(Equal::Yes(..)));
    assert_matches!(env.discover_equal(g.0, f.0, 2), Ok(Equal::Yes(..)));
    assert_matches!(env.discover_equal(g.0, f.0, 1), Ok(Equal::NeedsHigherLimit));
    if let Ok(Equal::Yes(lsubs)) = env.discover_equal(f.0, g.0, 2) {
        assert_eq!(lsubs.len(), 2);
        let mut entries = lsubs.iter();
        assert_eq!(entries.next(), Some(&(x.1, y.0)));
        let last = entries.next().unwrap();
        assert_eq!(last.0, f.1);
        if let Ok(Some(sub)) = env.get_and_downcast_construct_definition::<CSubstitution>(last.1) {
            assert_eq!(sub.base(), g.0);
            assert_eq!(sub.substitutions(), &subs(vec![(y.1, x.0)]))
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
    let a = env.unique();
    let x = env.variable_full();
    let y = env.variable_full();
    let f = env.variable_full_with_deps(vec![x.0]);
    let g = env.variable_full_with_deps(vec![y.0]);
    let f_sub_a = env.substitute(f.0, &subs(vec![(x.1, a)]));
    let g_sub_a = env.substitute(g.0, &subs(vec![(y.1, a)]));
    assert_matches!(env.discover_equal(f_sub_a, g_sub_a, 2), Ok(Equal::Yes(..)));
    assert_matches!(env.discover_equal(g_sub_a, f_sub_a, 2), Ok(Equal::Yes(..)));
    assert_matches!(
        env.discover_equal(g_sub_a, f_sub_a, 1),
        Ok(Equal::NeedsHigherLimit)
    );
    if let Ok(Equal::Yes(lsubs)) = env.discover_equal(f_sub_a, g_sub_a, 2) {
        assert_eq!(lsubs.len(), 1);
        let mut entries = lsubs.iter();
        let last = entries.next().unwrap();
        assert_eq!(last.0, f.1);
        if let Ok(Some(sub)) = env.get_and_downcast_construct_definition::<CSubstitution>(last.1) {
            assert_eq!(sub.base(), g.0);
            assert_eq!(sub.substitutions(), &subs(vec![(y.1, x.0)]))
        } else {
            panic!("Expected substitution to be itself another substitution");
        }
    } else {
        unreachable!()
    }
}

#[test]
fn fx_sub_gy_is_gy_sub_x() {
    let mut env = env();
    let x = env.variable_full(); // 13/0
    let y = env.variable_full(); // 14/1
    let f = env.variable_full_with_deps(vec![x.0]); // 15/2
    let g = env.variable_full_with_deps(vec![y.0]); // 16/3

    let gx = env.substitute(g.0, &subs(vec![(y.1, x.0)])); // 17
    let fx_sub_gy = env.substitute(f.0, &subs(vec![(f.1, gx)])); // 18

    assert_eq!(env.discover_equal(fx_sub_gy, gx, 6), Ok(Equal::yes()));
    assert_eq!(env.discover_equal(gx, fx_sub_gy, 6), Ok(Equal::yes()));
    assert_eq!(
        env.discover_equal(gx, fx_sub_gy, 1),
        Ok(Equal::NeedsHigherLimit)
    );
}

#[test]
fn fx_sub_nothing_is_gy_sub_nothing() {
    let mut env = env();
    let x = env.variable_full();
    let y = env.variable_full();
    let f = env.variable_full_with_deps(vec![x.0]);
    let f_sub = env.substitute(f.0, &Default::default());
    let g = env.variable_full_with_deps(vec![y.0]);
    let g_sub = env.substitute(g.0, &Default::default());
    assert_matches!(env.discover_equal(f_sub, g_sub, 3), Ok(Equal::Yes(..)));
    assert_matches!(env.discover_equal(g_sub, f_sub, 3), Ok(Equal::Yes(..)));
    assert_matches!(
        env.discover_equal(g_sub, f_sub, 0),
        Ok(Equal::NeedsHigherLimit)
    );
    if let Ok(Equal::Yes(lsubs)) = env.discover_equal(f_sub, g_sub, 3) {
        assert_eq!(lsubs.len(), 2);
        let mut entries = lsubs.iter();
        assert_eq!(entries.next(), Some(&(x.1, y.0)));
        let last = entries.next().unwrap();
        assert_eq!(last.0, f.1);
        if let Ok(Some(sub)) = env.get_and_downcast_construct_definition::<CSubstitution>(last.1) {
            assert_eq!(sub.base(), g.0);
            assert_eq!(sub.substitutions(), &subs(vec![(y.1, x.0)]))
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
    let x = env.variable_full();
    let y = env.variable_full();
    let z = env.variable_full();
    let f = env.variable_full_with_deps(vec![x.0]);
    let f_sub = env.substitute(f.0, &subs(vec![(x.1, z.0)]));
    let g = env.variable_full_with_deps(vec![y.0]);
    let g_sub = env.substitute(g.0, &Default::default());
    assert_matches!(env.discover_equal(f_sub, g_sub, 3), Ok(Equal::Yes(..)));
    assert_matches!(env.discover_equal(g_sub, f_sub, 3), Ok(Equal::Yes(..)));
    assert_matches!(
        env.discover_equal(g_sub, f_sub, 0),
        Ok(Equal::NeedsHigherLimit)
    );
    if let Ok(Equal::Yes(lsubs)) = env.discover_equal(f_sub, g_sub, 3) {
        assert_eq!(lsubs.len(), 2);
        let mut entries = lsubs.iter();
        assert_eq!(entries.next(), Some(&(z.1, y.0)));
        let last = entries.next().unwrap();
        assert_eq!(last.0, f.1);
        if let Ok(Some(sub)) = env.get_and_downcast_construct_definition::<CSubstitution>(last.1) {
            assert_eq!(sub.base(), g.0);
            assert_eq!(sub.substitutions(), &subs(vec![(y.1, x.0)]))
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
    let a = env.unique();
    let b = env.unique();
    let c = env.unique();
    let d = env.unique();

    let dec = env.decision(a, b, c, d);
    let x = env.variable_full();
    let f = env.variable_full_with_deps(vec![x.0]);
    let f_dec = env.substitute(f.0, &subs(vec![(x.1, dec)]));

    let dec = env.decision(a, b, c, d);
    let y = env.variable_full();
    let g = env.variable_full_with_deps(vec![y.0]);
    let g_dec = env.substitute(g.0, &subs(vec![(y.1, dec)]));

    assert_matches!(env.discover_equal(f_dec, g_dec, 3), Ok(Equal::Yes(..)));
    assert_matches!(env.discover_equal(g_dec, f_dec, 3), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(lsubs)) = env.discover_equal(f_dec, g_dec, 3) {
        assert_eq!(lsubs.len(), 1);
        let mut entries = lsubs.iter();
        let last = entries.next().unwrap();
        assert_eq!(last.0, f.1);
        if let Ok(Some(sub)) = env.get_and_downcast_construct_definition::<CSubstitution>(last.1) {
            assert_eq!(sub.base(), g.0);
            assert_eq!(sub.substitutions(), &subs(vec![(y.1, x.0)]))
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
    let a = env.unique();
    let b = env.unique();
    let c = env.unique();
    let d = env.unique();

    let s = env.variable_full();
    let t = env.variable_full();
    let u = env.variable_full();
    let v = env.variable_full();

    let dec_for_dex = env.decision(a, b, c, d);
    let x = env.variable_full();
    let dex = env.decision(x.0, d, c, b);
    let dex_dec = env.substitute(dex, &subs(vec![(x.1, dec_for_dex)]));

    let dec_for_g = env.decision(s.0, t.0, u.0, v.0);
    let y = env.variable_full();
    let g = env.variable_full_with_deps(vec![y.0]);
    let g_dec = env.substitute(g.0, &subs(vec![(y.1, dec_for_g)]));

    assert_matches!(env.discover_equal(g_dec, dex_dec, 3), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(lsubs)) = env.discover_equal(g_dec, dex_dec, 3) {
        assert_eq!(lsubs.len(), 5);
        let mut entries = lsubs.iter();
        let first = entries.next().unwrap();
        assert_eq!(first.0, g.1);
        if let Ok(Some(sub)) = env.get_and_downcast_construct_definition::<CSubstitution>(first.1) {
            assert_eq!(sub.base(), dex);
            assert_eq!(sub.substitutions(), &subs(vec![(x.1, y.0)]))
        } else {
            panic!("Expected second substitution to be itself another substitution");
        }
        assert_eq!(entries.next().unwrap(), &(s.1, a));
        assert_eq!(entries.next().unwrap(), &(t.1, b));
        assert_eq!(entries.next().unwrap(), &(u.1, c));
        assert_eq!(entries.next().unwrap(), &(v.1, d));
    } else {
        unreachable!()
    }
}

#[test]
fn fx_sub_decision_with_var_is_gy_sub_decision() {
    let mut env = env();

    let aa = env.variable_full();

    let a = env.unique();
    let b = env.unique();
    let c = env.unique();
    let d = env.unique();

    let dec = env.decision(aa.0, b, c, d);
    let x = env.variable_full();
    let f = env.variable_full_with_deps(vec![x.0]);
    let f_dec = env.substitute(f.0, &subs(vec![(x.1, dec)]));

    let dec = env.decision(a, b, c, d);
    let y = env.variable_full();
    let g = env.variable_full_with_deps(vec![y.0]);
    let g_dec = env.substitute(g.0, &subs(vec![(y.1, dec)]));

    assert_matches!(env.discover_equal(f_dec, g_dec, 4), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(lsubs)) = env.discover_equal(f_dec, g_dec, 4) {
        assert_eq!(lsubs.len(), 2);
        let mut entries = lsubs.iter();
        assert_eq!(Some(&(aa.1, a)), entries.next());
        let last = entries.next().unwrap();
        assert_eq!(last.0, f.1);
        if let Ok(Some(sub)) = env.get_and_downcast_construct_definition::<CSubstitution>(last.1) {
            assert_eq!(sub.base(), g.0);
            assert_eq!(sub.substitutions(), &subs(vec![(y.1, x.0)]))
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
    let a = env.unique();
    // 14
    let x = env.variable_full();
    // 15
    let y = env.variable_full();
    // 16
    let g = env.variable_full_with_deps(vec![y.0]);
    // 17
    let gx = env.substitute(g.0, &subs(vec![(y.1, x.0)]));

    // 18
    let f = env.variable_full_with_deps(vec![x.0]);
    // 19
    let f_sub_gx = env.substitute(f.0, &subs(vec![(f.1, gx)]));
    // 20
    let f_sub_gx_sub_a = env.substitute(f_sub_gx, &subs(vec![(x.1, a)]));

    // 21
    let gy_sub_a = env.substitute(g.0, &subs(vec![(y.1, a)]));

    assert_eq!(
        env.discover_equal(f_sub_gx_sub_a, gy_sub_a, 5),
        Ok(Equal::yes())
    );
    assert_eq!(
        env.discover_equal(gy_sub_a, f_sub_gx_sub_a, 5),
        Ok(Equal::yes())
    );
}

#[test]
fn fx_sub_a_sub_gy_is_gy_sub_a() {
    let mut env = env();

    let a = env.unique();

    let x = env.variable_full();
    let y = env.variable_full();
    let g = env.variable_full_with_deps(vec![y.0]);
    let gx = env.substitute(g.0, &subs(vec![(y.1, x.0)]));

    let f = env.variable_full_with_deps(vec![x.0]);
    let f_sub_a = env.substitute(f.0, &subs(vec![(x.1, a)]));
    let f_sub_a_sub_gy = env.substitute(f_sub_a, &subs(vec![(f.1, gx)]));

    let gy_sub_a = env.substitute(g.0, &subs(vec![(y.1, a)]));

    assert_eq!(
        env.discover_equal(f_sub_a_sub_gy, gy_sub_a, 4),
        Ok(Equal::yes())
    );
    assert_eq!(
        env.discover_equal(gy_sub_a, f_sub_a_sub_gy, 4),
        Ok(Equal::yes())
    );
}

#[test]
fn x_eq_y_sub_true_true_is_a_equal_a() {
    let mut env = env();
    let truee = env.unique();
    let falsee = env.unique();

    let a = env.variable_full();
    let x = env.variable_full();
    let y = env.variable_full();

    let x_eq_y = env.decision(x.0, y.0, truee, falsee);
    let true_eq_true = env.substitute(x_eq_y, &subs(vec![(x.1, truee), (y.1, truee)]));
    let a_eq_a = env.decision(a.0, a.0, truee, falsee);

    assert_matches!(
        env.discover_equal(a_eq_a, true_eq_true, 3),
        Ok(Equal::Yes(..))
    );
    if let Ok(Equal::Yes(lsubs)) = env.discover_equal(a_eq_a, true_eq_true, 3) {
        assert_eq!(lsubs.len(), 1);
        let mut entries = lsubs.iter();
        let last = entries.next().unwrap();
        assert_eq!(last, &(a.1, truee));
    } else {
        unreachable!()
    }
}
