mod resolve;

use crate::{
    constructs::base::BoxedConstruct,
    shared::{Id, Pool},
    tokens::structure::Token,
};

#[derive(Debug)]
pub enum ConstructDefinition<'x> {
    Placeholder,
    Resolved(BoxedConstruct<'x>),
    Unresolved(Token<'x>),
}

#[derive(Debug)]
pub struct AnnotatedConstruct<'x> {
    pub definition: ConstructDefinition<'x>,
    pub parent_scope: Option<ConstructId<'x>>,
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

    pub fn push_placeholder(&mut self) -> ConstructId<'x> {
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Placeholder,
            parent_scope: None,
        };
        self.constructs.push(con)
    }

    pub fn push_construct(&mut self, construct: BoxedConstruct<'x>) -> ConstructId<'x> {
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Resolved(construct),
            parent_scope: None,
        };
        self.constructs.push(con)
    }

    pub fn push_unresolved(&mut self, token: Token<'x>) -> ConstructId<'x> {
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Unresolved(token),
            parent_scope: None,
        };
        self.constructs.push(con)
    }
}
