use super::{variable::CVariable, ConstructId};
use crate::shared::OrderedMap;

pub type Substitutions = OrderedMap<CVariable, ConstructId>;
