use crate::{
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        dependencies::{Dcc, DepResult, Dependencies, DependenciesFeature, OnlyCalledByDcc},
        equality::{Ecc, Equal, EqualResult, EqualityFeature, OnlyCalledByEcc, PermissionToRefine},
        invariants::{
            Icc, InvariantSet, InvariantSetPtr, InvariantsFeature, InvariantsResult,
            OnlyCalledByIcc,
        },
        ItemDefinition, ItemPtr,
    },
    shared::{Id, Pool},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DOther {
    other: ItemPtr,
    recursive: bool,
}

impl DOther {
    pub fn new(other: ItemPtr, recursive: bool) -> Self {
        Self { other, recursive }
    }

    pub fn new_plain(other: ItemPtr) -> Self {
        Self::new(other, false)
    }

    pub fn new_recursive(other: ItemPtr) -> Self {
        Self::new(other, true)
    }

    pub fn other(&self) -> &ItemPtr {
        &self.other
    }

    pub fn is_recursive(&self) -> bool {
        self.recursive
    }

    pub fn mark_recursive(&mut self) {
        self.recursive = true;
    }
}

impl_any_eq_from_regular_eq!(DOther);

impl ItemDefinition for DOther {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }
}

impl CheckFeature for DOther {}

impl DependenciesFeature for DOther {
    fn get_dependencies_using_context(&self, ctx: &mut Dcc, _: OnlyCalledByDcc) -> DepResult {
        ctx.get_dependencies(&self.other)
    }
}

impl EqualityFeature for DOther {
    fn get_equality_using_context(
        &self,
        _ctx: &Ecc,
        _can_refine: PermissionToRefine,
        _: OnlyCalledByEcc,
    ) -> EqualResult {
        unreachable!()
    }
}

impl InvariantsFeature for DOther {
    fn get_invariants_using_context(
        &self,
        _this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        ctx.get_invariants(&self.other)
    }
}
