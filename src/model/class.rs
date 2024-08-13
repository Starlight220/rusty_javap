use crate::model::attrs::Attribute;
use crate::model::field::Field;
use crate::model::interface::Interface;
use crate::model::method::Method;
use crate::{w2, w4};
use core::fmt::{Display, Formatter};
use core::option::Option;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    pub version: Version,
    pub access_flags: Vec<ClassAccessModifier>,
    pub this_class: String,
    pub super_class: Option<String>,
    pub interfaces: Vec<Interface>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub magic: w4,
    pub major: w2,
    pub minor: w2,
}

impl Version {
    pub fn new(magic: w4, major: w2, minor: w2) -> Version {
        Version {
            magic,
            major,
            minor,
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "magic: {:#X}\nmajor: {}\nminor: {}",
            self.magic, self.major, self.minor
        )
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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

impl ClassAccessModifier {
    pub fn variants() -> Vec<Self> {
        use ClassAccessModifier::*;
        vec![
            PUBLIC, FINAL, SUPER, INTERFACE, ABSTRACT, SYNTHETIC, ANNOTATION, ENUM, MODULE,
        ]
    }
}

impl Display for ClassAccessModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
