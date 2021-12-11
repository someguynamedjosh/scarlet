use std::collections::HashMap;

use crate::{
    constructs::{base::BoxedConstruct, ConstructDefinition, ConstructId},
    environment::Environment,
    scope::{ScopeId, Scope},
    shared::OwnedOrBorrowed,
    tokens::structure::Token,
    transform::pattern::{Pattern, PatternMatchSuccess},
};

pub struct TransformerResult<'x>(pub Token<'x>);

pub struct ApplyContext<'a, 'x> {
    pub env: &'a mut Environment<'x>,
    pub scope: ScopeId,
}

impl<'a, 'x> ApplyContext<'a, 'x> {
    pub fn with_scope<'b>(&'b mut self, new_scope: ScopeId) -> ApplyContext<'b, 'x>
    where
        'a: 'b,
    {
        ApplyContext {
            env: self.env,
            scope: new_scope,
        }
    }

    pub fn push_placeholder(&mut self) -> ConstructId {
        let con = self.env.push_placeholder();
        self.env.constructs[con].scope = self.scope;
        con
    }

    pub fn push_construct(&mut self, construct: BoxedConstruct) -> ConstructId {
        let con = self.env.push_construct(construct);
        self.env.constructs[con].scope = self.scope;
        con
    }

    pub fn push_scope(&mut self, scope: Box<dyn Scope>) -> ScopeId {
        self.env.push_scope(scope, Some(self.scope))
    }

    pub fn push_unresolved(&mut self, token: Token<'x>) -> ConstructId {
        let con = self.env.push_unresolved(token.clone(), self.scope);
        let existing_scope = self.env.constructs[con].scope;
        if existing_scope != self.scope {
            let con = self.push_placeholder();
            self.env.constructs[con].definition = ConstructDefinition::Unresolved(token);
            self.env.constructs[con].scope = self.scope;
            con
        } else {
            con
        }
    }

    pub fn push_var(&mut self, invariants: ConstructId, capturing: bool) -> ConstructId {
        let con = self.env.push_variable(invariants, capturing);
        self.env.constructs[con].scope = self.scope;
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

    fn vomit<'x>(&self, c: &mut ApplyContext<'_, 'x>, to: &Token<'x>) -> Option<Token<'x>>;

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
