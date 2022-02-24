use crate::model::class::ClassAccessModifier;
use crate::model::field::FieldAccessModifier;
use crate::model::method::MethodAccessModifier;
use crate::*;
use std::iter::FromIterator;

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

impl Take<Vec<MethodAccessModifier>> for ByteReader {
    fn take(&mut self) -> Result<Vec<MethodAccessModifier>, String> {
        let flags: w2 = self.take()?;
        use model::method::MethodAccessModifier::*;
        let modifiers = vec![
            PUBLIC,
            PRIVATE,
            PROTECTED,
            STATIC,
            FINAL,
            SYNCHRONIZED,
            BRIDGE,
            VARARGS,
            NATIVE,
            ABSTRACT,
            STRICT,
            SYNTHETIC,
        ]
        .into_iter();
        Ok(Vec::from_iter(
            modifiers.filter(|&acc| (acc as i32 as w2) & flags != 0),
        ))
    }
}
