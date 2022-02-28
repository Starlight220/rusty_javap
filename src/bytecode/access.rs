use crate::model::class::ClassAccessModifier;
use crate::model::field::FieldAccessModifier;
use crate::model::method::MethodAccessModifier;
use crate::*;
use std::iter::FromIterator;
use crate::bytecode::reader::{ByteReader, Take};
use crate::bytecode::writer::{ByteWriter, Writeable};

macro_rules! impl_rw_for_modifiers {
    ($t:ty) => {
        impl Take<Vec<$t>> for ByteReader {
    fn take(&mut self) -> Result<Vec<$t>, String> {
        let flags: w2 = self.take()?;
        let modifiers = <$t>::variants().into_iter();
        Ok(Vec::from_iter(
            modifiers.filter(|&acc| (acc as i32 as w2) & flags != 0),
        ))
    }
}

impl Writeable for Vec<$t> {
    fn write(&self, writer: &mut ByteWriter) {
        let mut flags: w2 = 0;
        for &modifier in self {
            flags |= modifier as i32 as w2;
        }
        writer.write(&flags);
    }
}
    };
}
impl_rw_for_modifiers!(FieldAccessModifier);

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

impl Writeable for Vec<ClassAccessModifier> {
    fn write(&self, writer: &mut ByteWriter) {
        let mut flags: w2 = 0;
        for &modifier in self {
            flags |= modifier as i32 as w2;
        }
        writer.write(&flags);
    }
}

// impl Take<Vec<FieldAccessModifier>> for ByteReader {
//     fn take(&mut self) -> Result<Vec<FieldAccessModifier>, String> {
//         let flags: w2 = self.take()?;
//         use FieldAccessModifier::*;
//         let modifiers = vec![
//             PUBLIC, PRIVATE, PROTECTED, STATIC, FINAL, VOLATILE, TRANSIENT, SYNTHETIC, ENUM,
//         ]
//         .into_iter();
//         Ok(Vec::from_iter(
//             modifiers.filter(|&acc| (acc as i32 as w2) & flags != 0),
//         ))
//     }
// }
//
// impl Writeable for Vec<FieldAccessModifier> {
//     fn write(&self, writer: &mut ByteWriter) {
//         let mut flags: w2 = 0;
//         for &modifier in self {
//             flags |= modifier as i32 as w2;
//         }
//         writer.write(&flags);
//     }
// }

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
