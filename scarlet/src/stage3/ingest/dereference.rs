use super::{
    context::Context,
    dereferenced_item::DereferencedItem,
    get_member::get_member,
    helpers::{convert_defined_in, convert_iids, convert_reps},
};
use crate::{
    shared::{Item, ItemId, Replacements},
    stage2::structure::{self as stage2},
};

fn convert_dereffed_replacing(
    ctx: &mut Context,
    base: DereferencedItem,
    replacements: Replacements,
) -> Result<(ItemId, Option<ItemId>), String> {
    let (base, defined_in) = convert_dereffed(ctx, base)?;
    let id = ctx.insert_extra_item(Item::Replacing { base, replacements }, defined_in);
    Ok((id, defined_in))
}

pub fn convert_dereffed(
    ctx: &mut Context,
    item: DereferencedItem,
) -> Result<(ItemId, Option<ItemId>), String> {
    match item {
        DereferencedItem::Stage2Item(id) => {
            let old_defined_in = ctx.src.items[id.0].defined_in;
            let new_defined_in = convert_defined_in(ctx, old_defined_in)?;
            let new_id = *ctx.id_map.get(&id).unwrap();
            Ok((new_id, new_defined_in))
        }
        DereferencedItem::Replacing { base, replacements } => {
            convert_dereffed_replacing(ctx, *base, replacements)
        }
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
) -> Result<DereferencedItem, String> {
    let deref_base = dereference_iid(ctx, base, deref_define)?;
    let replacements = convert_reps(ctx, replacements)?;
    Ok(DereferencedItem::Replacing {
        base: Box::new(deref_base),
        replacements,
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
        stage2::UnresolvedItem::Just(Item::Replacing { base, replacements }) => {
            dereference_replacing(ctx, deref_define, *base, replacements)
        }
        _ => Ok(DereferencedItem::Stage2Item(id)),
    }
}
