use crate::{
    environment::Environment,
    impl_any_eq_from_regular_eq,
    item::{
        check::CheckFeature,
        dependencies::{Dcc, DepResult, DependenciesFeature, OnlyCalledByDcc},
        equality::{Ecc, Equal, EqualResult, EqualityFeature, OnlyCalledByEcc, PermissionToRefine},
        invariants::{
            Icc, InvariantSet, InvariantSetPtr, InvariantsFeature, InvariantsResult,
            OnlyCalledByIcc,
        },
        ItemDefinition, ItemPtr, ContainmentType,
    },
    scope::{LookupIdentResult, ReverseLookupIdentResult, Scope},
    util::PtrExtension,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DDecision {
    left: ItemPtr,
    right: ItemPtr,
    when_equal: ItemPtr,
    when_not_equal: ItemPtr,
}

impl DDecision {
    pub fn new(
        left: ItemPtr,
        right: ItemPtr,
        when_equal: ItemPtr,
        when_not_equal: ItemPtr,
    ) -> Self {
        Self {
            left,
            right,
            when_equal,
            when_not_equal,
        }
    }

    pub fn left(&self) -> &ItemPtr {
        &self.left
    }

    pub fn right(&self) -> &ItemPtr {
        &self.right
    }

    pub fn when_equal(&self) -> &ItemPtr {
        &self.when_equal
    }

    pub fn when_not_equal(&self) -> &ItemPtr {
        &self.when_not_equal
    }

    pub(crate) fn set_when_equal(&mut self, when_equal: ItemPtr) {
        self.when_equal = when_equal;
    }
}

impl_any_eq_from_regular_eq!(DDecision);

impl ItemDefinition for DDecision {
    fn clone_into_box(&self) -> Box<dyn ItemDefinition> {
        Box::new(self.clone())
    }

    fn contents(&self) -> Vec<(ContainmentType, &ItemPtr)> {
        vec![
            (ContainmentType::Computational, &self.left),
            (ContainmentType::Computational, &self.right),
            (ContainmentType::Computational, &self.when_equal),
            (ContainmentType::Computational, &self.when_not_equal),
        ]
    }
}

impl CheckFeature for DDecision {}

impl DependenciesFeature for DDecision {
    fn get_dependencies_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Dcc,
        _: OnlyCalledByDcc,
    ) -> DepResult {
        let mut deps = ctx.get_dependencies(&self.left);
        deps.append(ctx.get_dependencies(&self.right));
        deps.append(ctx.get_dependencies(&self.when_equal));
        deps.append(ctx.get_dependencies(&self.when_not_equal));
        deps
    }
}

impl EqualityFeature for DDecision {
    fn get_equality_using_context(
        &self,
        ctx: &mut Ecc,
        can_refine: PermissionToRefine,
        _: OnlyCalledByEcc,
    ) -> EqualResult {
        let others = if let Some(other) = ctx.rhs().downcast_definition::<Self>() {
            Some([
                other.left.ptr_clone(),
                other.right.ptr_clone(),
                other.when_equal.ptr_clone(),
                other.when_not_equal.ptr_clone(),
            ])
        } else {
            None
        };
        if let Some(others) = others {
            let [other_left, other_right, other_when_equal, other_when_not_equal] = others;
            Ok(Equal::and(vec![
                ctx.refine_and_get_equality(self.left.ptr_clone(), other_left, can_refine)?,
                ctx.refine_and_get_equality(self.right.ptr_clone(), other_right, can_refine)?,
                ctx.refine_and_get_equality(
                    self.when_equal.ptr_clone(),
                    other_when_equal,
                    can_refine,
                )?,
                ctx.refine_and_get_equality(
                    self.when_not_equal.ptr_clone(),
                    other_when_not_equal,
                    can_refine,
                )?,
            ]))
        } else {
            Ok(Equal::Unknown)
        }
    }
}

impl InvariantsFeature for DDecision {
    fn get_invariants_using_context(
        &self,
        this: &ItemPtr,
        ctx: &mut Icc,
        _: OnlyCalledByIcc,
    ) -> InvariantsResult {
        let true_invs = self.when_equal.get_invariants()?;
        let false_invs = self.when_equal.get_invariants()?;
        let mut result_statements = Vec::new();
        for true_inv in true_invs.borrow().statements() {
            for (index, false_inv) in false_invs.borrow().statements().iter().enumerate() {
                if true_inv.get_equality(false_inv, 4) == Ok(Equal::yes()) {
                    result_statements.push(true_inv.ptr_clone());
                    break;
                }
            }
        }
        let len = result_statements.len();
        Ok(InvariantSet::new_justified_by(
            this.ptr_clone(),
            result_statements,
            vec![vec![vec![true_invs, false_invs]]; len],
        ))
    }
}

#[derive(Clone, Debug)]
pub struct SWithInvariant(pub InvariantSetPtr, pub ItemPtr);

impl Scope for SWithInvariant {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident(&self, _ident: &str) -> LookupIdentResult {
        Ok(None)
    }

    fn local_reverse_lookup_ident(
        &self,
        _env: &mut Environment,
        _value: ItemPtr,
    ) -> ReverseLookupIdentResult {
        Ok(None)
    }

    fn local_get_invariant_sets(&self) -> Vec<InvariantSetPtr> {
        vec![self.0.ptr_clone()]
    }

    fn parent(&self) -> Option<ItemPtr> {
        Some(self.1.ptr_clone())
    }
}
