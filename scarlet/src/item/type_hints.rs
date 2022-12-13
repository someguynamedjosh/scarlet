use super::ItemPtr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeHint {
    MustBeContainedIn(ItemPtr),
    MustHaveField { name: String, value: ItemPtr },
}
