use super::{
    downcast_construct, substitution::Substitutions, Construct, ConstructDefinition, ConstructId,
};
use crate::{
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct,
    scope::SPlain,
    shared::TripleBool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CIfThenElse {
    condition: ConstructId,
    then: ConstructId,
    elsee: ConstructId,
}

impl CIfThenElse {
    pub fn new<'x>(condition: ConstructId, then: ConstructId, elsee: ConstructId) -> Self {
        Self {
            condition,
            then,
            elsee,
        }
    }

    pub fn condition(&self) -> ConstructId {
        self.condition
    }

    pub fn then(&self) -> ConstructId {
        self.then
    }

    pub fn elsee(&self) -> ConstructId {
        self.elsee
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
        this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<ConstructId> {
        let truee = env.get_language_item("true");
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
                let conditional_inv = CIfThenElse::new(self.condition, true_inv, truee);
                let conditional_inv = env.push_construct(conditional_inv, Box::new(SPlain(this)));
                env.reduce(conditional_inv);
                result.push(conditional_inv);
            }
        }
        for false_inv in false_invs {
            // Everything left over is conditional.
            let conditional_inv = CIfThenElse::new(self.condition, truee, false_inv);
            let conditional_inv = env.push_construct(conditional_inv, Box::new(SPlain(this)));
            env.reduce(conditional_inv);
            result.push(conditional_inv);
        }
        result
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        let mut deps = env.get_dependencies(self.condition);
        deps.append(env.get_dependencies(self.then));
        deps.append(env.get_dependencies(self.elsee));
        deps
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
        match env.is_def_equal(self.condition, env.get_language_item("true")) {
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
    ) -> Box<dyn Construct> {
        let condition = env.substitute(self.condition, substitutions);
        let then = env.substitute(self.then, substitutions);
        let elsee = env.substitute(self.elsee, substitutions);
        Self::new(condition, then, elsee).dyn_clone()
    }
}
