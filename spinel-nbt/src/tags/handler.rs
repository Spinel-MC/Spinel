use crate::{NbtCompound, Tag};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TagHandler {
    compound: NbtCompound,
}

impl TagHandler {
    pub fn new_handler() -> Self {
        Self::default()
    }

    pub fn from_compound(compound: NbtCompound) -> Self {
        Self { compound }
    }

    pub fn readable_copy(&self) -> Self {
        self.clone()
    }

    pub fn copy(&self) -> Self {
        self.clone()
    }

    pub fn update_content(&mut self, compound: NbtCompound) {
        self.compound = compound;
    }

    pub fn as_compound(&self) -> NbtCompound {
        self.compound.clone()
    }
}

pub trait TagReadable {
    fn get_tag<T>(&self, tag: &Tag<T>) -> Option<T>;

    fn has_tag<T>(&self, tag: &Tag<T>) -> bool {
        self.get_tag(tag).is_some()
    }
}

pub trait TagWritable: TagReadable {
    fn set_tag<T>(&mut self, tag: &Tag<T>, value: Option<T>);

    fn remove_tag<T>(&mut self, tag: &Tag<T>) {
        self.set_tag(tag, None);
    }

    fn get_and_set_tag<T>(&mut self, tag: &Tag<T>, value: Option<T>) -> Option<T> {
        let previous_value = self.get_tag(tag);
        self.set_tag(tag, value);
        previous_value
    }

    fn update_tag<T>(&mut self, tag: &Tag<T>, value: impl FnOnce(Option<T>) -> Option<T>) {
        let current_value = self.get_tag(tag);
        self.set_tag(tag, value(current_value));
    }

    fn update_and_get_tag<T>(
        &mut self,
        tag: &Tag<T>,
        value: impl FnOnce(Option<T>) -> Option<T>,
    ) -> Option<T> {
        self.update_tag(tag, value);
        self.get_tag(tag)
    }

    fn get_and_update_tag<T>(
        &mut self,
        tag: &Tag<T>,
        value: impl FnOnce(Option<T>) -> Option<T>,
    ) -> Option<T> {
        let previous_value = self.get_tag(tag);
        self.set_tag(
            tag,
            value(previous_value.as_ref().map(|value| tag.copy_value(value))),
        );
        previous_value
    }
}

pub trait Taggable: TagReadable + TagWritable {
    fn tag_handler(&self) -> &TagHandler;
    fn tag_handler_mut(&mut self) -> &mut TagHandler;
}

impl TagReadable for TagHandler {
    fn get_tag<T>(&self, tag: &Tag<T>) -> Option<T> {
        tag.read(&self.compound)
    }
}

impl TagWritable for TagHandler {
    fn set_tag<T>(&mut self, tag: &Tag<T>, value: Option<T>) {
        tag.write(&mut self.compound, value);
    }
}

impl<T> TagReadable for T
where
    T: Taggable,
{
    fn get_tag<R>(&self, tag: &Tag<R>) -> Option<R> {
        self.tag_handler().get_tag(tag)
    }
}

impl<T> TagWritable for T
where
    T: Taggable,
{
    fn set_tag<R>(&mut self, tag: &Tag<R>, value: Option<R>) {
        self.tag_handler_mut().set_tag(tag, value);
    }
}
