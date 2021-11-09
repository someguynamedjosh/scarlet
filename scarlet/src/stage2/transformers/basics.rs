use std::{collections::HashMap, ops::RangeInclusive};

use crate::stage2::structure::{Environment, ItemId, Token};

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
