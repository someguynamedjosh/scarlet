mod pattern;
mod plain_match;
mod rule;
mod stealing_match;
mod structs;

use self::{
    rule::{build_rules, Rule},
    structs::{MatchComp, RuleMatch, RuleMatcher},
};
use super::structure::SyntaxNode;
use crate::stage1::structure::Token;

pub fn ingest<'a, 't>(tokens: &'a [Token<'t>]) -> SyntaxNode<'t> {
    let mut matcher = RuleMatcher::new(tokens);
    matcher.process();
    matcher.finalize()
}

impl<'a, 't> RuleMatcher<'a, 't> {
    fn try_rules(&mut self, rules: &[Rule]) {
        for rule in rules {
            if let Some(matchh) = self.rule_is_plain_match(rule) {
                self.push_plain_match(matchh);
                self.try_rules(rules);
                return;
            } else if let Some(matchh) = self.rule_is_stealing_match(rule) {
                self.push_stolen_match(matchh);
                self.try_rules(rules);
                return;
            }
        }
    }

    fn push_stolen_match(&mut self, matchh: structs::RuleMatch<'t>) {
        for _ in 0..matchh.elements.len() - 1 {
            self.output.remove(0);
        }
        let steal_from = if let MatchComp::RuleMatch(matchh) = &mut self.output[0] {
            matchh
        } else {
            unreachable!()
        };
        steal_from.elements[0].1 = MatchComp::RuleMatch(matchh);
    }

    fn push_plain_match(&mut self, matchh: structs::RuleMatch<'t>) {
        for _ in 0..matchh.elements.len() {
            self.output.remove(0);
        }
        let comp = MatchComp::RuleMatch(matchh);
        self.output.push(comp);
    }

    fn process(&mut self) {
        let rules = build_rules();
        for token in self.tokens.iter().rev() {
            self.output.insert(0, MatchComp::Token(*token));
            self.try_rules(&rules[..]);
        }
    }

    fn eject_rule(&self, rule: &RuleMatch<'t>) -> SyntaxNode<'t> {
        let elements = rule
            .elements
            .iter()
            .map(|x| self.eject_component(&x.1))
            .collect();
        let name = rule.name.clone();
        SyntaxNode::Rule { elements, name }
    }

    fn eject_component(&self, component: &MatchComp<'t>) -> SyntaxNode<'t> {
        match component {
            MatchComp::Token(token) => SyntaxNode::Token(token),
            MatchComp::RuleMatch(rule_match) => self.eject_rule(rule_match),
        }
    }

    fn finalize(&self) -> SyntaxNode<'t> {
        println!("{:#?}", self);
        assert_eq!(self.output.len(), 1);
        self.eject_component(&self.output[0])
    }
}
