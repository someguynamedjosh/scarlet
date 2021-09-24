use super::{
    context::Context,
    dereferenced_item::DereferencedItem,
    get_member::get_member,
    helpers::{convert_iids, convert_reps},
};
use crate::{
    shared::{Item, ItemId, Replacements},
    stage2::structure::{self as stage2},
};

fn convert_dereffed_replacing(
    ctx: &mut Context,
    base: DereferencedItem,
    replacements: Replacements,
    unlabeled_replacements: Vec<ItemId>,
) -> ItemId {
    let base = convert_dereffed(ctx, base);
    ctx.insert_extra_item(Item::Replacing {
        base,
        replacements,
        unlabeled_replacements,
    })
}

pub fn convert_dereffed(ctx: &mut Context, item: DereferencedItem) -> ItemId {
    match item {
        DereferencedItem::Stage2Item(id) => *ctx.id_map.get(&id).unwrap(),
        DereferencedItem::Replacing {
            base,
            replacements,
            unlabeled_replacements,
        } => convert_dereffed_replacing(ctx, *base, replacements, unlabeled_replacements),
    }
}

fn dereference_define(
    ctx: &mut Context,
    deref_define: bool,
    define_id: ItemId,
    define_base: ItemId,
) -> Result<DereferencedItem, String> {
    if deref_define {
        dereference_iid(ctx, define_base, true)
    } else {
        Ok(DereferencedItem::Stage2Item(define_id))
    }
}

fn dereference_replacing(
    ctx: &mut Context,
    deref_define: bool,
    base: ItemId,
    replacements: &Replacements,
    unlabeled_replacements: &Vec<ItemId>,
) -> Result<DereferencedItem, String> {
    let deref_base = dereference_iid(ctx, base, deref_define)?;
    let replacements = convert_reps(ctx, replacements)?;
    let unlabeled_replacements = convert_iids(ctx, unlabeled_replacements)?;
    Ok(DereferencedItem::Replacing {
        base: Box::new(deref_base),
        replacements,
        unlabeled_replacements,
    })
}

/// Returns the target of the item if it is a reference to another item or
/// member.
pub fn dereference_iid(
    ctx: &mut Context,
    id: ItemId,
    deref_define: bool,
) -> Result<DereferencedItem, String> {
    match ctx.src.definition_of(id).definition.as_ref().unwrap() {
        stage2::UnresolvedItem::Just(Item::Defining { base, .. }) => {
            dereference_define(ctx, deref_define, id, *base)
        }
        stage2::UnresolvedItem::Item(id) => dereference_iid(ctx, *id, deref_define),
        stage2::UnresolvedItem::Member { base, name } => get_member(ctx, *base, name, deref_define),
        stage2::UnresolvedItem::Just(Item::Replacing {
            base,
            replacements,
            unlabeled_replacements,
        }) => dereference_replacing(
            ctx,
            deref_define,
            *base,
            replacements,
            unlabeled_replacements,
        ),
        _ => Ok(DereferencedItem::Stage2Item(id)),
    }
}
