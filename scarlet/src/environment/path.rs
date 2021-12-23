use super::{overlay::Overlay, Environment};
use crate::{
    constructs::{as_struct, ConstructDefinition, ConstructId},
    scope::Scope,
    shared::OwnedOrBorrowed,
};

#[derive(Clone, Debug)]
pub enum PathParentType {
    StructValue,
    StructRest,
}

#[derive(Clone, Debug)]
pub struct PathParent {
    typee: PathParentType,
    parent: ConstructId,
}

#[derive(Clone, Debug)]
pub struct Path {
    pub ident: String,
    pub access: Vec<PathParentType>,
}

#[derive(Debug)]
pub struct PathOverlay<'e, 'x> {
    data: Overlay<'e, 'x, Vec<PathParent>>,
}

impl<'e, 'x> PathOverlay<'e, 'x> {
    pub fn new(env: &'e mut Environment<'x>) -> Self {
        let mut this = Self {
            data: Overlay::new(env),
        };
        let mut next_id = this.data.env().constructs.first();
        while let Some(id) = next_id {
            this.label_children(id);
            next_id = this.data.env().constructs.next(id);
        }
        this
    }

    fn label_children(&mut self, of: ConstructId) {
        let con = self.data.env().get_construct(of);
        if let ConstructDefinition::Resolved(def) = &con.definition {
            if let Some(structt) = as_struct(&**def) {
                let value = structt.get_value();
                let rest = structt.get_rest();
                self.data.get_mut(value).push(PathParent {
                    typee: PathParentType::StructValue,
                    parent: of,
                });
                self.data.get_mut(rest).push(PathParent {
                    typee: PathParentType::StructRest,
                    parent: of,
                });
            }
        }
    }

    fn get_paths(&mut self, of: ConstructId, from: &dyn Scope) -> Vec<Path> {
        if let Some(ident) = from.reverse_lookup_ident(self.data.env_mut(), of) {
            vec![Path {
                ident,
                access: vec![],
            }]
        } else {
            let mut result = Vec::new();
            for parent in self.data.get_mut(of).clone() {
                let parent_paths = self.get_paths(parent.parent, from);
                result.extend(parent_paths.into_iter().map(|mut p| {
                    p.access.push(parent.typee.clone());
                    p
                }));
            }
            result
        }
    }

    pub fn get_path(&mut self, of: ConstructId, from: &dyn Scope) -> Option<Path> {
        self.get_paths(of, from)
            .into_iter()
            .min_by_key(|p| p.access.len())
    }
}
