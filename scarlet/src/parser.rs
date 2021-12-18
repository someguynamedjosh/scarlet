use std::{
    collections::HashSet,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
};

use indexmap::IndexSet;
use regex::Regex;
use Component::*;

use crate::shared::indented;

// https://en.wikipedia.org/wiki/Earley_parser

pub struct Token<'a> {
    role: &'static str,
    content: &'a str,
}

impl<'a> Debug for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.role, self.content)
    }
}

pub fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let name = Regex::new("[a-zA-Z0-9_]+|[^a-zA-Z0-9_]").unwrap();
    let whitespace = Regex::new(r"[\r\n\t ]+").unwrap();
    let mut index = 0;
    let mut tokens = Vec::new();
    while index < input.len() {
        let (result, role) = (|| {
            if let Some(result) = whitespace.find_at(input, index) {
                if result.start() == index {
                    return (result, "whitespace");
                }
            }
            if let Some(result) = name.find_at(input, index) {
                if result.start() == index {
                    return (result, "name");
                }
            }
            panic!("Unrecognized characters in input: {}", &input[index..])
        })();
        tokens.push(Token {
            role,
            content: result.as_str(),
        });
        index = result.end();
    }
    tokens
}

#[derive(Clone)]
pub enum Component {
    Nonterminal(String),
    Terminal(&'static str, fn(&Token) -> bool),
}

impl Debug for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nonterminal(nt) => write!(f, "{}", nt),
            Self::Terminal(name, ..) => write!(f, "{}", name),
        }
    }
}

impl PartialEq for Component {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Nonterminal(nt) => {
                if let Nonterminal(ont) = other {
                    nt == ont
                } else {
                    false
                }
            }
            Terminal(n, t) => {
                if let Terminal(on, ot) = other {
                    n == on && (*t as *const ()) == (*ot as *const ())
                } else {
                    false
                }
            }
        }
    }
}

impl Eq for Component {}

impl Hash for Component {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Nonterminal(nt) => {
                state.write_u8(0);
                nt.hash(state);
            }
            Terminal(n, t) => {
                state.write_u8(1);
                n.hash(state);
                state.write_usize((*t as *const ()) as usize);
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    pub produced_nonterminal: String,
    pub components: Vec<Component>,
    pub preferred: bool,
}

impl Debug for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.preferred {
            write!(f, "(low priority) ")?;
        }
        write!(f, "{} ->", self.produced_nonterminal)?;
        for component in &self.components {
            write!(f, " ")?;
            component.fmt(f)?;
        }
        Ok(())
    }
}

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateSet<'a> {
    pub position: usize,
    pub states: IndexSet<State<'a>>,
}

impl<'a> StateSet<'a> {
    pub fn new(rules: &'a [Rule], root_nonterminal: &str) -> Self {
        let mut res = Self {
            position: 0,
            states: IndexSet::new(),
        };
        for rule in rules {
            if rule.produced_nonterminal == root_nonterminal {
                res.states.insert(State::new(rule, 0));
            }
        }
        loop {
            let old_size = res.states.len();
            res.predict(rules);
            res.complete(&[]);
            let new_size = res.states.len();
            if old_size == new_size {
                break;
            }
        }
        res
    }

    pub fn advance(rules: &'a [Rule], previous: &[Self], token: &Token<'a>) -> Self {
        let immediate_predecessor = previous.last().unwrap();
        let mut next = Self {
            position: immediate_predecessor.position + 1,
            states: IndexSet::new(),
        };
        next.execute_steps_until_no_new_states_appear(rules, previous, token);
        next
    }

    fn execute_steps_until_no_new_states_appear(
        &mut self,
        rules: &'a [Rule],
        previous: &[Self],
        token: &Token<'a>,
    ) {
        let immediate_predecessor = previous.last().unwrap();
        loop {
            let old_size = self.states.len();
            self.predict(rules);
            self.scan(immediate_predecessor, token);
            self.complete(previous);
            let new_size = self.states.len();
            if old_size == new_size {
                break;
            }
        }
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
                self.states.insert(state.advanced(ComponentMatch::ByToken));
            }
        }
    }

    fn get_state_completing_nonterminal(&self, nonterminal: &str) -> Option<usize> {
        let mut backup = None;
        for (completed_state_index, state) in self.states.iter().enumerate() {
            if !state.is_complete() {
                continue;
            }
            if state.rule.produced_nonterminal == nonterminal {
                if state.rule.preferred {
                    return Some(completed_state_index);
                } else {
                    backup = Some(completed_state_index);
                }
            }
        }
        backup
    }

    fn complete_state(
        state: &State,
        index: usize,
        existing_states: &IndexSet<State<'a>>,
        previous: &[Self],
        new: &mut HashSet<State<'a>>,
    ) {
        let idx = state.start_position_in_input;
        let previous_states = if idx < previous.len() {
            &previous[idx].states
        } else {
            existing_states
        };
        for previous_state in previous_states {
            if previous_state.immediate_next_nonterminal() == Some(&state.rule.produced_nonterminal)
            {
                let mat = ComponentMatch::ByState(index);
                new.insert(previous_state.advanced(mat));
            }
        }
    }

    fn complete(&mut self, previous: &[Self]) {
        let mut completed_nonterminals = HashSet::new();
        for state in self.states.iter() {
            if state.is_complete() {
                completed_nonterminals.insert(&state.rule.produced_nonterminal);
            }
        }
        let mut new = HashSet::new();
        for completed_nonterminal in completed_nonterminals {
            let completed_state_index = self
                .get_state_completing_nonterminal(completed_nonterminal)
                .unwrap();
            let state = &self.states[completed_state_index];
            Self::complete_state(
                state,
                completed_state_index,
                &self.states,
                previous,
                &mut new,
            )
        }
        for (index, state) in self.states.iter().enumerate() {
            if !state.is_complete() {
                continue;
            }
            Self::complete_state(state, index, &self.states, previous, &mut new)
        }
        self.states.extend(new.into_iter());
    }
}

