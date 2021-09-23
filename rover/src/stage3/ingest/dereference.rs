use super::{
    context::Context,
    dereferenced_item::DereferencedItem,
    helpers::{convert_iids, convert_reps},
};
use crate::{
    shared::{Item, ItemId},
    stage2::structure::{self as stage2},
};

pub fn convert_dereffed(ctx: &mut Context, item: DereferencedItem) -> ItemId {
    match item {
        DereferencedItem::Stage2Item(id) => *ctx.id_map.get(&id).unwrap(),
        DereferencedItem::Replacing {
            base,
            replacements,
            unlabeled_replacements,
        } => {
            let base = convert_dereffed(ctx, *base);
            ctx.insert_extra_item(Item::Replacing {
                base,
                replacements,
                unlabeled_replacements,
            })
        }
    }
}

fn get_member(
    ctx: &mut Context,
    base: ItemId,
    name: &String,
    deref_define: bool,
) -> Result<DereferencedItem, String> {
    let og_base = dereference_iid(ctx, base, false)?;
    match ctx.src.definition_of(og_base.id()).as_ref().unwrap() {
        stage2::UnresolvedItem::Just(Item::Defining {
            base: def_base,
            definitions,
        }) => {
            if let Ok(member) = get_member(ctx, *def_base, name, deref_define) {
                return Ok(member);
            }
            for (cname, cdef) in definitions {
                if cname == name {
                    let dereffed_definition = dereference_iid(ctx, *cdef, deref_define)?;
                    return Ok(og_base.with_base(dereffed_definition));
                }
            }
            Err(format!("{:?} has no member named {}", og_base, name))
        }
        stage2::UnresolvedItem::Item(..) | stage2::UnresolvedItem::Member { .. } => {
            unreachable!()
        }
        stage2::UnresolvedItem::Just(Item::Replacing { .. }) => {
            unreachable!("{:?} {:?}", og_base, base)
        }
        _ => Err(format!("{:?} has no members", og_base)),
    }
}

/// Returns the target of the item if it is a reference to another item or
/// member.
pub fn dereference_iid(
    ctx: &mut Context,
    id: ItemId,
    deref_define: bool,
) -> Result<DereferencedItem, String> {
    match ctx.src.definition_of(id).as_ref().unwrap() {
        stage2::UnresolvedItem::Just(Item::Defining { base, .. }) => {
            if deref_define {
                dereference_iid(ctx, *base, true)
            } else {
                Ok(DereferencedItem::Stage2Item(id))
            }
        }
        stage2::UnresolvedItem::Item(id) => dereference_iid(ctx, *id, deref_define),
        stage2::UnresolvedItem::Member { base, name } => get_member(ctx, *base, name, deref_define),
        stage2::UnresolvedItem::Just(Item::Replacing {
            base,
            replacements,
            unlabeled_replacements,
        }) => {
            let deref_base = dereference_iid(ctx, *base, deref_define)?;
            let replacements = convert_reps(ctx, replacements)?;
            let unlabeled_replacements = convert_iids(ctx, unlabeled_replacements)?;
            Ok(DereferencedItem::Replacing {
                base: Box::new(deref_base),
                replacements,
                unlabeled_replacements,
            })
        }
        _ => Ok(DereferencedItem::Stage2Item(id)),
    }
}
