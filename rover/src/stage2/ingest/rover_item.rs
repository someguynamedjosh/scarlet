use crate::stage2::structure::{Environment, Item, ItemId, PrimitiveType};

fn define_lang_item(env: &mut Environment) -> (ItemId, ItemId) {
    let god_type = env.next_id();
    env.define(god_type, Item::GodType);
    let i32_type = env.next_id();
    env.define(i32_type, Item::PrimitiveType(PrimitiveType::I32));

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
