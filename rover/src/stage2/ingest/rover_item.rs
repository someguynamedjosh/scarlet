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
    let base = env.insert_item(Item::PrimitiveOperation(op(a, b)));
    let item = Item::defining(base, vec![("left", a), ("right", b)]);
    let item_id = env.insert_scope(item);
    env.set_defined_in(a, item_id);
    env.set_defined_in(b, item_id);
    env.set_defined_in(base, item_id);
    item_id
}

fn define_integer_type(
    env: &mut Environment,
    typee: PrimitiveType,
    op_builder: impl Fn(IntegerMathOperation) -> PrimitiveOperation,
) -> ItemId {
    use IntegerMathOperation as Imo;
    let itype_base = env.insert_item(Item::PrimitiveType(typee));
    let sum = define_binary_op(env, itype_base, |a, b| op_builder(Imo::Sum(a, b)));
    let difference = define_binary_op(env, itype_base, |a, b| op_builder(Imo::Difference(a, b)));
    let members = vec![("sum", sum), ("difference", difference)];
    let item_id = env.insert_self_referencing_define(itype_base, members);
    env.set_defined_in(itype_base, item_id);
    env.set_defined_in(sum, item_id);
    env.set_defined_in(difference, item_id);
    item_id
}

fn define_bool_type(env: &mut Environment) -> ItemId {
    let bool_type_base = env.insert_item(Item::PrimitiveType(PrimitiveType::Bool));
    let true_con = env.insert_item(Item::PrimitiveValue(PrimitiveValue::Bool(true)));
    let false_con = env.insert_item(Item::PrimitiveValue(PrimitiveValue::Bool(false)));
    let members = vec![("true", true_con), ("false", false_con)];
    let item_id = env.insert_self_referencing_define(bool_type_base, members);
    env.set_defined_in(bool_type_base, item_id);
    env.set_defined_in(true_con, item_id);
    env.set_defined_in(false_con, item_id);
    item_id
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
    let god_type = env.insert_item(Item::GodType);

    let lang_members = define_lang_members(env, god_type);
    let lang_item = env.insert_scope(Item::defining(god_type, lang_members.clone()));
    for (_, item) in &lang_members {
        env.set_defined_in(*item, lang_item);
    }

    (god_type, lang_item)
}

pub fn define_rover_item(env: &mut Environment) -> (ItemId, ItemId) {
    let (god_type, lang) = define_lang_item(env);
    let rover_members = vec![("lang", lang)];
    let rover = env.insert_scope(Item::defining(god_type, rover_members));
    env.set_defined_in(lang, rover);
    (rover, god_type)
}
