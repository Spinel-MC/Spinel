use crate::Identifier;
use std::{any::Any, collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataComponentType<T> {
    key: Identifier,
    marker: PhantomData<T>,
}

impl<T> DataComponentType<T> {
    #[must_use]
    pub const fn new(key: Identifier) -> Self {
        Self {
            key,
            marker: PhantomData,
        }
    }

    #[must_use]
    pub const fn key(&self) -> &Identifier {
        &self.key
    }
}

#[derive(Default)]
pub struct DataComponentMap {
    components: HashMap<Identifier, Box<dyn Any + Send + Sync>>,
}

impl DataComponentMap {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn common_item_components() -> Self {
        Self::new()
    }

    pub fn set<T>(&mut self, component: DataComponentType<T>, value: T)
    where
        T: Any + Send + Sync,
    {
        self.components
            .insert(component.key.clone(), Box::new(value));
    }

    #[must_use]
    pub fn get<T>(&self, component: DataComponentType<T>) -> Option<&T>
    where
        T: Any,
    {
        self.components
            .get(&component.key)
            .and_then(|value| value.downcast_ref())
    }
}
