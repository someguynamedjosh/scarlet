use std::collections::HashSet;

use super::structure::{Environment, ItemId, StructField, VariableId};
use crate::stage2::structure::{
    BuiltinOperation, BuiltinValue, Definition, Member, Token, VarType,
};

type Parent<'x> = (ItemId<'x>, String);
type Parents<'x> = Vec<Parent<'x>>;
type Path<'x> = Vec<Parent<'x>>;

impl<'x> Environment<'x> {
    pub fn show_all(&self) {
        for (id, item) in &self.items {
            for &context in &item.shown_from {
                println!(
                    "\n{:?} is\n{:#?}",
                    self.get_name(id, context)
                        .unwrap_or(Token::Plain("anonymous")),
                    self.get_code(id, context)
                );
            }
        }
    }

    pub fn get_name_or_code(&self, item: ItemId<'x>, context: ItemId<'x>) -> Token {
        if let Some(name) = self.get_name(item, context) {
            name
        } else {
            self.get_code(item, context)
        }
    }

    pub fn get_var_name_or_code(&self, var: VariableId<'x>, context: ItemId<'x>) -> Token {
        for (item_id, _) in &self.items {
            if let Definition::Variable { var: var_id, .. } = self.get_definition(item_id) {
                if *var_id == var {
                    if let Some(name) = self.get_name(item_id, context) {
                        return name;
                    }
                }
            }
        }
        for (item_id, _) in &self.items {
            if let Definition::Variable { var: var_id, .. } = self.get_definition(item_id) {
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

    pub fn get_code(&self, item: ItemId<'x>, context: ItemId<'x>) -> Token {
        let item = self.items[item].cached_reduction.unwrap_or(item);
        match self.get_definition(item).clone() {
            Definition::BuiltinOperation(op, args) => {
                let label = match op {
                    BuiltinOperation::Sum32U => "sum_32u",
                    BuiltinOperation::Difference32U => "difference_32u",
                    BuiltinOperation::Product32U => "product_32u",
                    BuiltinOperation::Quotient32U => "quotient_32u",
                    BuiltinOperation::Power32U => "power_32u",
                    BuiltinOperation::Modulo32U => "modulo_32u",

                    BuiltinOperation::GreaterThan32U => "greater_than_32u",
                    BuiltinOperation::GreaterThanOrEqual32U => "greater_than_or_equal_32u",
                    BuiltinOperation::LessThan32U => "less_than_32u",
                    BuiltinOperation::LessThanOrEqual32U => "less_than_or_equal_32u",
                };
                let contents = args
                    .into_iter()
                    .map(|arg| self.get_name_or_code(arg, context))
                    .collect();
                Token::Stream { label, contents }
            }
            Definition::BuiltinValue(val) => match val {
                BuiltinValue::_32U(val) => Token::Plain(self.token(format!("{}", val))),
                BuiltinValue::Bool(val) => match val {
                    true => Token::Plain("true"),
                    false => Token::Plain("false"),
                },
            },
            Definition::Match {
                base,
                conditions,
                else_value,
            } => {
                let base = self.get_name_or_code(base, context);

                let mut patterns = Vec::new();
                for cond in conditions {
                    let pattern = self.get_name_or_code(cond.pattern, context);
                    let value = self.get_name_or_code(cond.value, context);
                    patterns.push(Token::Stream {
                        label: "on",
                        contents: vec![pattern, value],
                    });
                }

                let else_value = self.get_name_or_code(else_value, context);
                patterns.push(Token::Stream {
                    label: "else",
                    contents: vec![else_value],
                });

                let patterns = Token::Stream {
                    label: "patterns",
                    contents: patterns,
                };
                Token::Stream {
                    label: "match",
                    contents: vec![base, patterns],
                }
            }
            Definition::Member(base, _) => {
                let base = self.get_name_or_code(base, context);
                let member = if let Definition::Member(_, member) = self.get_definition(item) {
                    member
                } else {
                    unreachable!()
                };
                match member {
                    Member::Named(name) => {
                        let name = Token::Plain(name);
                        Token::Stream {
                            label: "member",
                            contents: vec![base, name],
                        }
                    }
                    &Member::Index {
                        index,
                        proof_lt_len,
                    } => Token::Stream {
                        label: "member",
                        contents: vec![
                            base,
                            self.get_name_or_code(index, context),
                            self.get_name_or_code(proof_lt_len, context),
                        ],
                    },
                }
            }
            Definition::Resolvable { .. } => todo!(),
            Definition::SetEager { base, vals, eager } => {
                let base = self.get_name_or_code(base, context);
                let vals = vals
                    .into_iter()
                    .map(|v| self.get_name_or_code(v, context))
                    .collect();
                let vals = Token::Stream {
                    label: "vals",
                    contents: vals,
                };
                Token::Stream {
                    label: if eager { "eager" } else { "shy" },
                    contents: vec![vals, base],
                }
            }
            Definition::Struct(fields) => {
                let mut contents = Vec::new();
                for field in fields {
                    let value = self.get_name_or_code(field.value, context);
                    contents.push(match &field.name {
                        Some(name) => Token::Stream {
                            label: "target",
                            contents: vec![Token::Plain(name), value],
                        },
                        None => value,
                    })
                }
                Token::Stream {
                    label: "struct",
                    contents,
                }
            }
            Definition::Substitute(base, subs) => {
                let base = self.get_name_or_code(base, context);
                let mut tt_subs = Vec::new();
                for (target, value) in subs {
                    let value = self.get_name_or_code(value, context);
                    let target = self.get_var_name_or_code(target, context);
                    tt_subs.push(Token::Stream {
                        label: "target",
                        contents: vec![target, value],
                    })
                }
                let tt_subs = Token::Stream {
                    label: "substitutions",
                    contents: tt_subs,
                };
                Token::Stream {
                    label: "substitute",
                    contents: vec![base, tt_subs],
                }
            }
            Definition::Variable { typee, .. } => {
                // let typee = self.get_name_or_code(typee, context);
                match typee {
                    VarType::God => Token::Stream {
                        label: "PATTERN",
                        contents: vec![],
                    },
                    VarType::_32U => Token::Stream {
                        label: "32U",
                        contents: vec![],
                    },
                    VarType::Bool => Token::Stream {
                        label: "BOOL",
                        contents: vec![],
                    },
                    VarType::Just(other) => Token::Stream {
                        label: "variable",
                        contents: vec![self.get_name_or_code(other, context)],
                    },
                    VarType::And(left, right) => Token::Stream {
                        label: "AND",
                        contents: vec![
                            self.get_name_or_code(left, context),
                            self.get_name_or_code(right, context),
                        ],
                    },
                    VarType::Or(left, right) => Token::Stream {
                        label: "OR",
                        contents: vec![
                            self.get_name_or_code(left, context),
                            self.get_name_or_code(right, context),
                        ],
                    },
                }
            }
        }
    }

    fn dereference(&self, item: ItemId<'x>, context: ItemId<'x>) -> ItemId<'x> {
        let mut item = item;
        while let Definition::Resolvable(Token::Item(other))
        | Definition::SetEager { base: other, .. } =
            self.items[item].definition.as_ref().unwrap()
        {
            item = *other;
        }
        if let Some(reduced) = self.items[item].cached_reduction {
            if reduced != item {
                return self.dereference(reduced, context);
            }
        }
        item
    }

    pub fn get_name(&self, of: ItemId<'x>, context: ItemId<'x>) -> Option<Token> {
        let of = self.dereference(of, context);
        self.get_name_impl(of, context)
    }

    pub fn get_name_impl(&self, of: ItemId<'x>, context: ItemId<'x>) -> Option<Token> {
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
            let mut result = Token::Plain(self.token(base.1));
            for (_, member) in path {
                result = Token::Stream {
                    label: "member",
                    contents: vec![result, Token::Plain(self.token(member))],
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
        let item = self.dereference(item, context);
        let mut index = 0;
        for field in fields {
            let value = self.dereference(field.value, context);
            if self.get_definition(value) == self.get_definition(item) {
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
