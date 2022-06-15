#![cfg(test)]

use super::Dependencies;
use crate::item::{
    definitions::variable::{Variable, VariableOrder},
    test_util::unique,
};

#[test]
fn affecting_dep_overrides_non_affecting_dep() {
    let order = VariableOrder::new(0, 0, 0);
    let item = unique();
    let var = Variable::new(vec![], vec![], item, order);

    let mut under_test = Dependencies::new();
    under_test.append(Variable::as_dependency(&var, false));
    under_test.append(Variable::as_dependency(&var, true));

    let mut expected = Dependencies::new();
    expected.append(Variable::as_dependency(&var, true));

    assert_eq!(under_test, expected);
}

#[test]
fn non_affecting_dep_does_not_override_affecting_dep() {
    let order = VariableOrder::new(0, 0, 0);
    let item = unique();
    let var = Variable::new(vec![], vec![], item, order);

    let mut under_test = Dependencies::new();
    under_test.append(Variable::as_dependency(&var, true));
    under_test.append(Variable::as_dependency(&var, false));

    let mut expected = Dependencies::new();
    expected.append(Variable::as_dependency(&var, true));

    assert_eq!(under_test, expected);
}
