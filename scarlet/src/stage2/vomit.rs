use std::collections::HashSet;

use super::structure::{Environment, ItemId, StructField, VariableId};
use crate::{
    stage1::structure::TokenTree,
    stage2::structure::{BuiltinOperation, BuiltinPattern, BuiltinValue, Definition, Target},
};

type Parent<'x> = (ItemId<'x>, String);
type Parents<'x> = Vec<Parent<'x>>;
type Path<'x> = Vec<Parent<'x>>;

impl<'x> Environment<'x> {
    pub fn show_all(&self) {
        for (id, item) in &self.items {
            for &context in &item.shown_from {
                println!(
                    "{:?} is\n{:#?}",
                    self.get_name(id, context)
                        .unwrap_or(TokenTree::Token("anonymous")),
                    self.get_code(id, context)
                );
            }
        }
    }

    pub fn get_name_or_code(&self, item: ItemId<'x>, context: ItemId<'x>) -> TokenTree {
        if let Some(name) = self.get_name(item, context) {
            name
        } else {
            self.get_code(item, context)
        }
    }

    pub fn get_var_name_or_code(&self, var: VariableId<'x>, context: ItemId<'x>) -> TokenTree {
        for (item_id, _) in &self.items {
            if let Definition::Variable { var: var_id, .. } = self.definition_of(item_id) {
                if *var_id == var {
                    if let Some(name) = self.get_name(item_id, context) {
                        return name;
                    }
                }
            }
        }
        for (item_id, _) in &self.items {
            if let Definition::Variable { var: var_id, .. } = self.definition_of(item_id) {
                if *var_id == var {
                    return self.get_name_or_code(item_id, context);
                }
            }
        }
        unreachable!()
    }

    fn token(&self, of: String) -> &str {
        self.vomited_tokens.0.alloc(of)
    }

    pub fn get_code(&self, item: ItemId<'x>, context: ItemId<'x>) -> TokenTree {
        let item = self.items[item].cached_reduction.unwrap_or(item);
        match self.items[item].definition.as_ref().unwrap() {
            Definition::After { base, vals } => {
                let vals = vals
                    .into_iter()
                    .map(|val| self.get_name_or_code(*val, context))
                    .collect();
                let vals = TokenTree::BuiltinRule {
                    name: "vals",
                    body: vals,
                };
                let base = self.get_name_or_code(*base, context);
                TokenTree::BuiltinRule {
                    name: "after",
                    body: vec![vals, base],
                }
            }
            Definition::BuiltinOperation(op, args) => {
                let name = match op {
                    BuiltinOperation::Sum32U => "sum_32u",
                    BuiltinOperation::Dif32U => "dif_32u",
                };
                let body = args
                    .into_iter()
                    .map(|arg| self.get_name_or_code(*arg, context))
                    .collect();
                TokenTree::BuiltinRule { name, body }
            }
            Definition::BuiltinPattern(pat) => match pat {
                BuiltinPattern::God => TokenTree::BuiltinRule {
                    name: "God",
                    body: vec![],
                },
                BuiltinPattern::_32U => TokenTree::BuiltinRule {
                    name: "32U",
                    body: vec![],
                },
                BuiltinPattern::Bool => TokenTree::BuiltinRule {
                    name: "Bool",
                    body: vec![],
                },
                BuiltinPattern::And(left, right) => TokenTree::BuiltinRule {
                    name: "AND",
                    body: vec![
                        self.get_name_or_code(*left, context),
                        self.get_name_or_code(*right, context),
                    ],
                },
            },
            Definition::BuiltinValue(val) => match val {
                BuiltinValue::_32U(val) => TokenTree::Token(self.token(format!("{}", val))),
                BuiltinValue::Bool(val) => match *val {
                    true => TokenTree::Token("true"),
                    false => TokenTree::Token("false"),
                },
            },
            Definition::Match {
                base,
                conditions,
                else_value,
            } => {
                let base = self.get_name_or_code(*base, context);

                let mut patterns = Vec::new();
                for cond in conditions {
                    let pattern = self.get_name_or_code(cond.pattern, context);
                    let value = self.get_name_or_code(cond.value, context);
                    patterns.push(TokenTree::BuiltinRule {
                        name: "on",
                        body: vec![pattern, value],
                    });
                }

                let else_value = self.get_name_or_code(*else_value, context);
                patterns.push(TokenTree::BuiltinRule {
                    name: "else",
                    body: vec![else_value],
                });

                let patterns = TokenTree::BuiltinRule {
                    name: "patterns",
                    body: patterns,
                };
                TokenTree::BuiltinRule {
                    name: "match",
                    body: vec![base, patterns],
                }
            }
            Definition::Member(base, name) => {
                let base = self.get_name_or_code(*base, context);
                let member = TokenTree::Token(name);
                TokenTree::BuiltinRule {
                    name: "member",
                    body: vec![base, member],
                }
            }
            Definition::Other(item) => self.get_code(*item, context),
            Definition::Struct(fields) => {
                let mut body = Vec::new();
                for field in fields {
                    let value = self.get_name_or_code(field.value, context);
                    body.push(match &field.name {
                        Some(name) => TokenTree::BuiltinRule {
                            name: "target",
                            body: vec![TokenTree::Token(name), value],
                        },
                        None => value,
                    })
                }
                TokenTree::BuiltinRule {
                    name: "struct",
                    body,
                }
            }
            Definition::Substitute(base, subs) => {
                let base = self.get_name_or_code(*base, context);
                let mut tt_subs = Vec::new();
                for sub in subs {
                    let value = self.get_name_or_code(sub.value, context);
                    let target = if let Target::ResolvedItem(target) = sub.target {
                        self.get_name_or_code(target, context)
                    } else {
                        unreachable!()
                    };
                    tt_subs.push(TokenTree::BuiltinRule {
                        name: "target",
                        body: vec![target, value],
                    })
                }
                let tt_subs = TokenTree::BuiltinRule {
                    name: "substitutions",
                    body: tt_subs,
                };
                TokenTree::BuiltinRule {
                    name: "substitute",
                    body: vec![base, tt_subs],
                }
            }
            Definition::Variable { var: _, matches } => {
                let matches = self.get_name_or_code(*matches, context);
                TokenTree::BuiltinRule {
                    name: "any",
                    body: vec![matches],
                }
            }
        }
    }

