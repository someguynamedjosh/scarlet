use std::collections::HashMap;

use super::pattern::{Pattern, PatternMatchSuccess};
use crate::{
    constructs::{base::BoxedConstruct, variable::VarType},
    environment::{ConstructDefinition, ConstructId, Environment},
    shared::OwnedOrBorrowed,
    tokens::structure::Token,
};

pub struct TransformerResult<'x>(pub Token<'x>);

pub struct ApplyContext<'a, 'x> {
    pub env: &'a mut Environment<'x>,
    pub parent_scope: Option<ConstructId>,
}

impl<'a, 'x> ApplyContext<'a, 'x> {
    pub fn with_parent_scope<'b>(
        &'b mut self,
        new_parent_scope: Option<ConstructId>,
    ) -> ApplyContext<'b, 'x>
    where
        'a: 'b,
    {
        ApplyContext {
            env: self.env,
            parent_scope: new_parent_scope,
        }
    }

    pub fn push_placeholder(&mut self) -> ConstructId {
        let con = self.env.push_placeholder();
        self.env.constructs[con].parent_scope = self.parent_scope;
        con
    }

    pub fn push_construct(&mut self, construct: BoxedConstruct) -> ConstructId {
        let con = self.env.push_construct(construct);
        self.env.constructs[con].parent_scope = self.parent_scope;
        con
    }

    pub fn push_unresolved(&mut self, token: Token<'x>) -> ConstructId {
        let con = self.env.push_unresolved(token.clone(), None);
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

    pub fn push_var(&mut self, typee: VarType, capturing: bool) -> ConstructId {
        let con = self.env.push_variable(typee, capturing);
        self.env.constructs[con].parent_scope = self.parent_scope;
        con
    }
}

pub trait Transformer {
    fn input_pattern(&self) -> Box<dyn Pattern>;
    // fn output_pattern(&self) -> Box<dyn Pattern>;
    fn apply<'x>(
        &self,
        c: &mut ApplyContext<'_, 'x>,
        success: PatternMatchSuccess<'_, 'x>,
    ) -> TransformerResult<'x>;

    fn apply_checked<'x>(
        &self,
        c: &mut ApplyContext<'_, 'x>,
        success: PatternMatchSuccess<'_, 'x>,
    ) -> TransformerResult<'x> {
        let result = self.apply(c, success);
        // assert!(
        //     self.output_pattern()
        //         .match_at(c.env, &[result.0.clone()], 0)
        //         .is_ok(),
        //     "Output should match {:?}, but it is {:?} instead.",
        //     self.output_pattern(),
        //     result.0
        // );
        result
    }
}

pub type Precedence = u8;

pub type Extras<'e> = HashMap<Precedence, Vec<Box<dyn Transformer + 'e>>>;
pub type SomeTransformer<'e> = OwnedOrBorrowed<'e, dyn Transformer + 'e>;
