use super::{context::Context, dereferenced_item::DereferencedItem};
use crate::{
    shared::{Definitions, Item, ItemId},
    stage2::structure::{self as stage2, UnresolvedItem},
    stage3::ingest::dereference::dereference_iid,
};

fn definition_of_dereferenced_item<'a>(
    ctx: &mut Context<'a>,
    item: &DereferencedItem,
) -> &'a UnresolvedItem {
    ctx.src.definition_of(item.id()).as_ref().unwrap()
}

fn get_member_in_defining(
    ctx: &mut Context,
    def_item: &DereferencedItem,
    def_base: ItemId,
    definitions: &Definitions,
    expected_name: &String,
    deref_define: bool,
) -> Result<DereferencedItem, String> {
    if let Ok(member) = get_member(ctx, def_base, expected_name, deref_define) {
        return Ok(member);
    }
    for (cname, cdef) in definitions {
        if cname == expected_name {
            let dereffed_definition = dereference_iid(ctx, *cdef, deref_define)?;
            return Ok(def_item.with_base(dereffed_definition));
        }
    }
    Err(format!(
        "{:?} has no member named {}",
        def_item, expected_name
    ))
}

pub fn get_member(
    ctx: &mut Context,
    base: ItemId,
    name: &String,
    deref_define: bool,
) -> Result<DereferencedItem, String> {
    let og_base = dereference_iid(ctx, base, false)?;
    match definition_of_dereferenced_item(ctx, &og_base) {
        stage2::UnresolvedItem::Just(Item::Defining {
            base: def_base,
            definitions,
        }) => get_member_in_defining(ctx, &og_base, *def_base, definitions, name, deref_define),
        stage2::UnresolvedItem::Item(..) | stage2::UnresolvedItem::Member { .. } => {
            unreachable!()
        }
        stage2::UnresolvedItem::Just(Item::Replacing { .. }) => {
            unreachable!("{:?} {:?}", og_base, base)
        }
        _ => Err(format!("{:?} has no members", og_base)),
    }
}
