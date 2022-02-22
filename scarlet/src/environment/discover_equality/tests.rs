#![cfg(test)]

use std::assert_matches::assert_matches;

use crate::{
    constructs::{
        decision::CDecision,
        substitution::{CSubstitution, Substitutions},
        unique::CUnique,
        variable::{CVariable, Variable, VariableId},
        ConstructId,
    },
    environment::{discover_equality::Equal, Environment},
    scope::SRoot,
    shared::TripleBool,
};

fn env<'a>() -> Environment<'a> {
    Environment::new()
}

impl<'a> Environment<'a> {
    fn decision(
        &mut self,
        left: ConstructId,
        right: ConstructId,
        equal: ConstructId,
        unequal: ConstructId,
    ) -> ConstructId {
        self.push_construct(CDecision::new(left, right, equal, unequal), Box::new(SRoot))
    }

    fn unique(&mut self) -> ConstructId {
        let id = self.push_unique();
        self.push_construct(CUnique::new(id), Box::new(SRoot))
    }

    fn variable(&mut self) -> ConstructId {
        let id = self.push_variable(Variable {
            id: None,
            construct: None,
            invariants: vec![],
            dependencies: vec![],
        });
        self.push_construct(CVariable::new(id), Box::new(SRoot))
    }

    fn variable_full(&mut self) -> (ConstructId, VariableId) {
        let id = self.push_variable(Variable {
            id: None,
            construct: None,
            invariants: vec![],
            dependencies: vec![],
        });
        let con = CVariable::new(id);
        let cid = self.push_construct(con.clone(), Box::new(SRoot));
        (cid, id)
    }

    fn variable_full_with_deps(&mut self, deps: Vec<ConstructId>) -> (ConstructId, VariableId) {
        let id = self.push_variable(Variable {
            id: None,
            construct: None,
            invariants: vec![],
            dependencies: deps,
        });
        let con = CVariable::new(id);
        let cid = self.push_construct(con.clone(), Box::new(SRoot));
        (cid, id)
    }
}

fn subs(from: Vec<(VariableId, ConstructId)>) -> Substitutions {
    from.into_iter().collect()
}

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
    let left = Equal::Yes(expected.clone(), Default::default());
    let right = Equal::Yes(Default::default(), expected.clone());
    assert_eq!(env.discover_equal(var_con, thing, 1), Ok(left));
    assert_eq!(env.discover_equal(thing, var_con, 1), Ok(right));
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
fn decision_equals_similar_decision() {
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
        Ok(Equal::Yes(subs.clone(), Default::default()))
    );
    assert_eq!(
        env.discover_equal(dec2, dec1, 2),
        Ok(Equal::Yes(Default::default(), subs.clone()))
    );
}

#[test]
fn fx_is_gy() {
    let mut env = env();
    let x = env.variable_full();
    let y = env.variable_full();
    let f = env.variable_full_with_deps(vec![x.0]);
    let g = env.variable_full_with_deps(vec![y.0]);
    assert_matches!(env.discover_equal(f.0, g.0, 1), Ok(Equal::Yes(..)));
    assert_matches!(env.discover_equal(g.0, f.0, 1), Ok(Equal::Yes(..)));
    assert_matches!(env.discover_equal(g.0, f.0, 0), Ok(Equal::NeedsHigherLimit));
    if let Ok(Equal::Yes(lsubs, rsubs)) = env.discover_equal(f.0, g.0, 1) {
        assert_eq!(lsubs.len(), 2);
        assert_eq!(rsubs.len(), 0);
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
fn fx_sub_gy_is_gy_sub_x() {
    let mut env = env();
    let x = env.variable_full();
    let y = env.variable_full();
    let f = env.variable_full_with_deps(vec![x.0]);
    let g = env.variable_full_with_deps(vec![y.0]);

    let gx = env.substitute(g.0, &subs(vec![(y.1, x.0)]));
    let fx_sub_gy = env.substitute(f.0, &subs(vec![(f.1, gx)]));

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
    if let Ok(Equal::Yes(lsubs, rsubs)) = env.discover_equal(f_sub, g_sub, 3) {
        assert_eq!(lsubs.len(), 2);
        assert_eq!(rsubs.len(), 0);
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
    if let Ok(Equal::Yes(lsubs, rsubs)) = env.discover_equal(f_sub, g_sub, 3) {
        assert_eq!(lsubs.len(), 2);
        assert_eq!(rsubs.len(), 0);
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
    if let Ok(Equal::Yes(lsubs, rsubs)) = env.discover_equal(f_dec, g_dec, 3) {
        assert_eq!(lsubs.len(), 1);
        assert_eq!(rsubs.len(), 0);
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
    assert_matches!(env.discover_equal(g_dec, f_dec, 4), Ok(Equal::Yes(..)));
    if let Ok(Equal::Yes(lsubs, rsubs)) = env.discover_equal(f_dec, g_dec, 4) {
        assert_eq!(lsubs.len(), 2);
        assert_eq!(rsubs.len(), 0);
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
