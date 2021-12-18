use std::{
    collections::HashSet,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
};

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

use regex::Regex;
use Component::*;

use crate::shared::indented;

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
        if let Some(Terminal(name, matcher)) =
            self.rule.components.get(self.current_position_in_rule)
        {
            println!(
                "Next terminal is {}: matches {:?}? {}",
                name,
                token,
                matcher(token)
            );
            matcher(token)
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
            states: HashSet::new(),
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
        println!("{:?}", token);
        for state in &previous.states {
            if state.immediate_next_terminal_matches(token) {
                println!("{:?}", state);
                self.states.insert(state.advanced());
            }
        }
    }

    fn complete(&mut self, previous: &[Self]) {
        let mut new = HashSet::new();
        for state in &self.states {
            if state.is_complete() {
                let idx = state.start_position_in_input;
                let previous_states = if idx < previous.len() {
                    &previous[idx].states
                } else {
                    &self.states
                };
                for previous_state in previous_states {
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
    nonterminal: &str,
    end_index: usize,
) -> Result<(AstNode<'a>, usize), String> {
    let state_set_here = &sets[end_index];
    let mut rule = None;
    let mut low_priority_rule = None;
    for state in &state_set_here.states {
        if state.is_complete() && state.rule.produced_nonterminal == nonterminal {
            if state.rule.preferred {
                rule = Some(state.rule);
            } else {
                low_priority_rule = Some(state.rule);
            }
        }
    }
    let rule = if let Some(rule) = rule.or(low_priority_rule) {
        rule
    } else {
        // The parse was not successful.
        return Err(format!("Parsing {} failed", nonterminal));
    };
    let mut components = Vec::new();
    let mut input_index = end_index;
    for component in rule.components.iter().rev() {
        match component {
            Nonterminal(nonterminal) => {
                let (node, new_input_index) = build_ast(sets, tokens, nonterminal, input_index)?;
                components.push(node);
                input_index = new_input_index;
            }
            Terminal(_, _) => {
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
    let ast = build_ast(&state_sets[..], &tokens[..], root_nonterminal, tokens.len());
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
    move |token: &Token| {
        println!("Expecting {:?}, got {:?}", text, token.content);
        token.content == text
    }
}

pub fn parse(input: &str) {
    let mut rules = rules![
        (Root -> W Expr W)

        (ExprList -> Expr)
        (ExprList -> ExprList Expr)

        (Expr -> Expr2)
        (Expr2 -> Expr1)
        (Expr1 -> Expr0)

        (Expr2 -> Expr2 W :+ W Expr1)
        (Expr1 -> Expr1 W :* W Expr0)
        (
            Expr0 ->
            :POPULATED_STRUCT
            W (quote("["))
            W Expr
            W Expr
            W Expr
            W (quote("]"))
        )
        (Expr0 -> :UNIQUE)
        (W -> (any_whitespace))
        (W -> )
    ];
    let mut identifier = rule!(Expr0 -> (any_name));
    identifier.preferred = false;
    rules.push(identifier);
    println!("{:#?}", rules);
    parse_internal(input, &rules[..], "Root");
}
