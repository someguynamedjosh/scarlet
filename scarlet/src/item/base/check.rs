use super::ItemPtr;

pub type CheckResult = Result<(), ()>;

pub trait CheckFeature {
    #[allow(unused_variables)]
    fn check_self(&self, this: &ItemPtr) -> CheckResult {
        Ok(())
    }
}
