use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        dependencies::{Dcc, DepResult, DependenciesFeature, OnlyCalledByDcc},
        equality::{Ecc, Equal, EqualResult, EqualSuccess, EqualityFeature, OnlyCalledByEcc},
        invariants::{Icc, InvariantsFeature, InvariantsResult, OnlyCalledByIcc},
        util::{is_bool, placeholder},
        Item, ItemDefinition, ItemPtr,
    },
    scope::Scope,
    util::rcrc,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DIsPopulatedStruct {
    base: ItemPtr,
    is_bool: ItemPtr,
}

impl DIsPopulatedStruct {
    pub fn new(env: &Environment, base: ItemPtr, scope: Box<dyn Scope>) -> ItemPtr {
        let def = Self {
            base,
            is_bool: placeholder(),
        };
        Item::new_self_referencing(def, scope, |this_ptr, this| {
            this.is_bool = is_bool(env, this_ptr);
        })
    }

    pub fn get_base(&self) -> &ItemPtr {
        &self.base
    }
}

impl_any_eq_from_regular_eq!(DIsPopulatedStruct);

impl ItemDefinition for DIsPopulatedStruct {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }
}

impl CheckFeature for DIsPopulatedStruct {}

impl DependenciesFeature for DIsPopulatedStruct {
    fn get_dependencies_using_context(
        &self,
        _this: &ItemPtr,
        ctx: &mut Dcc,
        affects_return_value: bool,
        _: OnlyCalledByDcc,
    ) -> DepResult {
        ctx.get_dependencies(&self.base, affects_return_value)
    }
}

impl EqualityFeature for DIsPopulatedStruct {
    fn get_equality_using_context(&self, ctx: &mut Ecc, _: OnlyCalledByEcc) -> EqualResult {
        let other = if let Some(other) = ctx.other().downcast_definition::<Self>() {
            Some(other.base.ptr_clone())
        } else {
            None
        };
        let equal = if let Some(other) = other {
            ctx.with_primary_and_other(self.base.ptr_clone(), other)
                .get_equality_left()?
        } else {
            Equal::Unknown
        };
        Ok(EqualSuccess {
            equal,
            unique: true,
        })
    }
}

impl InvariantsFeature for DIsPopulatedStruct {
    fn get_invariants_using_context(
        &self,
        _this: &ItemPtr,
        _ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        let invs = self.base.get_invariants()?;
        let mut set = invs.borrow().clone();
        set.push(self.is_bool.ptr_clone());
        Ok(rcrc(set))
    }
}
