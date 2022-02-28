use crate::model::attrs::Attribute;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    access_flags: Vec<FieldAccessModifier>,
    name: String,
    descriptor: String, // TODO: add a descriptor struct?
    attributes: Vec<Attribute>,
}

impl Field {
    pub fn new(
        access_flags: Vec<FieldAccessModifier>,
        name: String,
        descriptor: String, // TODO: add a descriptor struct?
        attributes: Vec<Attribute>,
    ) -> Field {
        Field {
            access_flags,
            name,
            descriptor,
            attributes,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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

impl FieldAccessModifier {
    pub fn variants() -> Vec<Self> {
        use FieldAccessModifier::*;
        vec![
            PUBLIC, PRIVATE, PROTECTED, STATIC, FINAL, VOLATILE, TRANSIENT, SYNTHETIC, ENUM,
        ]
    }
}

impl Display for FieldAccessModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
