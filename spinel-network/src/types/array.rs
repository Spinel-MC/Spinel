use crate::data_type::DataType;
use std::io::{self, Read, Write};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default)]
pub struct Array<T>(pub Vec<T>);

impl<T> Deref for Array<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> IntoIterator for Array<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Array<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<T: DataType> DataType for Array<T> {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.0.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        Ok(Array(Vec::<T>::decode(r)?))
    }
}
