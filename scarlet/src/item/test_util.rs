#![cfg(test)]

use std::sync::Mutex;

use lazy_static::lazy_static;

use super::{
    definitions::{other::DOther, variable::VariablePtr},
    resolve::resolve_all,
    Item,
};
use crate::{
    environment::Environment,
    file_tree::FileNode,
    item::{
        definitions::{
            structt::DPopulatedStruct,
            substitution::Substitutions,
            unique::DUnique,
            variable::{DVariable, VariableOrder},
        },
        ItemPtr,
    },
    parser::{parse_tree, ParseContext},
    scope::SRoot,
    util::PtrExtension,
};

lazy_static! {
    static ref VARIABLE_COUNTER: Mutex<u32> = Mutex::new(0);
}

pub(super) fn env() -> Environment {
    let mut env = Environment::new();
    add_language_items(&mut env, "");
    env
}

pub(super) fn with_env_from_code(code: &str, callback: impl FnOnce(Environment, ItemPtr)) {
    let node = FileNode {
        self_content: code.to_owned(),
        children: Vec::new(),
    };
    let pc = ParseContext::new();
    let (mut env, root) = env_from_code(&node, &pc);
    add_language_items(&mut env, code);
    resolve_all(&mut env, root.ptr_clone()).unwrap();

    callback(env, root)
}

fn add_language_items(env: &mut Environment, code_containing_definitions_to_skip: &str) {
    for lang_item_name in env.language_item_names() {
        if code_containing_definitions_to_skip
            .contains(&format!("AS_LANGUAGE_ITEM({})", lang_item_name))
        {
            continue;
        }
        let def = if ["x", "y", "when_equal", "when_not_equal"].contains(&lang_item_name) {
            variable()
        } else {
            unique()
        };
        def.set_name(lang_item_name.to_owned());
        env.define_language_item(lang_item_name, def);
    }
}

fn env_from_code<'x>(code: &'x FileNode, pc: &'x ParseContext) -> (Environment, ItemPtr) {
    let mut file_counter = 0;
    let parsed = parse_tree(code, &pc, &mut file_counter).unwrap();

    let mut env = env();
    let root = parsed.as_item(&pc, &mut env, SRoot).unwrap();

    (env, root)
}

pub(super) fn subs(from: Vec<(VariablePtr, ItemPtr)>) -> Substitutions {
    from.into_iter().collect()
}

pub(super) fn unique() -> ItemPtr {
    Item::new(DUnique::new(), SRoot)
}

fn next_variable_order() -> u32 {
    let mut ptr = VARIABLE_COUNTER.lock().unwrap();
    let value = *ptr;
    *ptr += 1;
    value
}

pub(super) fn variable() -> ItemPtr {
    let order = VariableOrder::new(0, 0, next_variable_order());
    DVariable::new_value(vec![], order, Box::new(SRoot))
}

fn extract_var_ptr_from_item_ptr(item_ptr: &ItemPtr) -> VariablePtr {
    item_ptr
        .downcast_definition::<DVariable>()
        .unwrap()
        .get_variable()
        .ptr_clone()
}

pub(super) fn variable_full() -> (ItemPtr, VariablePtr) {
    let item = variable();
    let var = extract_var_ptr_from_item_ptr(&item);
    (item, var)
}

pub(super) fn variable_full_with_deps(deps: Vec<ItemPtr>) -> (ItemPtr, VariablePtr) {
    let order = VariableOrder::new(0, 0, next_variable_order());
    let item = DVariable::new_value(deps, order, Box::new(SRoot));
    let var = extract_var_ptr_from_item_ptr(&item);
    (item, var)
}

pub(super) fn structt(mut fields: Vec<(&str, ItemPtr)>, void: &ItemPtr) -> ItemPtr {
    if fields.len() == 0 {
        void.ptr_clone()
    } else {
        let tail = fields.pop().unwrap();
        let body = structt(fields, void);
        Item::new(
            DPopulatedStruct::new(body, tail.0.to_owned(), tail.1),
            SRoot,
        )
    }
}

pub(super) fn other(base: ItemPtr) -> ItemPtr {
    return Item::new(DOther::new(base), SRoot);
}

pub(super) fn get_member(root: &ItemPtr, name: &str) -> ItemPtr {
    root.downcast_definition::<DPopulatedStruct>()
        .unwrap()
        .get_tail_value()
        .lookup_ident(name)
        .unwrap()
        .unwrap()
}
