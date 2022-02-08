use crate::*;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone)]
pub enum ClassAccessModifier {
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

impl Display for ClassAccessModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Take<Vec<ClassAccessModifier>> for ByteReader {
    fn take(&mut self) -> Result<Vec<ClassAccessModifier>, String> {
        let flags: w2 = self.take()?;
        use ClassAccessModifier::*;
        let modifiers = vec![
            PUBLIC, FINAL, SUPER, INTERFACE, ABSTRACT, SYNTHETIC, ANNOTATION, ENUM, MODULE,
        ]
        .into_iter();
        Ok(Vec::from_iter(
            modifiers.filter(|&acc| (acc as i32 as w2) & flags != 0),
        ))
    }
}

// ***********************************************

#[derive(Debug, Copy, Clone)]
pub enum FieldAccessModifier {
    PUBLIC = 0x0001,
    PRIVATE = 0x0002,
    PROTECTED = 0x0004,
    STATIC = 0x0008,
    FINAL = 0x0010,
    VOLATILE = 0x0040,
    TRANSIENT = 0x0080,
    SYNTHETIC = 0x1000,
    ENUM = 0x4000,
}

impl Display for FieldAccessModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Take<Vec<FieldAccessModifier>> for ByteReader {
    fn take(&mut self) -> Result<Vec<FieldAccessModifier>, String> {
        let flags: w2 = self.take()?;
        use FieldAccessModifier::*;
        let modifiers = vec![
            PUBLIC, PRIVATE, PROTECTED, STATIC, FINAL, VOLATILE, TRANSIENT, SYNTHETIC, ENUM,
        ]
        .into_iter();
        Ok(Vec::from_iter(
            modifiers.filter(|&acc| (acc as i32 as w2) & flags != 0),
        ))
    }
}
