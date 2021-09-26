use super::{
    context::Context,
    helpers::{convert_defs, convert_iid, convert_iids, convert_reps, full_convert_iid},
};
use crate::shared::{
    Definitions, IntegerMathOperation, Item, ItemId, PrimitiveOperation, PrimitiveType,
    PrimitiveValue, Replacements,
};

pub fn convert_shared_item(ctx: &mut Context, item: &Item) -> Result<Item, String> {
    match item {
        Item::Defining { base, definitions } => convert_defining(ctx, *base, definitions),
        Item::FromType { base, vars } => convert_from_type(ctx, *base, vars),
        Item::GodType => Ok(Item::GodType),
        Item::InductiveValue {
            records,
            typee,
            variant_id,
        } => convert_inductive_value(ctx, records, *typee, *variant_id),
        Item::IsSameVariant { base, other } => convert_is_same_variant(ctx, *base, *other),
        Item::Pick {
            initial_clause,
            elif_clauses,
            else_clause,
        } => convert_pick(ctx, *initial_clause, elif_clauses, *else_clause),
        Item::PrimitiveOperation(op) => convert_primitive_operation(ctx, op),
        Item::PrimitiveType(pt) => convert_primitive_type(*pt),
        Item::PrimitiveValue(pv) => convert_primitive_value(*pv),
        Item::Replacing {
            base,
            replacements,
            unlabeled_replacements,
        } => convert_replacing(ctx, *base, replacements, unlabeled_replacements),
        Item::TypeIs { exact, base, typee } => convert_type_is(ctx, *exact, *base, *typee),
        Item::Variable { selff, typee } => convert_variable(ctx, *selff, *typee),
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
        vars: convert_iids(ctx, vars)?,
    })
}

fn convert_inductive_value(
    ctx: &mut Context,
    records: &Vec<ItemId>,
    typee: ItemId,
    variant_id: ItemId,
) -> Result<Item, String> {
    Ok(Item::InductiveValue {
        records: convert_iids(ctx, records)?,
        typee: full_convert_iid(ctx, typee)?,
        variant_id: convert_iid(ctx, variant_id, false)?,
    })
}

fn convert_is_same_variant(ctx: &mut Context, base: ItemId, other: ItemId) -> Result<Item, String> {
    Ok(Item::IsSameVariant {
        base: convert_iid(ctx, base, true)?,
        other: convert_iid(ctx, other, true)?,
    })
}

fn convert_pick(
    ctx: &mut Context,
    initial_clause: (ItemId, ItemId),
    elif_clauses: &Vec<(ItemId, ItemId)>,
    else_clause: ItemId,
) -> Result<Item, String> {
    Ok(Item::Pick {
        initial_clause: (
            convert_iid(ctx, initial_clause.0, true)?,
            convert_iid(ctx, initial_clause.1, true)?,
        ),
        // The type of a replacement is coincidentally the same as the
        // type of a condition, and it does the exact thing we want.
        elif_clauses: convert_reps(ctx, elif_clauses)?,
        else_clause: convert_iid(ctx, else_clause, true)?,
    })
}

fn convert_primitive_operation(ctx: &mut Context, op: &PrimitiveOperation) -> Result<Item, String> {
    Ok(match op {
        PrimitiveOperation::I32Math(op) => Item::PrimitiveOperation(PrimitiveOperation::I32Math(
            convert_integer_op(ctx, op.clone())?,
        )),
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

fn convert_primitive_type(pt: PrimitiveType) -> Result<Item, String> {
    Ok(Item::PrimitiveType(pt))
}

fn convert_primitive_value(pv: PrimitiveValue) -> Result<Item, String> {
    Ok(Item::PrimitiveValue(pv))
}

fn convert_replacing(
    ctx: &mut Context,
    base: ItemId,
    replacements: &Replacements,
    unlabeled_replacements: &Vec<ItemId>,
) -> Result<Item, String> {
    Ok(Item::Replacing {
        base: full_convert_iid(ctx, base)?,
        replacements: convert_reps(ctx, replacements)?,
        unlabeled_replacements: convert_iids(ctx, unlabeled_replacements)?,
    })
}

fn convert_type_is(
    ctx: &mut Context,
    exact: bool,
    base: ItemId,
    typee: ItemId,
) -> Result<Item, String> {
    Ok(Item::TypeIs {
        exact,
        base: full_convert_iid(ctx, base)?,
        typee: full_convert_iid(ctx, typee)?,
    })
}

fn convert_variable(ctx: &mut Context, selff: ItemId, typee: ItemId) -> Result<Item, String> {
    Ok(Item::Variable {
        selff: full_convert_iid(ctx, selff)?,
        typee: full_convert_iid(ctx, typee)?,
    })
}
