use super::context::Context;
use crate::{
    stage2::structure::{self as s3},
    stage4::structure as s4,
};

pub(super) struct DereferencedItem {
    pub base: s3::ItemId,
    pub subs: s4::Substitutions,
}

impl DereferencedItem {
    fn wrapped_with(self, other: Self) -> Self {
        Self {
            base: self.base,
            subs: self.subs.union(other.subs),
        }
    }
}

impl From<&s3::ItemId> for DereferencedItem {
    fn from(value: &s3::ItemId) -> Self {
        (*value).into()
    }
}

impl From<s3::ItemId> for DereferencedItem {
    fn from(value: s3::ItemId) -> Self {
        Self {
            base: value,
            subs: s4::Substitutions::new(),
        }
    }
}

impl<'e, 'i> Context<'e, 'i> {
    pub fn dereference_identifier(
        &mut self,
        name: &String,
        in_scope: s3::ItemId,
    ) -> DereferencedItem {
        let scope = &self.input.items[in_scope];
        if let s3::Item::Defining {
            base: _,
            definitions,
        } = &scope.item
        {
            for (candidate, definition) in definitions {
                if candidate == name {
                    return DereferencedItem {
                        base: *definition,
                        subs: s4::Substitutions::new(),
                    };
                }
            }
        }
        match scope.parent_scope {
            Some(parent_scope) => self.dereference_identifier(name, parent_scope),
            None => todo!("Nice error, failed to find identifier {:?}", name),
        }
    }

    pub fn dereference_member(
        &mut self,
        base: s3::ItemId,
        name: &String,
    ) -> Option<DereferencedItem> {
        match &self.input.items[base].item {
            s3::Item::Defining { base, definitions } => {
                if let Some(result) = self.dereference_member(*base, name) {
                    return Some(result);
                }
                for (candidate, item) in definitions {
                    if candidate == name {
                        return Some(item.into());
                    }
                }
                None
            }
            s3::Item::From { base, .. } => self.dereference_member(*base, name),
            s3::Item::Identifier(ident) => {
                let ident = self.dereference_identifier(ident, base);
                let err = format!("No member {} in {:?}", name, ident.base);
                let member = self.dereference_member(ident.base, name).expect(&err);
                Some(member.wrapped_with(ident))
            }
            s3::Item::Member {
                base: that_base,
                name: that_name,
            } => {
                let that = self
                    .dereference_member(*that_base, that_name)
                    .expect("TODO: Nice error");
                let err = format!("No member {} in {:?}", name, that.base);
                let member = self.dereference_member(that.base, name).expect(&err);
                Some(member.wrapped_with(that))
            }
            s3::Item::Substituting { .. } => todo!(),
            _ => None,
        }
    }
}
