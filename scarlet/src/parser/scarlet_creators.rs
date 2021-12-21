use crate::{
    constructs::{unique::CUnique, ConstructId},
    environment::Environment,
    scope::Scope,
};

fn create_unique(env: &mut Environment, scope: Box<dyn Scope>) -> ConstructId {
    let id = env.push_unique();
    env.push_construct(CUnique::new(id), scope)
}