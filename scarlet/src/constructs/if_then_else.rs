use super::{
    downcast_construct, substitution::Substitutions, variable::CVariable, Construct,
    ConstructDefinition, ConstructId,
};
use crate::{
    environment::Environment, impl_any_eq_for_construct, scope::SPlain, shared::TripleBool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CIfThenElse {
    condition: ConstructId,
    then: ConstructId,
    elsee: ConstructId,
}

impl CIfThenElse {
    pub fn new<'x>(
        env: &mut Environment<'x>,
        condition: ConstructId,
        then: ConstructId,
        elsee: ConstructId,
    ) -> ConstructId {
        let con = env.push_construct(Self {
            condition,
            then,
            elsee,
        });
        env.set_scope(condition, &SPlain(con));
        env.set_scope(then, &SPlain(con));
        env.set_scope(elsee, &SPlain(con));
        con
    }
}

impl_any_eq_for_construct!(CIfThenElse);

impl Construct for CIfThenElse {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn generated_invariants<'x>(
        &self,
        _this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<ConstructId> {
        let truee = env.get_builtin_item("true");
        let true_invs = env.generated_invariants(self.then);
        let mut false_invs = env.generated_invariants(self.then);
        let mut result = Vec::new();
        for true_inv in true_invs {
            let mut is_conditional = true;
            for (index, &false_inv) in false_invs.clone().iter().enumerate() {
                if env.is_def_equal(true_inv, false_inv) == TripleBool::True {
                    result.push(true_inv);
                    false_invs.remove(index);
                    is_conditional = false;
                }
            }
            if is_conditional {
                let conditional_inv = CIfThenElse::new(env, self.condition, true_inv, truee);
                env.reduce(conditional_inv);
                result.push(conditional_inv);
            }
        }
        for false_inv in false_invs {
            // Everything left over is conditional.
            let conditional_inv = CIfThenElse::new(env, self.condition, truee, false_inv);
            env.reduce(conditional_inv);
            result.push(conditional_inv);
        }
        result
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        [
            env.get_dependencies(self.condition),
            env.get_dependencies(self.then),
            env.get_dependencies(self.elsee),
        ]
        .concat()
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        if let Some(other) = downcast_construct::<Self>(other) {
            TripleBool::and(vec![
                env.is_def_equal(self.condition, other.condition),
                env.is_def_equal(self.then, other.then),
                env.is_def_equal(self.elsee, other.elsee),
            ])
        } else {
            TripleBool::Unknown
        }
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        env.reduce(self.condition);
        let condition = env.resolve(self.condition);
        match env.is_def_equal(condition, env.get_builtin_item("true")) {
            TripleBool::True => {
                env.reduce(self.then);
                self.then.into()
            }
            TripleBool::False => {
                env.reduce(self.elsee);
                self.elsee.into()
            }
            TripleBool::Unknown => self.dyn_clone().into(),
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let condition = env.substitute(self.condition, substitutions);
        let then = env.substitute(self.then, substitutions);
        let elsee = env.substitute(self.elsee, substitutions);
        Self::new(env, condition, then, elsee)
    }
}