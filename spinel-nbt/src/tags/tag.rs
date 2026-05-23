use crate::tags::serializer::TagSerializer;
use crate::{Nbt, NbtCompound};
use std::sync::Arc;

#[derive(Clone)]
pub struct Tag<T> {
    key: String,
    serializer: TagSerializer<T>,
    default_value: Option<Arc<dyn Fn() -> T + Send + Sync>>,
    pub(crate) path: Vec<String>,
    is_transient: bool,
}

impl<T> Tag<T> {
    pub fn new(key: impl Into<String>, serializer: TagSerializer<T>) -> Self {
        Self {
            key: key.into(),
            serializer,
            default_value: None,
            path: Vec::new(),
            is_transient: false,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn default_value(mut self, default_value: T) -> Self
    where
        T: Clone + Send + Sync + 'static,
    {
        self.default_value = Some(Arc::new(move || default_value.clone()));
        self
    }

    pub fn default_with(mut self, default_value: impl Fn() -> T + Send + Sync + 'static) -> Self {
        self.default_value = Some(Arc::new(default_value));
        self
    }

    pub fn path(mut self, path: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.path = path.into_iter().map(Into::into).collect();
        self
    }

    pub fn transient(mut self) -> Self {
        self.is_transient = true;
        self
    }

    pub fn is_transient(&self) -> bool {
        self.is_transient
    }

    pub fn is_view(&self) -> bool {
        self.key.is_empty()
    }

    pub fn read(&self, compound: &NbtCompound) -> Option<T> {
        self.read_without_default(compound).or_else(|| {
            self.default_value
                .as_ref()
                .map(|default_value| default_value())
        })
    }

    pub fn read_without_default(&self, compound: &NbtCompound) -> Option<T> {
        self.read_parent(compound)
            .and_then(|parent| self.read_from_parent(parent))
    }

    pub fn write(&self, compound: &mut NbtCompound, value: Option<T>) {
        if self.is_transient {
            return;
        }
        let Some(value) = value else {
            self.remove(compound);
            return;
        };
        let Some(nbt) = self.serializer.write(value) else {
            self.remove(compound);
            return;
        };
        self.write_nbt(compound, nbt);
    }

    pub fn copy_value(&self, value: &T) -> T {
        self.serializer.copy_value(value)
    }

    pub fn map<R>(
        self,
        read_map: impl Fn(T) -> R + Send + Sync + 'static,
        write_map: impl Fn(R) -> T + Send + Sync + 'static,
    ) -> Tag<R>
    where
        T: 'static,
        R: Clone + Send + Sync + 'static,
    {
        let read_serializer = self.serializer.clone();
        let write_serializer = self.serializer;
        Tag::new(
            self.key,
            TagSerializer::cloned(
                move |nbt| read_serializer.read(nbt).map(&read_map),
                move |value| write_serializer.write(write_map(value)),
            ),
        )
    }

    pub fn list(self) -> Tag<Vec<T>>
    where
        T: Clone + Send + Sync + 'static,
    {
        let read_serializer = self.serializer.clone();
        let write_serializer = self.serializer;
        Tag::new(
            self.key,
            TagSerializer::cloned(
                move |nbt| match nbt {
                    Nbt::List(values) => values
                        .iter()
                        .map(|value| read_serializer.read(value))
                        .collect(),
                    _ => None,
                },
                move |values: Vec<T>| {
                    Some(Nbt::List(
                        values
                            .into_iter()
                            .filter_map(|value| write_serializer.write(value))
                            .collect::<Vec<_>>()
                            .into_boxed_slice(),
                    ))
                },
            ),
        )
    }

    fn read_parent<'a>(&self, compound: &'a NbtCompound) -> Option<&'a NbtCompound> {
        self.path.iter().try_fold(compound, |parent, path_entry| {
            match parent.get(path_entry) {
                Some(Nbt::Compound(compound)) => Some(compound),
                _ => None,
            }
        })
    }

    fn read_from_parent(&self, parent: &NbtCompound) -> Option<T> {
        if self.is_view() {
            return self.serializer.read(&Nbt::Compound(parent.clone()));
        }
        parent
            .get(&self.key)
            .and_then(|nbt| self.serializer.read(nbt))
    }

    fn write_nbt(&self, compound: &mut NbtCompound, nbt: Nbt) {
        let parent = self.write_parent(compound);
        if self.is_view() {
            if let Nbt::Compound(value) = nbt {
                *parent = value;
            }
            return;
        }
        parent.insert(self.key.clone(), nbt);
    }

    fn remove(&self, compound: &mut NbtCompound) {
        if let Some(parent) = self.read_parent_mut(compound) {
            parent.remove(&self.key);
        }
    }
}
