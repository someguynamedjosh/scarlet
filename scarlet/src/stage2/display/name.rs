use crate::stage2::structure::{Environment, ItemId, ScopeId, Value};

#[derive(Clone)]
enum ChildOf {
    Definition { scope: ScopeId, name: String },
    Base(ScopeId),
}

impl ChildOf {
    pub fn parent_id(&self) -> ScopeId {
        match self {
            Self::Definition { scope, .. } => *scope,
            Self::Base(id) => *id,
        }
    }
}

impl Environment {
    pub(super) fn item_name(&self, id: ItemId, in_scope: ScopeId) -> Option<String> {
        if let Ok(Some(name)) = self.get_item_name_impl(id, in_scope, vec![]) {
            return Some(name);
        }
        None
    }

    fn get_parents(&self, of: ItemId) -> Vec<ChildOf> {
        let mut parents = Vec::new();
        for (_, def) in self.items.iter() {
            match def.value.as_ref().expect("ICE: Undefined value") {
                Value::Defining {
                    base,
                    definitions,
                    this_scope,
                } => {
                    if *base == of {
                        parents.push(ChildOf::Base(*this_scope))
                    }
                    for (name, def) in definitions {
                        if *def == of {
                            parents.push(ChildOf::Definition {
                                name: name.clone(),
                                scope: *this_scope,
                            });
                        }
                    }
                }
                _ => (),
            }
        }
        parents
    }

    fn a_is_b_or_parent_of_b(&self, a: ItemId, b: ItemId, already_checked: Vec<ItemId>) -> bool {
        if already_checked.contains(&b) {
            // Prevent infinite loops
            false
        } else if a == b {
            // If item is scope, return true.
            true
        } else {
            // Otherwise, if a parent of item or any of their parents matches the scope,
            // return true.
            for b_as_child in self.get_parents(b) {
                let b_parent = b_as_child.parent_id();
                let new_already_checked = [already_checked.clone(), vec![b]].concat();
                if self.a_is_b_or_parent_of_b(a, self[b_parent].definition, new_already_checked) {
                    return true;
                }
            }
            false
        }
    }

    fn get_item_name_impl(
        &self,
        id: ItemId,
        in_scope: ScopeId,
        already_checked: Vec<ItemId>,
    ) -> Result<Option<String>, ()> {
        let scope_def = self[in_scope].definition;
        if already_checked.contains(&id) {
            // Prevent infinite loops.
            Err(())
        } else if self.a_is_b_or_parent_of_b(id, scope_def, vec![]) {
            // If we are trying to name something which is a parent of the scope from which
            // the name should be resolved, that's an item with no name. I.E. any children
            // can be referred to by name without prefixing it with anything extra.
            Ok(None)
        } else {
            let mut candidates = Vec::new();
            for id_as_child in self.get_parents(id) {
                let parent_id = id_as_child.parent_id();
                let parent_def = self[parent_id].definition;
                let new_already_checked = [already_checked.clone(), vec![id]].concat();
                let parent_name =
                    self.get_item_name_impl(parent_def, in_scope, new_already_checked);
                match parent_name {
                    Ok(None) => match id_as_child {
                        ChildOf::Base(..) => return Ok(None),
                        ChildOf::Definition { name, .. } => candidates.push(name),
                    },
                    Ok(Some(parent_name)) => match id_as_child {
                        ChildOf::Base(..) => candidates.push(parent_name),
                        ChildOf::Definition { name, .. } => {
                            candidates.push(format!("{}::{}", parent_name, name))
                        }
                    },
                    Err(..) => (),
                }
            }
            let result = candidates.into_iter().min_by_key(|p| p.len());
            if result.is_none() {
                Err(())
            } else {
                Ok(result)
            }
        }
    }
}
