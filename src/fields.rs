use std::fmt;
use std::fmt::{Display, Formatter};
use std::string::ToString;

pub type Id = u64;

pub type TextField = String;

#[derive(Debug)]
pub enum ColorFmt {
    Rgba { r: u8, g: u8, b: u8, a: u8 },
    Hsl { h: u8, s: u8, l: u8 },
}

impl ToString for ColorFmt {
    fn to_string(&self) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub struct ColorField(ColorFmt);

impl Display for ColorField {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

#[derive(Debug)]
pub struct CharField<const SIZE: u32>(String);

impl<const SIZE: u32> Display for CharField<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
