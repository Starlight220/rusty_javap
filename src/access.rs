use crate::*;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone)]
pub enum AccessModifier {
    PUBLIC = 0x0001,
    FINAL = 0x0010,
    SUPER = 0x0020,
    INTERFACE = 0x0200,
    ABSTRACT = 0x0400,
    SYNTHETIC = 0x1000,
    ANNOTATION = 0x2000,
    ENUM = 0x4000,
    MODULE = 0x8000,
}

impl Display for AccessModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Take<Vec<AccessModifier>> for ByteReader {
    fn take(&mut self) -> Result<Vec<AccessModifier>, String> {
        let flags: w2 = self.take()?;
        use AccessModifier::*;
        let modifiers = vec![
            PUBLIC, FINAL, SUPER, INTERFACE, ABSTRACT, SYNTHETIC, ANNOTATION, ENUM, MODULE,
        ]
        .into_iter();
        Ok(Vec::from_iter(
            modifiers.filter(|&acc| (acc as i32 as w2) & flags != 0),
        ))
    }
}
