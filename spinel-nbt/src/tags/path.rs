use crate::{Nbt, NbtCompound, Tag};

impl<T> Tag<T> {
    pub(crate) fn write_parent<'a>(&self, compound: &'a mut NbtCompound) -> &'a mut NbtCompound {
        let mut parent = compound;
        for path_entry in self.path_entries() {
            let needs_compound = !matches!(parent.get(path_entry), Some(Nbt::Compound(_)));
            if needs_compound {
                parent.insert(path_entry.clone(), NbtCompound::new());
            }
            parent = match parent.get_mut(path_entry) {
                Some(Nbt::Compound(compound)) => compound,
                _ => unreachable!(),
            };
        }
        parent
    }

    pub(crate) fn read_parent_mut<'a>(
        &self,
        compound: &'a mut NbtCompound,
    ) -> Option<&'a mut NbtCompound> {
        self.path_entries()
            .iter()
            .try_fold(compound, |parent, path_entry| {
                match parent.get_mut(path_entry) {
                    Some(Nbt::Compound(compound)) => Some(compound),
                    _ => None,
                }
            })
    }

    pub(crate) fn path_entries(&self) -> &Vec<String> {
        &self.path
    }
}
