use crate::stage2::structure::{
    Environment, IntegerMathOperation, Item, ItemId, PrimitiveOperation, PrimitiveType,
};

fn define_two_vars(env: &mut Environment, typee: ItemId) -> (ItemId, ItemId) {
    let var1 = env.next_id();
    env.define(var1, Item::Variable { selff: var1, typee });
    let var2 = env.next_id();
    env.define(var2, Item::Variable { selff: var2, typee });
    (var1, var2)
}

fn define_binary_op(
    env: &mut Environment,
    typee: ItemId,
    op: impl FnOnce(ItemId, ItemId) -> PrimitiveOperation,
) -> ItemId {
    let (a, b) = define_two_vars(env, typee);
    let base = env.next_id();
    env.define(base, Item::PrimitiveOperation(op(a, b)));
    let into = env.next_id();
    env.define(
        into,
        Item::Defining {
            base,
            definitions: vec![(format!("left"), a), (format!("right"), b)],
        },
    );
    into
}

fn define_integer_type(
    env: &mut Environment,
    typee: PrimitiveType,
    op_builder: impl Fn(IntegerMathOperation) -> PrimitiveOperation,
) -> ItemId {
    use IntegerMathOperation as Imo;
    let itype_base = env.next_id();
    env.define(itype_base, Item::PrimitiveType(typee));
    let itype = env.next_id();
    let add = define_binary_op(env, itype, |a, b| op_builder(Imo::Add(a, b)));
    let subtract = define_binary_op(env, itype, |a, b| op_builder(Imo::Subtract(a, b)));
    env.define(
        itype,
        Item::Defining {
            base: itype_base,
            definitions: vec![(format!("add"), add), (format!("subtract"), subtract)],
        },
    );
    itype
}

fn define_lang_item(env: &mut Environment) -> (ItemId, ItemId) {
    let god_type = env.next_id();
    env.define(god_type, Item::GodType);
    let i32_type = define_integer_type(env, PrimitiveType::I32, |o| PrimitiveOperation::I32Math(o));

    let lang = env.next_id();
    env.mark_as_module(lang);
    env.define(
        lang,
        Item::Defining {
            base: god_type,
            definitions: vec![
                (format!("TYPE"), god_type),
                (format!("Integer32"), i32_type),
            ],
        },
    );

    (god_type, lang)
}

fn define_core_item(env: &mut Environment, god_type: ItemId) -> ItemId {
    let core = env.next_id();
    env.mark_as_module(core);
    env.define(
        core,
        Item::Defining {
            base: god_type,
            definitions: vec![],
        },
    );

    core
}

pub fn define_rover_item(env: &mut Environment) -> (ItemId, ItemId) {
    let (god_type, lang) = define_lang_item(env);
    let core = define_core_item(env, god_type);

    let rover = env.next_id();
    env.mark_as_module(rover);
    env.define(
        rover,
        Item::Defining {
            base: god_type,
            definitions: vec![(format!("lang"), lang), (format!("core"), core)],
        },
    );
    (rover, god_type)
}
