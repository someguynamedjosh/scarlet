use crate::{
    environment::Environment,
    item::{
        definitions::{
            decision::DDecision,
            is_populated_struct::DIsPopulatedStruct,
            structt::{AtomicStructMember, DAtomicStructMember, DPopulatedStruct},
            substitution::DSubstitution,
            variable::DVariable,
        },
        item::ItemPtr,
        resolvable::{from::RFrom, DResolvable, RSubstitution, UnresolvedItemError},
        Item, ItemDefinition,
    },
    scope::{SPlain, Scope},
    util::PtrExtension,
};

/// Makes a dex that returns true if the dependency "x" is substituted with a
/// value that could have been returned by `from_item`.
pub(super) fn create_from_dex(env: &Environment, from: ItemPtr) -> ItemPtr {
    let scope = || SPlain(from.ptr_clone());
    let into = Item::placeholder_with_scope(Box::new(scope()));
    from.borrow_mut().from_dex = Some(into.ptr_clone());
    let x = env.get_language_item("x");

    if let Some(structt) = from.downcast_definition::<DPopulatedStruct>() {
        let structt = structt.clone();

        let is_populated_struct = DIsPopulatedStruct::new(env, x.ptr_clone(), Box::new(scope()));

        let x_value = DAtomicStructMember::new(x.ptr_clone(), AtomicStructMember::Value);
        let x_value = Item::new(x_value, scope());
        let value_from_value = RFrom {
            left: x_value,
            right: structt.get_value().ptr_clone(),
        };
        let value_from_value = Item::new(DResolvable::new(value_from_value), scope());

        let x_rest = DAtomicStructMember::new(x.ptr_clone(), AtomicStructMember::Rest);
        let x_rest = Item::new(x_rest, scope());
        let rest_from_rest = RFrom {
            left: x_rest,
            right: structt.get_rest().ptr_clone(),
        };
        let rest_from_rest = Item::new(DResolvable::new(rest_from_rest), scope());

        let first_two = create_and(env, is_populated_struct, value_from_value, scope());
        redefine_as_and(env, into.ptr_clone(), first_two, rest_from_rest);
    } else if let Some(var) = from.downcast_definition::<DVariable>() {
        let var_ptr = var.get_variable();
        let var = var_ptr.borrow();
        let invs = Vec::from(var.get_invariants());
        let deps = Vec::from(var.get_dependencies());
        if deps.len() > 0 {
            todo!();
        }
        let truee = env.get_true();

        let statement = if invs.len() == 0 {
            truee.ptr_clone()
        } else {
            let mut statement = invs[0].ptr_clone();
            for part in &invs[1..] {
                statement = create_and(env, statement, part.ptr_clone(), scope());
            }
            statement
        };

        let subs = vec![(var_ptr.ptr_clone(), x.ptr_clone())]
            .into_iter()
            .collect();
        let con = DSubstitution::new_unchecked(into.ptr_clone(), statement, subs);
        into.redefine(con.clone_into_box());
    } else {
        let truee = env.get_true();
        let falsee = env.get_false();
        let equal = DDecision::new(
            x.ptr_clone(),
            from.ptr_clone(),
            truee.ptr_clone(),
            falsee.ptr_clone(),
        );
        into.redefine(equal.clone_into_box());
    }
    into
}

fn create_and(
    env: &Environment,
    left: ItemPtr,
    right: ItemPtr,
    scope: impl Scope + 'static,
) -> ItemPtr {
    let and = env.get_language_item("and");
    Item::new(
        DResolvable::new(RSubstitution {
            base: and.ptr_clone(),
            named_subs: vec![].into_iter().collect(),
            anonymous_subs: vec![left, right],
        }),
        scope,
    )
}

fn redefine_as_and(env: &Environment, original: ItemPtr, left: ItemPtr, right: ItemPtr) {
    let and = env.get_language_item("and");
    original.redefine(
        DResolvable::new(RSubstitution {
            base: and.ptr_clone(),
            named_subs: vec![].into_iter().collect(),
            anonymous_subs: vec![left, right],
        })
        .clone_into_box(),
    )
}
