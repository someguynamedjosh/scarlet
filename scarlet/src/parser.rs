use std::collections::HashSet;

// https://en.wikipedia.org/wiki/Earley_parser

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Component {
    Nonterminal(String),
    RepeatedTerminal(fn(char) -> bool),
    Terminal(fn(char) -> bool),
}

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

    pub fn advanced_past_next_nonterminal(&self) -> Self {
        let mut new = Self { ..*self };
        while let Some(RepeatedTerminal(..)) =
            self.rule.components.get(new.current_position_in_rule)
        {
            new.current_position_in_rule += 1;
        }
        if let Some(Nonterminal(..)) = self.rule.components.get(new.current_position_in_rule) {
            new.current_position_in_rule += 1
        } else {
            panic!(
                "advance_past_next_nonterminal called when the next thing is not a nonterminal!"
            );
        }
        new
    }

    pub fn is_complete(&self) -> bool {
        let mut position = self.current_position_in_rule;
        while let Some(RepeatedTerminal(..)) = self.rule.components.get(position) {
            position += 1;
        }
        position == self.rule.components.len()
    }

    pub fn immediate_next_nonterminal(&self) -> Option<&str> {
        let mut position = self.current_position_in_rule;
        let mut next = self.rule.components.get(position);
        while let Some(RepeatedTerminal(..)) = next {
            position += 1;
            next = self.rule.components.get(position);
        }
        if let Some(Nonterminal(nt)) = next {
            Some(&nt[..])
        } else {
            None
        }
    }

    pub fn match_against_character(&self, c: char) -> Option<Self> {
        let mut position = self.current_position_in_rule;
        let mut next = self.rule.components.get(position);
        while let Some(RepeatedTerminal(matcher)) = next {
            if matcher(c) {
                return Some(Self {
                    current_position_in_rule: position,
                    ..*self
                });
            }
            position += 1;
            next = self.rule.components.get(position);
        }
        if let Some(Terminal(t)) = next {
            Some(Self {
                current_position_in_rule: position,
                ..*self
            })
        } else {
            None
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

    pub fn accepts(&self, root_nonterminal: &str) -> bool {
        for state in &self.states {
            if state.rule.produced_nonterminal == root_nonterminal && state.is_complete() {
                return true
            }
        }
        false
    }

    pub fn advance(rules: &'a [Rule], previous: &[Self], this_char: char) -> Self {
        let immediate_predecessor = previous.last().unwrap();
        let mut next = Self {
            position: immediate_predecessor.position + 1,
            states: HashSet::new(),
        };
        loop {
            let old_size = next.states.len();
            next.predict(rules);
            next.scan(immediate_predecessor, this_char);
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

    fn scan(&mut self, previous: &Self, this_char: char) {
        for state in &previous.states {
            if let Some(advanced) = state.match_against_character(this_char) {
                self.states.insert(advanced);
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
                        new.insert(previous_state.advanced_past_next_nonterminal());
                    }
                }
            }
        }
        self.states.extend(new.into_iter());
    }
}

fn parse_internal(input: &str, rules: &[Rule], root_nonterminal: &str) {
    let mut state_sets = vec![StateSet::new(rules, root_nonterminal)];
    for this_char in input.chars() {
        let next_state = StateSet::advance(rules, &state_sets[..], this_char);
        state_sets.push(next_state);
    }
    println!("{:#?}", state_sets);
    println!("{:#?}", state_sets.last().unwrap().accepts(root_nonterminal));
}

macro_rules! components {
    ([] [$($items:tt);*]) => {
        vec![$($items),*]
    };
    ([$nonterminal:ident $($input:tt)*] [$($items:tt);*]) => {
        components!(
            [$($input)*]
            [$($items;)* (Component::Nonterminal(String::from(stringify!($nonterminal))))]
        )
    };
    ([$eval:tt* $($input:tt)*] [$($items:tt);*]) => {
        {
            fn matcher(arg: char) -> bool {
                $eval(arg)
            }
            components!(
                [$($input)*]
                [$($items;)* (Component::RepeatedTerminal(matcher))]
            )
        }
    };
    ([$eval:tt $($input:tt)*] [$($items:tt);*]) => {
        {
            fn matcher(arg: char) -> bool {
                $eval(arg)
            }
            components!(
                [$($input)*]
                [$($items;)* (Component::Terminal(matcher))]
            )
        }
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

fn char_allowed_in_identifier(c: char) -> bool {
    c.is_alphanumeric()
}

pub fn parse(input: &str) {
    let rules = vec![
        rule!(Identifier -> (char_allowed_in_identifier)*),
    ];
    println!("{:#?}", rules);
    parse_internal(input, &rules[..], "Identifier");
}
