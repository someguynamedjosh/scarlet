use super::{
    dependencies::{DepResult, Dependencies},
    Environment,
};
use crate::{
    item::{variable::VariableId, ItemPtr},
    shared::OrderedMap,
    util::Isomorphism,
};

pub type NestedSubstitutions<'a> = OrderedMap<VariableId, SubExpr<'a>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SubExpr<'a>(pub ItemPtr, pub &'a NestedSubstitutions<'a>);

impl<'a> SubExpr<'a> {
    pub fn deps(&self, env: &mut Environment) -> DepResult {
        let mut result = Dependencies::new();
        let base = env.get_dependencies(self.0);
        for (target, value) in self.1.iter() {
            if base.contains_var(*target) {
                result.append(value.deps(env));
            }
        }
        result
    }

    pub fn to_owned(&self) -> OwnedSubExpr {
        OwnedSubExpr(
            self.0,
            self.1
                .iter()
                .map(|(key, value)| (*key, value.to_owned()))
                .collect(),
        )
    }
}

impl<'a> Isomorphism<OwnedSubExpr> for SubExpr<'a> {
    fn convert(self) -> OwnedSubExpr {
        self.to_owned()
    }

    fn equals(&self, other: &OwnedSubExpr) -> bool {
        for (sel, oth) in self.1.iter().zip(other.1.iter()) {
            if sel.0 != oth.0 || !sel.1.equals(&oth.1) {
                return false;
            }
        }
        self.0 == other.0 && self.1.len() == other.1.len()
    }
}

pub type OwnedNestedSubstitutions = OrderedMap<VariableId, OwnedSubExpr>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OwnedSubExpr(pub ItemPtr, pub OwnedNestedSubstitutions);
