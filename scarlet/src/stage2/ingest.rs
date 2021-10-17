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
    let rules = build_rules();
    let mut matcher = RuleMatcher::new(&rules[..], tokens);
    matcher.process();
    matcher.finalize()
}

impl<'x, 't> RuleMatcher<'x, 't> {
    fn try_rules(&mut self) {
        for rule in self.rules {
            if let Some(matchh) = self.rule_is_stealing_match(rule) {
                self.push_stolen_match(matchh);
                self.try_rules();
                return;
            } else if let Some(matchh) = self.rule_is_plain_match(rule) {
                self.push_plain_match(matchh);
                self.try_rules();
                return;
            }
        }
    }

    fn push_stolen_match(&mut self, matchh: structs::RuleMatch<'x, 't>) {
        println!("{:#?}", matchh);
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

    fn push_plain_match(&mut self, matchh: structs::RuleMatch<'x, 't>) {
        for _ in 0..matchh.elements.len() {
            self.output.remove(0);
        }
        let comp = MatchComp::RuleMatch(matchh);
        self.output.insert(0, comp);
    }

    fn process(&mut self) {
        for token in self.tokens.iter().rev() {
            self.output.insert(0, MatchComp::Token(*token));
            self.try_rules();
            println!("Pushed {}", token);
            println!("{:#?}", self.output);
        }
    }

    fn eject_rule_match(&self, rule_match: &RuleMatch<'x, 't>) -> SyntaxNode<'t> {
        let elements = rule_match
            .elements
            .iter()
            .map(|x| self.eject_component(&x.1))
            .collect();
        let name = rule_match.rule.name.clone();
        SyntaxNode::Rule { elements, name }
    }

    fn eject_component(&self, component: &MatchComp<'x, 't>) -> SyntaxNode<'t> {
        match component {
            MatchComp::Token(token) => SyntaxNode::Token(token),
            MatchComp::RuleMatch(rule_match) => self.eject_rule_match(rule_match),
        }
    }

    fn finalize(&self) -> SyntaxNode<'t> {
        println!("{:#?}", &self.output);
        assert_eq!(self.output.len(), 1);
        self.eject_component(&self.output[0])
    }
}
