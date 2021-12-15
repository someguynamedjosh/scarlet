use std::{collections::HashMap, fmt::Debug};

use crate::{
    constructs::{Construct, ConstructDefinition, ConstructId},
    environment::Environment,
    scope::{SPlain, Scope},
    shared::OwnedOrBorrowed,
    tokens::structure::Token,
    transform::pattern::{Pattern, PatternMatchSuccess},
};

pub struct ApplyContext<'a, 'x> {
    pub env: &'a mut Environment<'x>,
}

pub trait Parser<'x>: Fn(&'x str) -> Result<(&'x str, BoxedResolvable<'x>), String> {}

impl<'x, T> Parser<'x> for T where T: Fn(&'x str) -> Result<(&'x str, BoxedResolvable<'x>), String> {}

pub type BoxedResolvable<'x> = Box<dyn Resolvable<'x> + 'x>;

pub trait Resolvable<'x>: Debug {
    fn resolve(&self, env: &mut Environment<'x>, scope: &dyn Scope) -> ConstructDefinition;
}

pub type Precedence = u8;

pub type SomeParser<'x> = OwnedOrBorrowed<'x, dyn Parser<'x>>;

pub type Extras<'x> = HashMap<Precedence, Vec<SomeParser<'x>>>;

pub fn p_identifier<'x>(input: &'x str) -> Result<(&'x str, BoxedResolvable<'x>), String> {
    let mut split_at = 0;
    for char in input.chars() {
        if char.is_alphanumeric() {
            split_at += char.len_utf8()
        } else {
            break;
        }
    }
    if split_at > 0 {
        let (identifier, remainder) = input.split_at(split_at);
        Ok((remainder, Box::new(RIdentifier(identifier))))
    } else {
        Err(format!("invalid identifier"))
    }
}

#[derive(Debug)]
pub struct RIdentifier<'x>(&'x str);

impl<'x> Resolvable<'x> for RIdentifier<'x> {
    fn resolve(&self, env: &mut Environment<'x>, scope: &dyn Scope) -> ConstructDefinition {
        scope.lookup_ident(env, self.0).unwrap().into()
    }
}
