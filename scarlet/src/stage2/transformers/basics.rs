use std::{collections::HashMap, ops::RangeInclusive};

use crate::stage2::structure::{Definition, Environment, ItemId, Token, VarType};

pub struct TransformerResult<'t> {
    pub replace_range: RangeInclusive<usize>,
    pub with: Token<'t>,
}

pub struct ApplyContext<'a, 't> {
    pub env: &'a mut Environment<'t>,
    pub parent_scope: Option<ItemId<'t>>,
    pub to: &'a mut Vec<Token<'t>>,
}

impl<'a, 't> ApplyContext<'a, 't> {
    pub fn with_target<'b, 'c>(
        &'c mut self,
        new_target: &'b mut Vec<Token<'t>>,
    ) -> ApplyContext<'b, 't>
    where
        'a: 'b,
        'c: 'b,
    {
        ApplyContext {
            env: self.env,
            parent_scope: self.parent_scope,
            to: new_target,
        }
    }

    pub fn with_parent_scope<'b>(
        &'b mut self,
        new_parent_scope: Option<ItemId<'t>>,
    ) -> ApplyContext<'b, 't>
    where
        'a: 'b,
    {
        ApplyContext {
            env: self.env,
            parent_scope: new_parent_scope,
            to: self.to,
        }
    }

    pub fn push_def(&mut self, def: Definition<'t>) -> ItemId<'t> {
        let item = self.env.push_def(def);
        self.env.items[item].parent_scope = self.parent_scope;
        item
    }

    pub fn push_token(&mut self, token: Token<'t>) -> ItemId<'t> {
        let item = self.env.push_token(token);
        let existing_scope = self.env.items[item].parent_scope;
        if existing_scope.is_some() && existing_scope != self.parent_scope {
            self.push_def(Definition::Unresolved(Token::Item(item)))
        } else {
            self.env.items[item].parent_scope = self.parent_scope;
            item
        }
    }

    pub fn push_var(&mut self, typee: VarType<'t>) -> ItemId<'t> {
        let item = self.env.push_var(typee);
        self.env.items[item].parent_scope = self.parent_scope;
        item
    }
}

pub trait Transformer {
    /// Returns true if the transformer should be applied at the given
    /// location.
    fn should_be_applied_at<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> bool;
    fn apply<'t>(&self, c: &mut ApplyContext<'_, 't>, at: usize) -> TransformerResult<'t>;
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
