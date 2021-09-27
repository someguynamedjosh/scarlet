use crate::{
    shared::{BuiltinOperation, ItemId},
    stage4::{ingest::var_list::VarList, structure::Environment},
    util::*,
};

impl Environment {
    fn maybe_add_deps(
        &mut self,
        item: ItemId,
        to: &mut VarList,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<(), String> {
        if let Some(deps) = self
            .compute_dependencies(item, currently_computing)
            .into_option_or_err()?
        {
            to.append(deps.as_slice());
        }
        MOk(())
    }

    pub fn compute_pick_dependencies(
        &mut self,
        initial_clause: (ItemId, ItemId),
        elif_clauses: Vec<(ItemId, ItemId)>,
        else_clause: ItemId,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<VarList, String> {
        let mut deps = VarList::new();
        self.maybe_add_deps(initial_clause.0, &mut deps, currently_computing.clone())?;
        self.maybe_add_deps(initial_clause.1, &mut deps, currently_computing.clone())?;
        self.maybe_add_deps(else_clause, &mut deps, currently_computing.clone())?;
        for (cond, val) in elif_clauses {
            self.maybe_add_deps(cond, &mut deps, currently_computing.clone())?;
            self.maybe_add_deps(val, &mut deps, currently_computing.clone())?;
        }
        MOk(deps)
    }
}
