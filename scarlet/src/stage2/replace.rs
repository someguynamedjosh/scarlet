use std::collections::HashMap;

use super::structure::{Substitutions, UnresolvedSubstitution};
use crate::stage2::structure::{Condition, Definition, ItemId, StructField};

pub type Reps<'x> = HashMap<ItemId<'x>, ItemId<'x>>;

pub fn apply_reps<'x>(reps: &Reps<'x>, to: &mut ItemId<'x>) {
    if let Some(&replaced_with) = reps.get(&*to) {
        *to = replaced_with;
    }
}

pub fn apply_reps_to_def<'x>(reps: &Reps<'x>, to: &mut Definition<'x>) {
    match to {
        Definition::BuiltinOperation(_, args) => apply_reps_to_builtin_op(args, reps),
        Definition::BuiltinValue(..) => (),
        Definition::Match {
            base,
            conditions,
            else_value,
        } => apply_reps_to_match(reps, base, conditions, else_value),
        Definition::Member(base, ..) => apply_reps(reps, base),
        Definition::Other(item) => (),
        Definition::ResolvedSubstitute(base, subs) => {
            apply_reps_to_resolved_substitution(reps, base, subs)
        }
        Definition::SetEager { .. } => todo!(),
        Definition::Struct(fields) => apply_reps_to_struct(fields, reps),
        Definition::UnresolvedSubstitute(base, subs) => {
            apply_reps_to_unresolved_substitution(reps, base, subs)
        }
        Definition::Variable { .. } => (),
    }
}

fn apply_reps_to_unresolved_substitution<'x>(
    reps: &Reps<'x>,
    base: &mut ItemId<'x>,
    subs: &mut Vec<UnresolvedSubstitution<'x>>,
) {
    apply_reps(reps, base);
    for sub in subs {
        if let Some(item) = &mut sub.target_meaning {
            apply_reps(reps, item);
        }
        apply_reps(reps, &mut sub.value);
    }
}

fn apply_reps_to_resolved_substitution<'x>(
    reps: &Reps<'x>,
    base: &mut ItemId<'x>,
    subs: &mut Substitutions<'x>,
) {
    apply_reps(reps, base);
    for (_, value) in subs {
        apply_reps(reps, value);
    }
}

fn apply_reps_to_struct<'x>(fields: &mut Vec<StructField<'x>>, reps: &Reps<'x>) {
    for field in fields {
        apply_reps(reps, &mut field.value);
    }
}

fn apply_reps_to_match<'x>(
    reps: &Reps<'x>,
    base: &mut ItemId<'x>,
    conditions: &mut Vec<Condition<'x>>,
    else_value: &mut ItemId<'x>,
) {
    apply_reps(reps, base);
    for cond in conditions {
        apply_reps(reps, &mut cond.pattern);
        apply_reps(reps, &mut cond.value);
    }
    apply_reps(reps, else_value);
}

fn apply_reps_to_builtin_op<'x>(args: &mut Vec<ItemId<'x>>, reps: &Reps<'x>) {
    for arg in args {
        apply_reps(reps, arg)
    }
}
