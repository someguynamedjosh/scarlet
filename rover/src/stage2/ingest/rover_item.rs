use crate::{
    shared::{
        IntegerMathOperation, Item, ItemId, PrimitiveOperation, PrimitiveType, PrimitiveValue,
    },
    stage2::structure::Environment,
};

fn define_two_vars(env: &mut Environment, typee: ItemId) -> (ItemId, ItemId) {
    (env.insert_variable(typee), env.insert_variable(typee))
}

fn define_binary_op(
    env: &mut Environment,
    typee: ItemId,
    op: impl FnOnce(ItemId, ItemId) -> PrimitiveOperation,
) -> ItemId {
    let (a, b) = define_two_vars(env, typee);
    let base = env.insert(Item::PrimitiveOperation(op(a, b)).into());
    let item = Item::defining(base, vec![("left", a), ("right", b)]);
    env.insert(item.into())
}

fn define_integer_type(
    env: &mut Environment,
    typee: PrimitiveType,
    op_builder: impl Fn(IntegerMathOperation) -> PrimitiveOperation,
) -> ItemId {
    use IntegerMathOperation as Imo;
    let itype_base = env.insert(Item::PrimitiveType(typee).into());
    let sum = define_binary_op(env, itype_base, |a, b| op_builder(Imo::Sum(a, b)));
    let difference = define_binary_op(env, itype_base, |a, b| op_builder(Imo::Difference(a, b)));
    let members = vec![("sum", sum), ("difference", difference)];
    env.insert_self_referencing_define(itype_base, members)
}

fn define_bool_type(env: &mut Environment) -> ItemId {
    let bool_type_base = env.insert(Item::PrimitiveType(PrimitiveType::Bool).into());
    let true_con = env.insert(Item::PrimitiveValue(PrimitiveValue::Bool(true)).into());
    let false_con = env.insert(Item::PrimitiveValue(PrimitiveValue::Bool(false)).into());
    let members = vec![("true", true_con), ("false", false_con)];
    env.insert_self_referencing_define(bool_type_base, members)
}

fn define_lang_members(env: &mut Environment, god_type: ItemId) -> Vec<(&'static str, ItemId)> {
    let i32_type = define_integer_type(env, PrimitiveType::I32, PrimitiveOperation::I32Math);
    let bool_type = define_bool_type(env);

    vec![
        ("TYPE", god_type),
        ("Integer32", i32_type),
        ("Boolean", bool_type),
    ]
}

fn define_lang_item(env: &mut Environment) -> (ItemId, ItemId) {
    let god_type = env.insert(Item::GodType.into());

    let lang_members = define_lang_members(env, god_type);
    let lang_item = env.insert(Item::defining(god_type, lang_members).into());
    env.mark_as_module(lang_item);

    (god_type, lang_item)
}

pub fn define_rover_item(env: &mut Environment) -> (ItemId, ItemId) {
    let (god_type, lang) = define_lang_item(env);
    let rover_members = vec![("lang", lang)];
    let rover = env.insert(Item::defining(god_type, rover_members).into());
    env.mark_as_module(rover);
    (rover, god_type)
}