    fn dereference(&self, item: ItemId<'x>, context: ItemId<'x>) -> (ItemId<'x>, Vec<TokenTree>) {
        let mut item = item;
        let mut afters = Vec::new();
        while let Definition::Other(other) = self.items[item].definition.as_ref().unwrap() {
            match self.items[item].after.clone() {
                None => (),
                Some(vars) => {
                    for (var, _) in vars {
                        afters.push(self.get_var_name_or_code(var.var, context))
                    }
                }
            }
            item = *other;
        }
        if let Some(reduced) = self.items[item].cached_reduction {
            if reduced != item {
                return self.dereference(reduced, context);
            }
        }
        (item, afters)
    }

    fn prepend_afters<'a>(&'a self, base: TokenTree<'a>, afters: Vec<TokenTree<'a>>) -> TokenTree {
        if afters.len() == 0 {
            base
        } else {
            let vals = TokenTree::BuiltinRule {
                name: "vals",
                body: afters,
            };
            TokenTree::BuiltinRule {
                name: "after",
                body: vec![vals, base],
            }
        }
    }

    pub fn get_name(&self, of: ItemId<'x>, context: ItemId<'x>) -> Option<TokenTree> {
        let (of, afters) = self.dereference(of, context);
        let name = self.get_name_impl(of, context);
        name.map(|name| self.prepend_afters(name, afters))
    }

    pub fn get_name_impl(&self, of: ItemId<'x>, context: ItemId<'x>) -> Option<TokenTree> {
        let all_context_parents: HashSet<ItemId<'x>> = self
            .get_paths(context, context)
            .into_iter()
            .map(|p| p[0].0)
            .collect();
        let reachable_paths = self
            .get_paths(of, context)
            .into_iter()
            .filter(|p| all_context_parents.contains(&p[0].0));
        let path = reachable_paths.min_by_key(|p| p.len());
        path.map(|mut path| {
            let base = path.remove(0);
            let mut result = TokenTree::Token(self.token(base.1));
            for (_, member) in path {
                result = TokenTree::BuiltinRule {
                    name: "member",
                    body: vec![result, TokenTree::Token(self.token(member))],
                }
            }
            result
        })
    }

    fn get_parents(&self, of: ItemId<'x>, context: ItemId<'x>) -> Parents<'x> {
        let mut parents = Parents::new();
        for (candidate_id, candidate) in &self.items {
            if let Definition::Struct(fields) = candidate.definition.as_ref().unwrap() {
                self.note_occurences_of_item(&mut parents, of, context, candidate_id, &fields[..]);
            }
        }
        parents
    }

    fn note_occurences_of_item(
        &self,
        parents: &mut Parents<'x>,
        item: ItemId<'x>,
        context: ItemId<'x>,
        struct_id: ItemId<'x>,
        fields: &[StructField],
    ) {
        let (item, _) = self.dereference(item, context);
        let mut index = 0;
        for field in fields {
            let value = self.dereference(field.value, context).0;
            if self.definition_of(value) == self.definition_of(item) {
                let name = field_name(field, index);
                parents.push((struct_id, name))
            }
            index += 1;
        }
    }

    fn get_paths(&self, item: ItemId<'x>, context: ItemId<'x>) -> Vec<Path<'x>> {
        let mut result = vec![];
        for parent in self.get_parents(item, context) {
            result.push(vec![parent.clone()]);
            for path in self.get_paths(parent.0, context) {
                result.push([path, vec![parent.clone()]].concat());
            }
        }
        result
    }
}

fn field_name(field: &StructField, index: i32) -> String {
    field
        .name
        .map(ToOwned::to_owned)
        .unwrap_or(format!("{}", index))
}
