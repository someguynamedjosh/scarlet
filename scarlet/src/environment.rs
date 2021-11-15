mod resolve;

use crate::{
    shared::{Id, Pool},
    tokens::structure::Token,
};

pub type BoxedConstruct = ();

#[derive(Debug)]
pub enum ConstructDefinition<'x> {
    Resolved(BoxedConstruct),
    Unresolved(Token<'x>),
}

#[derive(Debug)]
pub struct AnnotatedConstruct<'x> {
    pub definition: ConstructDefinition<'x>,
}

pub type ConstructId<'x> = Id<AnnotatedConstruct<'x>, 'C'>;

pub struct Environment<'x> {
    constructs: Pool<AnnotatedConstruct<'x>, 'C'>,
}

impl<'x> Environment<'x> {
    pub fn new() -> Self {
        Self {
            constructs: Pool::new(),
        }
    }
}
