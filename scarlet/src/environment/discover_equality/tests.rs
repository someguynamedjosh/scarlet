use crate::{
    constructs::{
        decision::CDecision,
        substitution::Substitutions,
        unique::CUnique,
        variable::{CVariable, Variable, VariableId},
        ConstructId,
    },
    environment::{def_equal::IsDefEqual, discover_equality::Equal, Environment},
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
            invariants: vec![],
            dependencies: vec![],
        });
        self.push_construct(CVariable::new(id), Box::new(SRoot))
    }

    fn variable_full(&mut self) -> (ConstructId, VariableId) {
        let id = self.push_variable(Variable {
            id: None,
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
    assert_eq!(env.discover_equal(f.0, g.0, 2), Ok(Equal::yes()));
}
