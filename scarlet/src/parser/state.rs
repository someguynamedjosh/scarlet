use std::{
    collections::HashSet,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
};

use indexmap::IndexSet;
use regex::Regex;

use super::{
    rule::{Component::*, Rule},
    token::Token,
};
use crate::shared::indented;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComponentMatch {
    ByToken,
    ByState(usize),
}

#[derive(Clone, Debug, Eq)]
pub struct State<'a> {
    pub rule: &'a Rule,
    pub current_position_in_rule: usize,
    pub start_position_in_input: usize,
    pub matches: Vec<ComponentMatch>,
}

impl<'a> PartialEq for State<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.rule == other.rule
            && self.current_position_in_rule == other.current_position_in_rule
            && self.start_position_in_input == other.start_position_in_input
    }
}

impl<'a> Hash for State<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rule.hash(state);
        self.current_position_in_rule.hash(state);
        self.start_position_in_input.hash(state);
    }
}

impl<'a> State<'a> {
    pub fn new(rule: &'a Rule, start_position_in_input: usize) -> Self {
        Self {
            rule,
            current_position_in_rule: 0,
            start_position_in_input,
            matches: vec![],
        }
    }

    pub fn advanced(&self, by: ComponentMatch) -> Self {
        Self {
            current_position_in_rule: self.current_position_in_rule + 1,
            matches: [self.matches.clone(), vec![by]].concat(),
            ..*self
        }
    }

    pub fn is_complete(&self) -> bool {
        self.current_position_in_rule == self.rule.components.len()
    }

    pub fn immediate_next_nonterminal(&self) -> Option<&str> {
        if let Some(Nonterminal(nt)) = self.rule.components.get(self.current_position_in_rule) {
            Some(&nt[..])
        } else {
            None
        }
    }

    pub fn immediate_next_terminal_matches(&self, token: &Token) -> bool {
        if let Some(Terminal(_name, matcher)) =
            self.rule.components.get(self.current_position_in_rule)
        {
            matcher(token)
        } else {
            false
        }
    }
}
