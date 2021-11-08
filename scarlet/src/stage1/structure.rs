use std::fmt::{self, Debug, Formatter};

use crate::{stage2::structure::Token, util::indented};

#[derive(Clone, Debug)]
pub struct Module<'a> {
    pub self_content: Token<'a>,
    pub children: Vec<(String, Module<'a>)>,
}
