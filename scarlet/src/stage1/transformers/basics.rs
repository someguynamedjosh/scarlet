use std::{collections::HashMap, ops::RangeInclusive};

use crate::stage1::structure::TokenTree;

pub struct TransformerResult<'t> {
    pub replace_range: RangeInclusive<usize>,
    pub with: TokenTree<'t>,
}

pub trait Transformer {
    /// Returns true if the transformer should be applied at the given
    /// location.
    fn should_be_applied_at(&self, to: &[TokenTree], at: usize) -> bool;
    fn apply<'t>(&self, to: &Vec<TokenTree<'t>>, at: usize) -> TransformerResult<'t>;
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
