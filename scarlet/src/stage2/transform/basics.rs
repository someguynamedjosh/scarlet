use std::collections::HashMap;

use super::pattern::{Pattern, PatternMatchSuccess};
use crate::stage2::structure::{Definition, Environment, ConstructId, Token, VarType};

pub struct TransformerResult<'t>(pub Token<'t>);

pub struct ApplyContext<'a, 't> {
    pub env: &'a mut Environment<'t>,
    pub parent_scope: Option<ConstructId<'t>>,
}

impl<'a, 't> ApplyContext<'a, 't> {
    pub fn with_parent_scope<'b>(
        &'b mut self,
        new_parent_scope: Option<ConstructId<'t>>,
    ) -> ApplyContext<'b, 't>
    where
        'a: 'b,
    {
        ApplyContext {
            env: self.env,
            parent_scope: new_parent_scope,
        }
    }

    pub fn begin_item(&mut self) -> ConstructId<'t> {
        let item = self.env.begin_item();
        self.env.items[item].parent_scope = self.parent_scope;
        item
    }

    pub fn push_def(&mut self, def: Definition<'t>) -> ConstructId<'t> {
        let item = self.env.push_def(def);
        self.env.items[item].parent_scope = self.parent_scope;
        item
    }

    pub fn push_token(&mut self, token: Token<'t>) -> ConstructId<'t> {
        let item = self.env.push_token(token);
        let existing_scope = self.env.items[item].parent_scope;
        if existing_scope.is_some() && existing_scope != self.parent_scope {
            self.push_def(Definition::Unresolved(Token::Item(item)))
        } else {
            self.env.items[item].parent_scope = self.parent_scope;
            item
        }
    }

    pub fn push_var(&mut self, typee: VarType<'t>) -> ConstructId<'t> {
        let item = self.env.push_var(typee);
        self.env.items[item].parent_scope = self.parent_scope;
        item
    }
}

pub trait Transformer {
    fn pattern(&self) -> Box<dyn Pattern>;
    fn apply<'t>(
        &self,
        c: &mut ApplyContext<'_, 't>,
        success: PatternMatchSuccess<'_, 't>,
    ) -> TransformerResult<'t>;
}

pub enum OwnedOrBorrowed<'a, T: ?Sized> {
    Owned(Box<T>),
    Borrowed(&'a T),
}

impl<'a, T: ?Sized> AsRef<T> for OwnedOrBorrowed<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            OwnedOrBorrowed::Owned(data) => &*data,
            OwnedOrBorrowed::Borrowed(data) => *data,
        }
    }
}

pub type Precedence = u8;

pub type Extras<'e> = HashMap<Precedence, Vec<Box<dyn Transformer + 'e>>>;
pub type SomeTransformer<'e> = OwnedOrBorrowed<'e, dyn Transformer + 'e>;
