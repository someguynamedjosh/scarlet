#![cfg(test)]

use crate::{
    environment::Environment,
    file_tree::FileNode,
    item::{
        definitions::{
            decision::DDecision,
            structt::DPopulatedStruct,
            substitution::Substitutions,
            unique::DUnique,
            variable::{DVariable, Variable, VariableId, VariableOrder},
        },
        ItemPtr,
    },
    parser::{parse_tree, ParseContext},
    scope::SRoot,
};

pub(super) fn env() -> Environment {
    Environment::new()
}

pub(super) fn with_env_from_code(code: &str, callback: impl FnOnce(Environment, ItemPtr)) {
    let node = FileNode {
        self_content: code.to_owned(),
        children: Vec::new(),
    };
    let pc = ParseContext::new();
    let (mut env, root) = env_from_code(&node, &pc);
    for lang_item_name in env.language_item_names() {
        if code.contains(&format!("AS_LANGUAGE_ITEM[{}]", lang_item_name)) {
            continue;
        }
        let def = env.push_unique();
        let def = env.push_construct(DUnique::new(def), Box::new(SRoot));
        env.set_name(def, lang_item_name.to_owned());
        env.define_language_item(lang_item_name, def);
    }
    env.resolve_all();

    let root = env
        .get_and_downcast_construct_definition::<DPopulatedStruct>(root)
        .unwrap()
        .unwrap()
        .get_value();

    callback(env, root)
}

fn env_from_code<'x>(code: &'x FileNode, pc: &'x ParseContext) -> (Environment, ItemPtr) {
    let mut file_counter = 0;
    let parsed = parse_tree(code, &pc, &mut file_counter);

    let mut env = env();
    let root = parsed.as_construct(&pc, &mut env, SRoot);

    (env, root)
}

pub(super) fn subs(from: Vec<(VariableId, ItemPtr)>) -> Substitutions {
    from.into_iter().collect()
}

impl Environment {
    pub(super) fn decision(
        &mut self,
        left: ItemPtr,
        right: ItemPtr,
        equal: ItemPtr,
        unequal: ItemPtr,
    ) -> ItemPtr {
        self.push_construct(DDecision::new(left, right, equal, unequal), Box::new(SRoot))
    }

    pub(super) fn unique(&mut self) -> ItemPtr {
        let id = self.push_unique();
        self.push_construct(DUnique::new(id), Box::new(SRoot))
    }

    pub(super) fn variable(&mut self) -> ItemPtr {
        let id = self.push_variable(Variable {
            id: None,
            item: None,
            invariants: vec![],
            dependencies: vec![],
            order: VariableOrder::new(0, 0, self.variables.len() as _),
        });
        self.push_construct(DVariable::new(id), Box::new(SRoot))
    }

    pub(super) fn variable_full(&mut self) -> (ItemPtr, VariableId) {
        let id = self.push_variable(Variable {
            id: None,
            item: None,
            invariants: vec![],
            dependencies: vec![],
            order: VariableOrder::new(0, 0, self.variables.len() as _),
        });
        let con = DVariable::new(id);
        let cid = self.push_construct(con.clone(), Box::new(SRoot));
        (cid, id)
    }

    pub(super) fn variable_full_with_deps(&mut self, deps: Vec<ItemPtr>) -> (ItemPtr, VariableId) {
        let id = self.push_variable(Variable {
            id: None,
            item: None,
            invariants: vec![],
            dependencies: deps,
            order: VariableOrder::new(0, 0, self.variables.len() as _),
        });
        let con = DVariable::new(id);
        let cid = self.push_construct(con.clone(), Box::new(SRoot));
        (cid, id)
    }
}
