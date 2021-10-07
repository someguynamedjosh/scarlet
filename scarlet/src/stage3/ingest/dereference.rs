use super::context::Context;
use crate::{stage2::structure as s2, stage3::structure as s3};

pub(super) struct DereferencedItem {
    pub base: s2::ItemId,
    pub subs: Vec<(s3::VariableId, s3::ValueId)>,
}

impl DereferencedItem {
    fn wrapped_with(self, other: Self) -> Self {
        Self {
            base: self.base,
            subs: [self.subs, other.subs].concat(),
        }
    }
}

impl From<&s2::ItemId> for DereferencedItem {
    fn from(value: &s2::ItemId) -> Self {
        (*value).into()
    }
}

impl From<s2::ItemId> for DereferencedItem {
    fn from(value: s2::ItemId) -> Self {
        Self {
            base: value,
            subs: Vec::new(),
        }
    }
}

impl<'e, 'i> Context<'e, 'i> {
    pub fn dereference_identifier(&mut self, name: &String) -> DereferencedItem {
        for index in 0..self.parent_scopes.len() {
            let scope = &self.parent_scopes[index];
            if let Some(item) = scope.get(name) {
                let result = item.into();
                self.exclude_scopes(index);
                return result;
            }
        }
        todo!(
            "Nice error, no identifier {} in {:#?}",
            name,
            self.parent_scopes
        )
    }

    pub fn dereference_member(
        &mut self,
        base: s2::ItemId,
        name: &String,
    ) -> Option<DereferencedItem> {
        match &self.input.items[base] {
            s2::Item::Defining { base, definitions } => {
                self.parent_scopes.push(definitions);
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
            s2::Item::From { base, .. } => self.dereference_member(*base, name),
            s2::Item::Identifier(ident) => {
                let ident = self.dereference_identifier(ident);
                let err = format!("No member {} in {:?}", name, ident.base);
                let member = self.dereference_member(ident.base, name).expect(&err);
                Some(member.wrapped_with(ident))
            }
            s2::Item::Member { .. } => todo!(),
            s2::Item::Substituting { .. } => todo!(),
            _ => None,
        }
    }
}