#[derive(Clone)]
enum AstNode<'a> {
    Rule {
        rule: &'a Rule,
        components: Vec<AstNode<'a>>,
    },
    Terminal(&'a Token<'a>),
}

impl<'a> Debug for AstNode<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AstNode::Rule { rule, components } => {
                writeln!(f, "{:?} [", rule)?;
                for component in components {
                    writeln!(f, "    {},", indented(&format!("{:?}", component)))?;
                }
                write!(f, "]")
            }
            AstNode::Terminal(token) => token.fmt(f),
        }
    }
}

fn build_ast<'a>(
    sets: &'a [StateSet<'a>],
    tokens: &'a [Token<'a>],
    state: usize,
    end_index: usize,
) -> Result<(AstNode<'a>, usize), String> {
    let state_set_here = &sets[end_index];
    let state = state_set_here.states.get_index(state).unwrap();
    assert!(state.is_complete());
    let rule = &state.rule;
    let mut components = Vec::new();
    let mut input_index = end_index;
    for component in state.matches.iter().rev() {
        match component {
            &ComponentMatch::ByState(state) => {
                let (node, new_input_index) = build_ast(sets, tokens, state, input_index)?;
                components.push(node);
                input_index = new_input_index;
            }
            &ComponentMatch::ByToken => {
                input_index -= 1;
                components.push(AstNode::Terminal(&tokens[input_index]));
            }
        }
    }
    components.reverse();
    let node = AstNode::Rule { rule, components };
    Ok((node, input_index))
}

fn parse_internal(input: &str, rules: &[Rule], root_nonterminal: &str) {
    let mut state_sets = vec![StateSet::new(rules, root_nonterminal)];
    let tokens = tokenize(input);
    println!("{:?}", tokens);
    for token in &tokens {
        let next_state = StateSet::advance(rules, &state_sets[..], token);
        state_sets.push(next_state);
    }
    println!("{:#?}", state_sets);
    let end_index = tokens.len();
    let last_set = &state_sets[end_index];
    let root_state = last_set
        .states
        .iter()
        .position(|state| {
            state.is_complete() && state.rule.produced_nonterminal == root_nonterminal
        })
        .unwrap();
    let ast = build_ast(&state_sets[..], &tokens[..], root_state, end_index);
    let ast = ast.unwrap().0;
    println!("{:#?}", ast);
}

macro_rules! components {
    ([] [$($items:tt);*]) => {
        vec![$($items),*]
    };
    ([:$text:tt $($input:tt)*] [$($items:tt);*]) => {
        {
            components!(
                [$($input)*]
                [$($items;)* ({
                    fn eval(token: &Token) -> bool {
                        quote(stringify!($text))(token)
                    }
                    Component::Terminal(
                        concat!("(quote(\"", stringify!($text), "\"))"),
                        eval
                    )
                })]
            )
        }
    };
    ([$nonterminal:ident $($input:tt)*] [$($items:tt);*]) => {
        components!(
            [$($input)*]
            [$($items;)* (Component::Nonterminal(String::from(stringify!($nonterminal))))]
        )
    };
    ([$eval:tt $($input:tt)*] [$($items:tt);*]) => {
        {
            components!(
                [$($input)*]
                [$($items;)* ({
                    fn eval(token: &Token) -> bool {
                        $eval(token)
                    }
                    Component::Terminal(stringify!($eval), eval)
                })]
            )
        }
    };
}

macro_rules! rule {
    ($produced_nonterminal:ident -> $($components:tt)*) => {
        Rule {
            produced_nonterminal: String::from(stringify!($produced_nonterminal)),
            components: components!([$($components)*] []),
            preferred: true,
        }
    };
}

macro_rules! rules {
    ($(($nt:ident -> $($c:tt)*))*) => {
        vec![
            $(
                rule!($nt -> $($c)*)
            ),*
        ]
    }
}

fn any_name(token: &Token) -> bool {
    token.role == "name"
}

fn any_whitespace(token: &Token) -> bool {
    token.role == "whitespace"
}

fn quote(text: &'static str) -> impl Fn(&Token) -> bool {
    move |token: &Token| token.content == text
}

pub fn parse(input: &str) {
    let mut rules = rules![
        (Root -> W Expr W)

        // (ExprList -> Expr)
        // (ExprList -> ExprList Expr)

        (Expr -> Expr1)
        // (Expr2 -> Expr1)
        (Expr1 -> Expr0)

        // (Expr2 -> Expr2 W :+ W Expr1)
        (Expr1 -> Expr1 W :* W Expr0)
        (
            Expr0 ->
            :POPULATED_STRUCT
            W (quote("["))
            W Expr
            // W Expr
            // W Expr
            W (quote("]"))
        )
        (Expr0 -> Expr W :. W :LABEL)
        // (Expr0 -> Expr W :. W :VALUE)
        // (Expr0 -> Expr W :. W :REST)
        // (Expr0 -> Expr W :. W :IS_POPULATED_STRUCT)
        // (Expr0 -> :UNIQUE)
        (W -> (any_whitespace))
        (W -> )
    ];
    let mut identifier = rule!(Expr0 -> (any_name));
    identifier.preferred = false;
    rules.push(identifier);
    println!("{:#?}", rules);
    parse_internal(input, &rules[..], "Root");
}
