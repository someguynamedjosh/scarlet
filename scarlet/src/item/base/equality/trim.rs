use super::Equal;
use crate::item::definitions::substitution::Substitutions;

pub(super) fn trim_result(result: &mut Equal) {
    match result {
        Equal::Yes(left, right) => trim_yes(left, right),
        _ => (),
    }
}

fn trim_yes(left: &mut Substitutions, right: &mut Substitutions) {}
