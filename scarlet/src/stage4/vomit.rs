use super::structure::OpaqueId;
use crate::{
    stage2::structure::{self as s2, Item, ItemId},
    stage4::structure as s3,
};

#[derive(Clone, Debug, PartialEq, Eq)]
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

    fn visually_equals(&self, other: &Self) -> bool {
        self.visual_flatten() == other.visual_flatten()
    }

    /// Like flatten, but does not include defining base dereferences.
    fn visual_flatten(&self) -> Vec<s2::ItemId> {
        match self {
            RelativePath::Just(result) => vec![*result],
            RelativePath::DefiningBase { parent, result } => {
                let mut retval = parent.flatten();
                retval[0] = *result;
                retval
            }
            RelativePath::DefiningMember { parent, result, .. } => {
                [vec![*result], parent.flatten()].concat()
            }
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

fn get_full_path(original_s2: &s2::Environment, item_id: s2::ItemId) -> RelativePath {
    let item = &original_s2.items[item_id];
    if let Some(parent) = item.parent_scope {
        let parent_path = get_full_path(original_s2, parent);
        match &original_s2.items[parent].item {
            s2::Item::Substituting { .. } => {
                return parent_path;
            }
            s2::Item::Defining { base, definitions } => {
                let result = item_id;
                let parent = Box::new(parent_path);
                if *base == item_id {
                    return RelativePath::DefiningBase { parent, result };
                }
                for (member_name, member) in definitions {
                    if *member == item_id {
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
    RelativePath::Just(item_id)
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
                target.push_item(item)
            } else {
                let base = path_to_item(target, *parent);
                let name = member;
                let item = s2::Item::Member { base, name };
                target.push_item(item)
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

fn vomit_opaque(
    env: &s3::Environment,
    value: OpaqueId,
    target_env: &mut s2::Environment,
    display_path: &RelativePath,
) -> ItemId {
    for (_, env_value) in &env.values {
        if let &s3::Value::Opaque { id, .. } = &env_value.value {
            if id == value {
                return vomit_value(env, env_value, target_env, display_path);
            }
        }
    }
    unreachable!()
}

fn vomit_value_as_code(
    env: &s3::Environment,
    value: &s3::AnnotatedValue,
    target_env: &mut s2::Environment,
    display_path: &RelativePath,
) -> ItemId {
    match value.value.clone() {
        s3::Value::BuiltinOperation(_) => todo!(),
        s3::Value::BuiltinValue(value) => target_env.push_item(Item::BuiltinValue(value)),
        s3::Value::From { base, variable } => {
            let base = vomit_value(env, &env.values[base], target_env, display_path);
            let value = vomit_opaque(env, variable, target_env, display_path);
            target_env.push_item(Item::From { base, value })
        }
        s3::Value::Match { base, cases } => {
            let base = vomit_value(env, &env.values[base], target_env, display_path);
            let mut vom_cases = Vec::new();
            for (target, value) in cases {
                vom_cases.push((
                    vomit_value(env, &env.values[target], target_env, display_path),
                    vomit_value(env, &env.values[value], target_env, display_path),
                ))
            }
            let cases = vom_cases;
            target_env.push_item(Item::Match { base, cases })
        }
        s3::Value::Opaque {
            class,
            id: _,
            typee,
        } => {
            let id = target_env.new_opaque_value();
            let typee = vomit_value(env, &env.values[typee], target_env, display_path);
            target_env.push_item(Item::Opaque { class, id, typee })
        }
        s3::Value::Placeholder(..) => unreachable!(),
        s3::Value::Substituting {
            base,
            substitutions,
        } => {
            let base = vomit_value(env, &env.values[base], target_env, display_path);
            let mut vomited_subs = Vec::new();
            for (target, value) in substitutions {
                let target = vomit_opaque(env, target, target_env, display_path);
                let value = vomit_value(env, &env.values[value], target_env, display_path);
                vomited_subs.push((Some(target), value));
            }
            target_env.push_item(Item::Substituting {
                base,
                substitutions: vomited_subs,
            })
        }
    }
}

fn vomit_value(
    env: &s3::Environment,
    value: &s3::AnnotatedValue,
    target: &mut s2::Environment,
    display_path: &RelativePath,
) -> ItemId {
    if let Some(&(definition, _)) = value.defined_at.iter().next() {
        let definition_path = get_full_path(target, definition);
        if &definition_path == display_path {
            vomit_value_as_code(env, value, target, display_path)
        } else {
            let (_, definition_path) =
                truncate_paths_to_common_ancestor(display_path.clone(), definition_path);
            path_to_item(target, definition_path)
        }
    } else {
        vomit_value_as_code(env, value, target, display_path)
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

    let id = vomit_value(env, value, target, &display_path);
    let typee = &env.values[value.cached_type.unwrap()];
    let typee = vomit_value(env, typee, target, &display_path);
    DisplayResult {
        value_name: name.unwrap_or(format!("anonymous")),
        vomited_root: id,
        vomited_type: typee,
    }
}

pub struct DisplayResult {
    pub value_name: String,
    pub vomited_root: s2::ItemId,
    pub vomited_type: s2::ItemId,
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
