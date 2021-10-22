use std::collections::HashSet;

use super::structure::{Environment, ItemId, StructField};
use crate::{
    stage1::structure::TokenTree,
    stage2::structure::{BuiltinOperation, BuiltinValue, Definition},
};

type Parent<'x> = (ItemId<'x>, String);
type Parents<'x> = Vec<Parent<'x>>;
type Path<'x> = Vec<Parent<'x>>;

impl<'x> Environment<'x> {
    pub fn show_all(&self) {
        for (id, item) in &self.items {
            for &context in &item.shown_from {
                println!("{:?}", self.get_code(id, context));
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

    fn token(&self, of: String) -> &str {
        self.vomited_tokens.0.alloc(of)
    }

    pub fn get_code(&self, item: ItemId<'x>, context: ItemId<'x>) -> TokenTree {
        let item = self.items[item].cached_reduction.unwrap_or(item);
        match self.items[item].definition.as_ref().unwrap() {
            Definition::BuiltinOperation(op, args) => {
                let name = match op {
                    BuiltinOperation::Sum32U => "sum_32u",
                    BuiltinOperation::Dif32U => "dif_32u",
                    BuiltinOperation::_32UPattern => "32U",
                };
                let body = args
                    .into_iter()
                    .map(|arg| self.get_name_or_code(*arg, context))
                    .collect();
                TokenTree::BuiltinRule { name, body }
            }
            Definition::BuiltinValue(val) => match val {
                BuiltinValue::GodPattern => TokenTree::BuiltinRule {
                    name: "PATTERN",
                    body: vec![],
                },
                BuiltinValue::_32U(val) => TokenTree::Token(self.token(format!("{}", val))),
            },
            Definition::Match {
                base,
                conditions,
                else_value,
            } => {
                todo!()
            }
            Definition::Member(base, name) => {
                let base = self.get_name_or_code(*base, context);
                let member = TokenTree::Token(name);
                TokenTree::BuiltinRule {
                    name: "member",
                    body: vec![base, member],
                }
            }
            Definition::Other(_) => todo!(),
            Definition::Struct(_) => todo!(),
            Definition::Substitute(_, _) => todo!(),
            Definition::Variable(var) => {
                let pattern = self.vars[*var].pattern;
                let pattern = self.get_name_or_code(pattern, context);
                TokenTree::BuiltinRule {
                    name: "any",
                    body: vec![pattern],
                }
            }
        }
    }

    pub fn get_name(&self, of: ItemId<'x>, context: ItemId<'x>) -> Option<TokenTree> {
        let of = self.items[of].cached_reduction.unwrap_or(of);
        let all_context_parents: HashSet<ItemId<'x>> = self
            .get_paths(context)
            .into_iter()
            .map(|p| p[0].0)
            .collect();
        let reachable_paths = self
            .get_paths(of)
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

    fn get_parents(&self, of: ItemId<'x>) -> Parents<'x> {
        let mut parents = Parents::new();
        for (candidate_id, candidate) in &self.items {
            if let Definition::Struct(fields) = candidate.definition.as_ref().unwrap() {
                note_occurences_of_item(&mut parents, of, candidate_id, &fields[..]);
            }
        }
        parents
    }

    fn get_paths(&self, item: ItemId<'x>) -> Vec<Path<'x>> {
        let mut result = vec![];
        for parent in self.get_parents(item) {
            result.push(vec![parent.clone()]);
            for path in self.get_paths(parent.0) {
                result.push([path, vec![parent.clone()]].concat());
            }
        }
        result
    }
}

fn note_occurences_of_item<'x>(
    parents: &mut Parents<'x>,
    item: ItemId<'x>,
    struct_id: ItemId<'x>,
    fields: &[StructField],
) {
    let mut index = 0;
    for field in fields {
        if field.value == item {
            let name = field_name(field, index);
            parents.push((struct_id, name))
        }
        index += 1;
    }
}

fn field_name(field: &StructField, index: i32) -> String {
    let name = field.name.clone().unwrap_or(format!("{}", index));
    name
}