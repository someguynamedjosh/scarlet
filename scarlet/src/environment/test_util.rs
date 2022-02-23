#![cfg(test)]

use crate::{
    constructs::{
        decision::CDecision,
        structt::CPopulatedStruct,
        substitution::Substitutions,
        unique::CUnique,
        variable::{CVariable, Variable, VariableId},
        ConstructId,
    },
    environment::Environment,
    file_tree::FileNode,
    parser::{parse_tree, ParseContext},
    scope::SRoot,
};

pub(super) fn env<'x>() -> Environment<'x> {
    Environment::new()
}

pub(super) fn with_env_from_code(
    code: &str,
    callback: impl for<'x> FnOnce(Environment<'x>, ConstructId),
) {
    let node = FileNode {
        self_content: code.to_owned(),
        children: Vec::new(),
    };
    let pc = ParseContext::new();
    let (mut env, root) = env_from_code(&node, &pc);
    for lang_item_name in env.language_item_names() {
        if code.contains(lang_item_name) {
            continue;
        }
        let def = env.push_unique();
        let def = env.push_construct(CUnique::new(def), Box::new(SRoot));
        env.define_language_item(lang_item_name, def);
    }
    env.resolve_all();
    env.check_all().unwrap();

    let root = env
        .get_and_downcast_construct_definition::<CPopulatedStruct>(root)
        .unwrap()
        .unwrap()
        .get_value();

    callback(env, root)
}

fn env_from_code<'x>(code: &'x FileNode, pc: &'x ParseContext) -> (Environment<'x>, ConstructId) {
    let parsed = parse_tree(code, &pc);

    let mut env = env();
    let root = parsed.as_construct(&pc, &mut env, SRoot);

    (env, root)
}

pub(super) fn subs(from: Vec<(VariableId, ConstructId)>) -> Substitutions {
    from.into_iter().collect()
}

impl<'a> Environment<'a> {
    pub(super) fn decision(
        &mut self,
        left: ConstructId,
        right: ConstructId,
        equal: ConstructId,
        unequal: ConstructId,
    ) -> ConstructId {
        self.push_construct(CDecision::new(left, right, equal, unequal), Box::new(SRoot))
    }

    pub(super) fn unique(&mut self) -> ConstructId {
        let id = self.push_unique();
        self.push_construct(CUnique::new(id), Box::new(SRoot))
    }

    pub(super) fn variable(&mut self) -> ConstructId {
        let id = self.push_variable(Variable {
            id: None,
            construct: None,
            invariants: vec![],
            dependencies: vec![],
        });
        self.push_construct(CVariable::new(id), Box::new(SRoot))
    }

    pub(super) fn variable_full(&mut self) -> (ConstructId, VariableId) {
        let id = self.push_variable(Variable {
            id: None,
            construct: None,
            invariants: vec![],
            dependencies: vec![],
        });
        let con = CVariable::new(id);
        let cid = self.push_construct(con.clone(), Box::new(SRoot));
        (cid, id)
    }

    pub(super) fn variable_full_with_deps(
        &mut self,
        deps: Vec<ConstructId>,
    ) -> (ConstructId, VariableId) {
        let id = self.push_variable(Variable {
            id: None,
            construct: None,
            invariants: vec![],
            dependencies: deps,
        });
        let con = CVariable::new(id);
        let cid = self.push_construct(con.clone(), Box::new(SRoot));
        (cid, id)
    }
}
