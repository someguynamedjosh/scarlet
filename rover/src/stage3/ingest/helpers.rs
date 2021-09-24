use super::{
    context::Context,
    dereference::{convert_dereffed, dereference_iid},
};
use crate::shared::{Definitions, ItemId, Replacements};

pub fn convert_defs(ctx: &mut Context, defs: &[(String, ItemId)]) -> Result<Definitions, String> {
    let mut result = Vec::new();
    for (name, def) in defs {
        // Don't dereference defines so we can preserve module structure for
        // when we go backwards from IDs to names.
        result.push((name.clone(), convert_iid(ctx, *def, false)?));
    }
    Ok(result)
}

pub fn convert_reps(ctx: &mut Context, reps: &[(ItemId, ItemId)]) -> Result<Replacements, String> {
    let mut result = Vec::new();
    for (target, val) in reps {
        result.push((
            full_convert_iid(ctx, *target)?,
            full_convert_iid(ctx, *val)?,
        ));
    }
    Ok(result)
}

pub fn convert_iids(ctx: &mut Context, ids: &[ItemId]) -> Result<Vec<ItemId>, String> {
    let mut result = Vec::new();
    for id in ids {
        result.push(full_convert_iid(ctx, *id)?);
    }
    Ok(result)
}

pub fn full_convert_iid(ctx: &mut Context, id: ItemId) -> Result<ItemId, String> {
    convert_iid(ctx, id, true)
}

/// Applies dereferencing and id_map to the provided item id.
pub fn convert_iid(ctx: &mut Context, id: ItemId, deref_define: bool) -> Result<ItemId, String> {
    let dereffed = dereference_iid(ctx, id, deref_define)?;
    Ok(convert_dereffed(ctx, dereffed)?.0)
}

pub fn convert_defined_in(ctx: &mut Context, defined_in: Option<ItemId>) -> Result<Option<ItemId>, String> {
    if let Some(id) = defined_in {
        convert_iid(ctx, id, false).map(Some)
    } else {
        Ok(None)
    }
}
