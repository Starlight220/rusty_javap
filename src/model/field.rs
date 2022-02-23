use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Field {
    access_flags: Vec<FieldAccessModifier>,
    name: String,
    descriptor: String, // TODO: add a descriptor struct?
    attributes: Attributes,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:", self.name)?;
        writeln!(f, "\t\tDescriptor:\t{}", self.descriptor)?;
        writeln!(f, "\t\tAccess:\t{:?}", self.access_flags)?;
        writeln!(
            f,
            "\t\t{}",
            format!("{}", self.attributes).replace("\t", "\t\t\t")
        )?;
        write!(f, "")
    }
}

#[derive(Debug, Copy, Clone)]
#[derive(Serialize, Deserialize)]
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
