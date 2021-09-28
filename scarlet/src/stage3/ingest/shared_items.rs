use super::{
    context::Context,
    helpers::{
        convert_clauses, convert_defs, convert_iid, convert_iids, convert_reps, full_convert_iid,
    },
};
use crate::shared::{
    BuiltinOperation, BuiltinValue, ConditionalClause, Definitions, IntegerMathOperation, Item,
    ItemId, Replacements,
};

pub fn convert_shared_item(ctx: &mut Context, item: &Item) -> Result<Item, String> {
    match item {
        Item::Any { selff, typee } => convert_any(ctx, *selff, *typee),
        Item::BuiltinOperation(op) => convert_builtin_operation(ctx, op),
        Item::BuiltinValue(pv) => convert_builtin_value(*pv),
        Item::Defining { base, definitions } => convert_defining(ctx, *base, definitions),
        Item::FromType { base, values: vars } => convert_from_type(ctx, *base, vars),
        Item::Pick { clauses, default } => convert_pick(ctx, clauses, *default),
        Item::Replacing { base, replacements } => convert_replacing(ctx, *base, replacements),
        Item::TypeIs {
            base_type_only,
            base,
            typee,
        } => convert_type_is(ctx, *base_type_only, *base, *typee),
        Item::Variant { selff, typee } => convert_variant(ctx, *selff, *typee),
    }
}

fn convert_defining(
    ctx: &mut Context,
    base: ItemId,
    definitions: &Definitions,
) -> Result<Item, String> {
    Ok(Item::Defining {
        base: full_convert_iid(ctx, base)?,
        definitions: convert_defs(ctx, definitions)?,
    })
}

fn convert_from_type(ctx: &mut Context, base: ItemId, vars: &Vec<ItemId>) -> Result<Item, String> {
    Ok(Item::FromType {
        base: full_convert_iid(ctx, base)?,
        values: convert_iids(ctx, vars)?,
    })
}

fn convert_variant(ctx: &mut Context, selff: ItemId, typee: ItemId) -> Result<Item, String> {
    Ok(Item::Variant {
        selff: full_convert_iid(ctx, selff)?,
        typee: full_convert_iid(ctx, typee)?,
    })
}

fn convert_pick(
    ctx: &mut Context,
    clauses: &Vec<ConditionalClause>,
    default: ItemId,
) -> Result<Item, String> {
    Ok(Item::Pick {
        clauses: convert_clauses(ctx, clauses)?,
        default: full_convert_iid(ctx, default)?,
    })
}

fn convert_builtin_operation(ctx: &mut Context, op: &BuiltinOperation) -> Result<Item, String> {
    Ok(match op {
        BuiltinOperation::I32Math(op) => Item::BuiltinOperation(BuiltinOperation::I32Math(
            convert_integer_op(ctx, op.clone())?,
        )),
        BuiltinOperation::AreSameVariant { base, other } => {
            Item::BuiltinOperation(BuiltinOperation::AreSameVariant {
                base: full_convert_iid(ctx, *base)?,
                other: full_convert_iid(ctx, *other)?,
            })
        }
        BuiltinOperation::Reinterpret {
            proof_equal,
            original_type,
            new_type,
            original,
        } => Item::BuiltinOperation(BuiltinOperation::Reinterpret {
            proof_equal: full_convert_iid(ctx, *proof_equal)?,
            original_type: full_convert_iid(ctx, *original_type)?,
            new_type: full_convert_iid(ctx, *new_type)?,
            original: full_convert_iid(ctx, *original)?,
        }),
    })
}

fn convert_integer_op(
    ctx: &mut Context,
    op: IntegerMathOperation,
) -> Result<IntegerMathOperation, String> {
    use IntegerMathOperation as Imo;
    Ok(match op {
        Imo::Sum(l, r) => Imo::Sum(full_convert_iid(ctx, l)?, full_convert_iid(ctx, r)?),
        Imo::Difference(l, r) => {
            Imo::Difference(full_convert_iid(ctx, l)?, full_convert_iid(ctx, r)?)
        }
    })
}

fn convert_builtin_value(pv: BuiltinValue) -> Result<Item, String> {
    Ok(Item::BuiltinValue(pv))
}

fn convert_replacing(
    ctx: &mut Context,
    base: ItemId,
    replacements: &Replacements,
) -> Result<Item, String> {
    Ok(Item::Replacing {
        base: full_convert_iid(ctx, base)?,
        replacements: convert_reps(ctx, replacements)?,
    })
}

fn convert_type_is(
    ctx: &mut Context,
    base_type_only: bool,
    base: ItemId,
    typee: ItemId,
) -> Result<Item, String> {
    Ok(Item::TypeIs {
        base_type_only,
        base: full_convert_iid(ctx, base)?,
        typee: full_convert_iid(ctx, typee)?,
    })
}

fn convert_any(ctx: &mut Context, selff: ItemId, typee: ItemId) -> Result<Item, String> {
    Ok(Item::Any {
        selff: full_convert_iid(ctx, selff)?,
        typee: full_convert_iid(ctx, typee)?,
    })
}
