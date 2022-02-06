use crate::{
    constructs::{
        decision::CDecision,
        unique::CUnique,
        variable::{CVariable, Variable, VariableId},
        ConstructId,
    },
    environment::Environment,
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

    fn assert_def_equal(&mut self, left: ConstructId, right: ConstructId) {
        assert_eq!(
            self.is_def_equal_without_subs(left, right, 1024),
            TripleBool::True
        );
    }

    fn assert_def_equal_not(&mut self, left: ConstructId, right: ConstructId) {
        assert_eq!(
            self.is_def_equal_without_subs(left, right, 1024),
            TripleBool::False
        );
    }

    fn assert_def_equal_unknown(&mut self, left: ConstructId, right: ConstructId) {
        assert_eq!(
            self.is_def_equal_without_subs(left, right, 1024),
            TripleBool::Unknown
        );
    }
}

#[test]
fn unique_is_self() {
    let mut env = env();
    let id = env.unique();
    env.assert_def_equal(id, id);
}

#[test]
fn unique_is_not_unique() {
    let mut env = env();
    let left = env.unique();
    let right = env.unique();
    env.assert_def_equal_not(left, right);
}

#[test]
fn var_is_self() {
    let mut env = env();
    let left = env.variable();
    let right = left;
    env.assert_def_equal(left, right);
}

#[test]
fn var_eq_var_is_self() {
    let mut env = env();
    let truee = env.unique();
    let falsee = env.unique();

    let left = env.variable();
    let right = env.variable();
    let decision = env.decision(left, right, truee, falsee);
    env.assert_def_equal(decision, decision);
}

#[test]
fn var_equals_self_is_true() {
    let mut env = env();
    let truee = env.unique();
    let falsee = env.unique();

    let left = env.variable();
    let right = left;
    let decision = env.decision(left, right, truee, falsee);
    env.assert_def_equal(decision, truee);
}

#[test]
fn unique_equals_unique_is_false() {
    let mut env = env();
    let truee = env.unique();
    let falsee = env.unique();

    let left = env.unique();
    let right = env.unique();
    let decision = env.decision(left, right, truee, falsee);
    env.assert_def_equal(decision, falsee);
}

#[test]
fn var_sub_not_equal_var() {
    let mut env = env();
    let truee = env.unique();
    let (var_id, var) = env.variable_full();
    let sub = env.substitute(var_id, &vec![(var, truee)].into_iter().collect());
    env.assert_def_equal_unknown(sub, var_id);
    env.assert_def_equal_unknown(var_id, sub);
}

#[test]
fn var_dep_x_sub_x_is_x() {
    let mut env = env();
    let x = env.variable();
    let (var_id, var) = env.variable_full_with_deps(vec![x]);
    let sub = env.substitute(var_id, &vec![(var, x)].into_iter().collect());
    env.assert_def_equal(sub, x);
}

#[test]
fn var_dep_x_sub_x_and_y_is_y() {
    let mut env = env();
    let y = env.unique();
    let (x_id, x_var) = env.variable_full();
    let (var_id, var) = env.variable_full_with_deps(vec![x_id]);
    let sub = env.substitute(var_id, &vec![(var, x_id), (x_var, y)].into_iter().collect());
    env.assert_def_equal(sub, y);
}
