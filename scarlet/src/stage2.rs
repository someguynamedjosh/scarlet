use crate::stage1::structure::Token;

pub type Precedence = u8;

#[derive(Clone, Debug)]
pub enum AtomicPat {
    ExactToken(Token<'static>),
    Expression { max_precedence: Precedence },
}

#[derive(Clone, Debug)]
pub enum Pattern {
    Atomic(AtomicPat),
    Composite(Vec<Pattern>),
    Repeat(Box<Pattern>),
}

fn rep(base: Pattern) -> Pattern {
    Pattern::Repeat(Box::new(base))
}

impl From<Token<'static>> for Pattern {
    fn from(token: Token<'static>) -> Self {
        Self::Atomic(AtomicPat::ExactToken(token))
    }
}

impl From<Precedence> for Pattern {
    fn from(max_precedence: Precedence) -> Self {
        Self::Atomic(AtomicPat::Expression { max_precedence })
    }
}

impl From<Vec<Pattern>> for Pattern {
    fn from(parts: Vec<Pattern>) -> Self {
        Self::Composite(parts)
    }
}

macro_rules! pattern {
    ([$($pat:expr),*]) => {
        vec![$(pattern!($pat)),*].into()
    };
    ($pat:expr) => {
        $pat.into()
    };
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub name: String,
    pub pattern: Pattern,
    pub result_precedence: Precedence,
}

pub fn build_rules() -> Vec<Rule> {
    vec![
        Rule {
            name: format!("+"),
            pattern: pattern!([80, "+", 79]),
            result_precedence: 80,
        },
        Rule {
            name: format!("*"),
            pattern: pattern!([70, "*", 69]),
            result_precedence: 70,
        },
        Rule {
            name: format!("^"),
            pattern: pattern!([59, "^", 60]),
            result_precedence: 60,
        },
        Rule {
            name: format!("paren"),
            pattern: pattern!(["(", rep(pattern!(255)), ")"]),
            result_precedence: 1,
        },
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchComp {
    Token(usize),
    RuleMatch(usize),
}

pub type PatternMatch = Vec<(AtomicPat, MatchComp)>;

#[derive(Clone, Debug)]
pub struct RuleMatch {
    pub elements: PatternMatch,
    pub name: String,
    pub precedence: Precedence,
}

#[derive(Clone, Debug)]
pub struct RuleMatcher<'a, 't> {
    pub output: Vec<MatchComp>,
    pub tokens: &'a [Token<'t>],
    pub matches: Vec<RuleMatch>,
}

pub fn ingest<'a, 't>(tokens: &'a [Token<'t>]) -> RuleMatcher<'a, 't> {
    let mut matcher = RuleMatcher::new(tokens);
    matcher.process();
    matcher
}

