use crate::item::{CycleDetectingDebug, ItemDefinition, ItemPtr};

pub struct DVariable {
    r#type: ItemPtr,
}

impl CycleDetectingDebug for DVariable {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter,
        stack: &[*const crate::item::Item],
    ) -> std::fmt::Result {
        write!(f, "ANY ")?;
        self.r#type.fmt(f, stack)
    }
}

impl ItemDefinition for DVariable {}

impl DVariable {
    pub fn new(r#type: ItemPtr) -> Self {
        Self { r#type }
    }
}
