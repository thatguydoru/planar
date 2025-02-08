use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;

use crate::errors::ValueError;

pub type Id = u64;

pub type TextField = String;

pub type TitleField = CharField<255>;

#[derive(Debug)]
pub struct CharField<const SIZE: usize>(String);

impl<'a, const SIZE: usize> TryFrom<&'a str> for CharField<SIZE> {
    type Error = ValueError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.len() <= SIZE {
            Ok(Self(value.to_string()))
        } else {
            Err(ValueError::from("string is too long"))
        }
    }
}

impl<const SIZE: usize> Display for CharField<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct ForeignKeyField<M, I> {
    pub id: I,
    _model: PhantomData<M>,
}

impl<M, I> ForeignKeyField<M, I> {
    pub fn new(id: I) -> Self {
        Self {
            id,
            _model: PhantomData,
        }
    }
}

#[derive(Debug, Default)]
pub enum CardActiveStateField {
    #[default]
    Todo,
    Doing,
    Done,
    Archived,
}

#[derive(Debug)]
pub struct RgbaColorField {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