impl<'a, 't> RuleMatcher<'a, 't> {
    fn new(tokens: &'a [Token<'t>]) -> Self {
        Self {
            output: Vec::new(),
            tokens,
            matches: Vec::new(),
        }
    }

    fn atomic_is_plain_match(
        &self,
        pattern: &AtomicPat,
        component: MatchComp,
    ) -> Option<MatchComp> {
        match (pattern, component) {
            (AtomicPat::ExactToken(expected), MatchComp::Token(index)) => {
                if self.tokens[index] == *expected {
                    Some(component)
                } else {
                    None
                }
            }
            (AtomicPat::Expression { .. }, MatchComp::Token(..)) => Some(component),
            (AtomicPat::Expression { max_precedence }, MatchComp::RuleMatch(matchh_index)) => {
                let matchh = &self.matches[matchh_index];
                if matchh.precedence <= *max_precedence {
                    Some(component)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn composite_is_plain_match(
        &self,
        elements: &[Pattern],
        remaining_output: &[MatchComp],
    ) -> Option<PatternMatch> {
        if elements.len() > remaining_output.len() {
            return None;
        }
        let mut result = PatternMatch::new();
        let mut remaining_output = remaining_output;
        for element in elements {
            let mut matchh = self.pattern_is_plain_match(element, remaining_output)?;
            remaining_output = &remaining_output[matchh.len()..];
            result.append(&mut matchh);
        }
        Some(result)
    }

    fn repeat_is_plain_match(
        &self,
        repeated: &Pattern,
        remaining_output: &[MatchComp],
    ) -> Option<PatternMatch> {
        let mut result = PatternMatch::new();
        let mut remaining_output = remaining_output;
        while let Some(mut matchh) = self.pattern_is_plain_match(repeated, remaining_output) {
            remaining_output = &remaining_output[matchh.len()..];
            result.append(&mut matchh);
        }
        Some(result)
    }

    fn pattern_is_plain_match(
        &self,
        pattern: &Pattern,
        remaining_output: &[MatchComp],
    ) -> Option<PatternMatch> {
        match pattern {
            Pattern::Atomic(pat) => {
                if remaining_output.len() == 0 {
                    None
                } else {
                    let matchh = self.atomic_is_plain_match(pat, remaining_output[0])?;
                    Some(vec![(pat.clone(), matchh)])
                }
            }
            Pattern::Composite(elements) => {
                self.composite_is_plain_match(elements, remaining_output)
            }
            Pattern::Repeat(repeated) => self.repeat_is_plain_match(repeated, remaining_output),
        }
    }

    /// Returns a RuleMatch if the given rule matches the current output without
    /// stealing from any existing rule matches.
    fn rule_is_plain_match(&self, rule: &Rule) -> Option<RuleMatch> {
        self.pattern_is_plain_match(&rule.pattern, &self.output[..])
            .map(|matchh| RuleMatch {
                elements: matchh,
                name: rule.name.clone(),
                precedence: rule.result_precedence,
            })
    }

    fn atomic_is_stealing_match(
        &self,
        pattern: &AtomicPat,
        component: MatchComp,
        parent_rule_precedence: Precedence,
    ) -> Option<MatchComp> {
        match component {
            MatchComp::RuleMatch(index) => {
                let first_element = &self.matches[index].elements[0];
                match first_element.0 {
                    AtomicPat::ExactToken(..) => None,
                    AtomicPat::Expression { max_precedence } => {
                        if parent_rule_precedence > max_precedence {
                            None
                        } else {
                            self.atomic_is_plain_match(pattern, first_element.1)
                        }
                    }
                }
            }
            _ => None,
        }
    }

    fn composite_is_stealing_match(
        &self,
        elements: &[Pattern],
        remaining_output: &[MatchComp],
        parent_rule_precedence: Precedence,
    ) -> Option<PatternMatch> {
        debug_assert!(elements.len() > 0);
        if elements.len() > remaining_output.len() {
            return None;
        }
        let mut result = PatternMatch::new();
        let mut remaining_output = remaining_output;
        let last = elements.len() - 1;
        for element in &elements[..last] {
            let mut matchh = self.pattern_is_plain_match(element, remaining_output)?;
            remaining_output = &remaining_output[matchh.len()..];
            result.append(&mut matchh);
        }
        let mut matchh = self.pattern_is_stealing_match(
            &elements[last],
            remaining_output,
            parent_rule_precedence,
        )?;
        result.append(&mut matchh);
        Some(result)
    }

    fn repeat_is_stealing_match(
        &self,
        repeated: &Pattern,
        remaining_output: &[MatchComp],
        parent_rule_precedence: Precedence,
    ) -> Option<PatternMatch> {
        let mut result = PatternMatch::new();
        let mut remaining_output = remaining_output;
        while let Some(mut matchh) = self.pattern_is_plain_match(repeated, remaining_output) {
            remaining_output = &remaining_output[matchh.len()..];
            result.append(&mut matchh);
        }
        let mut matchh =
            self.pattern_is_stealing_match(repeated, remaining_output, parent_rule_precedence)?;
        result.append(&mut matchh);
        Some(result)
    }

    fn pattern_is_stealing_match(
        &self,
        pattern: &Pattern,
        remaining_output: &[MatchComp],
        parent_rule_precedence: Precedence,
    ) -> Option<PatternMatch> {
        match pattern {
            Pattern::Atomic(pat) => {
                if remaining_output.len() == 0 {
                    None
                } else {
                    let matchh = self.atomic_is_stealing_match(
                        pat,
                        remaining_output[0],
                        parent_rule_precedence,
                    )?;
                    Some(vec![(pat.clone(), matchh)])
                }
            }
            Pattern::Composite(elements) => {
                self.composite_is_stealing_match(elements, remaining_output, parent_rule_precedence)
            }
            Pattern::Repeat(repeated) => {
                self.repeat_is_stealing_match(repeated, remaining_output, parent_rule_precedence)
            }
        }
    }

    /// Returns a RuleMatch if the given rule matches the current output without
    /// stealing from any existing rule matches.
    fn rule_is_stealing_match(&self, rule: &Rule) -> Option<RuleMatch> {
        self.pattern_is_stealing_match(&rule.pattern, &self.output[..], rule.result_precedence)
            .map(|matchh| RuleMatch {
                elements: matchh,
                name: rule.name.clone(),
                precedence: rule.result_precedence,
            })
    }

    fn try_rules(&mut self, rules: &[Rule]) {
        for rule in rules {
            if let Some(matchh) = self.rule_is_plain_match(rule) {
                for _ in 0..matchh.elements.len() {
                    self.output.remove(0);
                }
                let comp = MatchComp::RuleMatch(self.matches.len());
                self.matches.push(matchh);
                self.output.push(comp);
                self.try_rules(rules);
                return;
            } else if let Some(matchh) = self.rule_is_stealing_match(rule) {
                for _ in 0..matchh.elements.len() - 1 {
                    self.output.remove(0);
                }
                let comp = MatchComp::RuleMatch(self.matches.len());
                self.matches.push(matchh);
                let steal_from = if let MatchComp::RuleMatch(index) = self.output[0] {
                    &mut self.matches[index]
                } else {
                    unreachable!()
                };
                steal_from.elements[0].1 = comp;
                self.try_rules(rules);
                return;
            }
        }
    }

    fn process(&mut self) {
        let rules = build_rules();
        for token_index in (0..self.tokens.len()).rev() {
            self.output.insert(0, MatchComp::Token(token_index));
            self.try_rules(&rules[..]);
        }
    }
}
