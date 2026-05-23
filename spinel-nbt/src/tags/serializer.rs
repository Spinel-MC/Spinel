use crate::Nbt;
use std::sync::Arc;

pub struct TagSerializer<T> {
    reader: Arc<dyn Fn(&Nbt) -> Option<T> + Send + Sync>,
    writer: Arc<dyn Fn(T) -> Option<Nbt> + Send + Sync>,
    copier: Arc<dyn Fn(&T) -> T + Send + Sync>,
}

impl<T> Clone for TagSerializer<T> {
    fn clone(&self) -> Self {
        Self {
            reader: self.reader.clone(),
            writer: self.writer.clone(),
            copier: self.copier.clone(),
        }
    }
}

impl<T> TagSerializer<T> {
    pub fn new(
        reader: impl Fn(&Nbt) -> Option<T> + Send + Sync + 'static,
        writer: impl Fn(T) -> Option<Nbt> + Send + Sync + 'static,
        copier: impl Fn(&T) -> T + Send + Sync + 'static,
    ) -> Self {
        Self {
            reader: Arc::new(reader),
            writer: Arc::new(writer),
            copier: Arc::new(copier),
        }
    }

    pub fn read(&self, nbt: &Nbt) -> Option<T> {
        (self.reader)(nbt)
    }

    pub fn write(&self, value: T) -> Option<Nbt> {
        (self.writer)(value)
    }

    pub fn copy_value(&self, value: &T) -> T {
        (self.copier)(value)
    }
}

impl<T> TagSerializer<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn cloned(
        reader: impl Fn(&Nbt) -> Option<T> + Send + Sync + 'static,
        writer: impl Fn(T) -> Option<Nbt> + Send + Sync + 'static,
    ) -> Self {
        Self::new(reader, writer, Clone::clone)
    }
}
