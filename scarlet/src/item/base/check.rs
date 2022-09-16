use crate::{item::ItemPtr, diagnostic::Diagnostic, environment::Environment};

pub type CheckResult = Result<(), Diagnostic>;

pub trait CheckFeature {
    #[allow(unused_variables)]
    fn check_self(&self, this: &ItemPtr, env: &mut Environment) -> CheckResult {
        Ok(())
    }
}
