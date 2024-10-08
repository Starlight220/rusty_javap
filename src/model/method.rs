use crate::model::attrs::Attribute;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Method {
    pub access_flags: Vec<MethodAccessModifier>,
    pub name: String,
    pub descriptor: String, // TODO: add a descriptor struct?
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum MethodAccessModifier {
    PUBLIC = 0x0001,
    PRIVATE = 0x0002,
    PROTECTED = 0x0004,
    STATIC = 0x0008,
    FINAL = 0x0010,
    SYNCHRONIZED = 0x0020,
    BRIDGE = 0x0040,
    VARARGS = 0x0080,
    NATIVE = 0x0100,
    ABSTRACT = 0x0400,
    STRICT = 0x0800,
    SYNTHETIC = 0x1000,
}
impl MethodAccessModifier {
    pub fn variants() -> Vec<Self> {
        use MethodAccessModifier::*;
        vec![
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
    }
}
