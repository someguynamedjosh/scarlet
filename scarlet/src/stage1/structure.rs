use std::fmt::Debug;

use crate::stage2::structure::Token;

#[derive(Clone, Debug)]
pub struct Module<'a> {
    pub self_content: Token<'a>,
    pub children: Vec<(String, Module<'a>)>,
}
