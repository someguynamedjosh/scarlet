use crate::{
    constructs::{decision::CDecision, unique::CUnique, variable::CVariable, ConstructId},
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
        let id = self.push_variable();
        self.push_construct(CVariable::new(id, vec![], vec![]), Box::new(SRoot))
    }

    fn assert_def_equal(&mut self, left: ConstructId, right: ConstructId) {
        assert_eq!(
            self.is_def_equal_without_subs(left, right),
            TripleBool::True
        );
    }

    fn assert_def_equal_not(&mut self, left: ConstructId, right: ConstructId) {
        assert_eq!(
            self.is_def_equal_without_subs(left, right),
            TripleBool::False
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
