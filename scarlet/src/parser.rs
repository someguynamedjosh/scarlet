use std::collections::HashSet;

// https://en.wikipedia.org/wiki/Earley_parser

pub struct Token<'a> {
    role: &'static str,
    content: &'a str,
}

pub fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let name = Regex::new("[a-zA-Z0-9]+|[^a-zA-Z0-9]").unwrap();
    let whitespace = Regex::new(r"[\r\n\t ]+").unwrap();
    let mut index = 0;
    let mut tokens = Vec::new();
    while index < input.len() {
        let (result, role) = if let Some(result) = whitespace.find_at(input, index) {
            (result, "whitespace")
        } else if let Some(result) = name.find_at(input, index) {
            (result, "name")
        } else {
            panic!("Unrecognized characters in input: {}", &input[index..])
        };
        tokens.push(Token {
            role,
            content: result.as_str(),
        });
        index = result.end();
    }
    tokens
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Component {
    Nonterminal(String),
    Terminal {
        role: &'static str,
        content: Option<&'static str>,
    },
}

use regex::Regex;
use Component::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rule {
    pub produced_nonterminal: String,
    pub components: Vec<Component>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct State<'a> {
    pub rule: &'a Rule,
    pub current_position_in_rule: usize,
    pub start_position_in_input: usize,
}

impl<'a> State<'a> {
    pub fn new(rule: &'a Rule, start_position_in_input: usize) -> Self {
        Self {
            rule,
            current_position_in_rule: 0,
            start_position_in_input,
        }
    }

    pub fn advanced(&self) -> Self {
        Self {
            current_position_in_rule: self.current_position_in_rule + 1,
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
        if let Some(Terminal { role, content }) =
            self.rule.components.get(self.current_position_in_rule)
        {
            (if let Some(content) = content {
                *content == token.content
            } else {
                true
            }) && (*role == token.role)
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateSet<'a> {
    pub position: usize,
    pub states: HashSet<State<'a>>,
}

impl<'a> StateSet<'a> {
    pub fn new(rules: &'a [Rule], root_nonterminal: &str) -> Self {
        let mut res = Self {
            position: 0,
            states: HashSet::new(),
        };
        for rule in rules {
            if rule.produced_nonterminal == root_nonterminal {
                res.states.insert(State::new(rule, 0));
            }
        }
        res
    }

    pub fn advance(rules: &'a [Rule], previous: &[Self], token: &Token<'a>) -> Self {
        let immediate_predecessor = previous.last().unwrap();
        let mut next = Self {
            position: immediate_predecessor.position + 1,
            states: HashSet::new(),
        };
        loop {
            let old_size = next.states.len();
            next.predict(rules);
            next.scan(immediate_predecessor, token);
            next.complete(previous);
            let new_size = next.states.len();
            if old_size == new_size {
                break;
            }
        }
        next
    }

    fn predict(&mut self, rules: &'a [Rule]) {
        let mut new = HashSet::new();
        for existing in &self.states {
            if let Some(nt) = existing.immediate_next_nonterminal() {
                for rule in rules {
                    if rule.produced_nonterminal == nt {
                        new.insert(State::new(rule, self.position));
                    }
                }
            }
        }
        self.states.extend(new.into_iter());
    }

    fn scan(&mut self, previous: &Self, token: &Token<'a>) {
        for state in &previous.states {
            if state.immediate_next_terminal_matches(token) {
                self.states.insert(state.advanced());
            }
        }
    }

    fn complete(&mut self, previous: &[Self]) {
        let mut new = HashSet::new();
        for state in &self.states {
            if state.is_complete() {
                for previous_state in &previous[state.start_position_in_input].states {
                    if previous_state.immediate_next_nonterminal()
                        == Some(&state.rule.produced_nonterminal)
                    {
                        new.insert(previous_state.advanced());
                    }
                }
            }
        }
        self.states.extend(new.into_iter());
    }
}

fn parse_internal(input: &str, rules: &[Rule], root_nonterminal: &str) {
    let mut state_sets = vec![StateSet::new(rules, root_nonterminal)];
    let tokens = tokenize(input);
    for token in &tokens {
        let next_state = StateSet::advance(rules, &state_sets[..], token);
        state_sets.push(next_state);
    }
    println!("{:#?}", state_sets);
}

macro_rules! components {
    ([] [$($items:tt);*]) => {
        vec![$($items),*]
    };
    ([name $($input:tt)*] [$($items:tt);*]) => {
        {
            components!(
                [$($input)*]
                [$($items;)* (Component::Terminal { role: "name", content: None })]
            )
        }
    };
    ([$nonterminal:ident $($input:tt)*] [$($items:tt);*]) => {
        components!(
            [$($input)*]
            [$($items;)* (Component::Nonterminal(String::from(stringify!($nonterminal))))]
        )
    };
}

macro_rules! rule {
    ($produced_nonterminal:ident -> $($components:tt)*) => {
        Rule {
            produced_nonterminal: String::from(stringify!($produced_nonterminal)),
            components: components!([$($components)*] []),
        }
    };
}

pub fn parse(input: &str) {
    let rules = vec![rule!(Expr0 -> name)];
    println!("{:#?}", rules);
    parse_internal(input, &rules[..], "Expr0");
}
