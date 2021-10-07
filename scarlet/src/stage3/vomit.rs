use crate::{stage2::structure as s2, stage3::structure as s3};

#[derive(Clone, Debug)]
enum RelativePath {
    Just(s2::ItemId),
    DefiningBase {
        parent: Box<RelativePath>,
        result: s2::ItemId,
    },
    DefiningMember {
        parent: Box<RelativePath>,
        member: String,
        result: s2::ItemId,
    },
}

impl RelativePath {
    /// The ID you get if you follow this entire path.
    fn final_id(&self) -> s2::ItemId {
        match self {
            RelativePath::Just(result) => *result,
            RelativePath::DefiningBase { result, .. }
            | RelativePath::DefiningMember { result, .. } => *result,
        }
    }

    /// The ID you get if you only follow the first component of this path.
    fn topmost_id(&self) -> s2::ItemId {
        match self {
            RelativePath::Just(result) => *result,
            RelativePath::DefiningBase { parent, .. }
            | RelativePath::DefiningMember { parent, .. } => parent.topmost_id(),
        }
    }

    /// Returns the IDs you get starting with the full path's ID, then the ID of
    /// its parent, then the ID of that ID's parent, and so on.
    fn flatten(&self) -> Vec<s2::ItemId> {
        match self {
            RelativePath::Just(result) => vec![*result],
            RelativePath::DefiningBase { parent, result }
            | RelativePath::DefiningMember { parent, result, .. } => {
                [vec![*result], parent.flatten()].concat()
            }
        }
    }

    /// Returns a copy of this path with a number of parent components trimmed
    /// away such that only $desired_num_components remain.
    fn slice(self, max_index: usize) -> Self {
        match self {
            RelativePath::Just(_) => {
                assert_eq!(max_index, 0);
                self
            }
            RelativePath::DefiningBase { parent, result } => {
                if max_index == 0 {
                    Self::Just(result)
                } else {
                    let parent = Box::new(parent.slice(max_index - 1));
                    Self::DefiningBase { parent, result }
                }
            }
            RelativePath::DefiningMember {
                parent,
                member,
                result,
            } => {
                if max_index == 0 {
                    Self::Just(result)
                } else {
                    Self::DefiningMember {
                        parent: Box::new(parent.slice(max_index - 1)),
                        member,
                        result,
                    }
                }
            }
        }
    }
}

fn get_full_path(original_s2: &s2::Environment, item: s2::ItemId) -> RelativePath {
    for (id, maybe_parent) in &original_s2.items {
        match maybe_parent {
            s2::Item::Defining { base, definitions } => {
                let result = item;
                if *base == item {
                    let parent = Box::new(get_full_path(original_s2, id));
                    return RelativePath::DefiningBase { parent, result };
                }
                for (member_name, member) in definitions {
                    if *member == item {
                        let parent = Box::new(get_full_path(original_s2, id));
                        return RelativePath::DefiningMember {
                            member: member_name.clone(),
                            parent,
                            result,
                        };
                    }
                }
            }
            _ => (),
        }
    }
    RelativePath::Just(item)
}

fn truncate_paths_to_common_ancestor(
    a: RelativePath,
    b: RelativePath,
) -> (RelativePath, RelativePath) {
    let a_parts = a.flatten();
    let b_parts = b.flatten();
    for (ai, ap) in a_parts.iter().enumerate() {
        for (bi, bp) in b_parts.iter().enumerate() {
            if ap == bp {
                return (a.slice(ai), b.slice(bi));
            }
        }
    }
    (a, b)
}

fn path_to_item(target: &mut s2::Environment, path: RelativePath) -> s2::ItemId {
    match path {
        RelativePath::Just(item) => item,
        RelativePath::DefiningBase { parent, .. } => path_to_item(target, *parent),
        RelativePath::DefiningMember { parent, member, .. } => {
            if let RelativePath::Just(..) = &*parent {
                let item = s2::Item::Identifier(member);
                target.items.get_or_push(item)
            } else {
                let base = path_to_item(target, *parent);
                let name = member;
                let item = s2::Item::Member { base, name };
                target.items.get_or_push(item)
            }
        }
    }
}

fn path_to_string(path: RelativePath, root_is: s2::ItemId) -> Option<String> {
    match path {
        RelativePath::Just(id) => {
            if id == root_is {
                Some(format!("ROOT"))
            } else {
                None
            }
        }
        RelativePath::DefiningBase { parent, .. } => path_to_string(*parent, root_is),
        RelativePath::DefiningMember { parent, member, .. } => {
            let parent = path_to_string(*parent, root_is);
            Some(if let Some(parent) = parent {
                format!("{}::{}", parent, member)
            } else {
                member
            })
        }
    }
}

fn vomit(
    env: &s3::Environment,
    value: &s3::AnnotatedValue,
    display_at: s2::ItemId,
    target: &mut s2::Environment,
    original_root: s2::ItemId,
) -> DisplayResult {
    let display_path = get_full_path(target, display_at);
    let name = path_to_string(display_path.clone(), original_root);

    let &(definition, _) = value.defined_at.iter().next().unwrap();
    let definition_path = get_full_path(target, definition);
    let (_, definition_path) = truncate_paths_to_common_ancestor(display_path, definition_path);
    let id = path_to_item(target, definition_path);
    DisplayResult {
        value_name: name.unwrap_or(format!("anonymous")),
        vomited_root: id,
    }
}

pub struct DisplayResult {
    pub value_name: String,
    pub vomited_root: s2::ItemId,
}

impl s3::Environment {
    pub fn display_all(
        &self,
        target: &mut s2::Environment,
        original_root: s2::ItemId,
    ) -> Vec<DisplayResult> {
        let mut displays = Vec::new();
        for (_, value) in &self.values {
            for (display_requested_scope, _) in &value.display_requested_from {
                let vomited = vomit(self, value, *display_requested_scope, target, original_root);
                displays.push(vomited);
            }
        }
        displays
    }
}
