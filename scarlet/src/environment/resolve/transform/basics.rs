use std::collections::HashMap;

use super::pattern::{Pattern, PatternMatchSuccess};
use crate::{
    constructs::base::BoxedConstruct,
    environment::{ConstructDefinition, ConstructId, Environment},
    shared::OwnedOrBorrowed,
    tokens::structure::Token,
};

pub struct TransformerResult<'x>(pub Token<'x>);

pub struct ApplyContext<'a, 'x> {
    pub env: &'a mut Environment<'x>,
    pub parent_scope: Option<ConstructId<'x>>,
}

impl<'a, 'x> ApplyContext<'a, 'x> {
    pub fn with_parent_scope<'b>(
        &'b mut self,
        new_parent_scope: Option<ConstructId<'x>>,
    ) -> ApplyContext<'b, 'x>
    where
        'a: 'b,
    {
        ApplyContext {
            env: self.env,
            parent_scope: new_parent_scope,
        }
    }

    pub fn push_placeholder(&mut self) -> ConstructId<'x> {
        let con = self.env.push_placeholder();
        self.env.constructs[con].parent_scope = self.parent_scope;
        con
    }

    pub fn push_construct(&mut self, construct: BoxedConstruct<'x>) -> ConstructId<'x> {
        let con = self.env.push_construct(construct);
        self.env.constructs[con].parent_scope = self.parent_scope;
        con
    }

    pub fn push_unresolved(&mut self, token: Token<'x>) -> ConstructId<'x> {
        let con = self.env.push_unresolved(token.clone());
        let existing_scope = self.env.constructs[con].parent_scope;
        if existing_scope.is_some() && existing_scope != self.parent_scope {
            let con = self.push_placeholder();
            self.env.constructs[con].definition = ConstructDefinition::Unresolved(token);
            con
        } else {
            self.env.constructs[con].parent_scope = self.parent_scope;
            con
        }
    }
}

pub trait Transformer {
    fn pattern(&self) -> Box<dyn Pattern>;
    fn apply<'x>(
        &self,
        c: &mut ApplyContext<'_, 'x>,
        success: PatternMatchSuccess<'_, 'x>,
    ) -> TransformerResult<'x>;
}

pub type Precedence = u8;

pub type Extras<'e> = HashMap<Precedence, Vec<Box<dyn Transformer + 'e>>>;
pub type SomeTransformer<'e> = OwnedOrBorrowed<'e, dyn Transformer + 'e>;
