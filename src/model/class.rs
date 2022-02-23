use std::vec::Vec;
use core::fmt::{Display, Formatter};
use core::option::Option;
use crate::{w2, w4};
use crate::model::attrs::Attribute;
use crate::model::field::Field;
use crate::model::interface::Interface;
use crate::model::method::Method;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Class {
    version: Version,
    access_flags: Vec<ClassAccessModifier>,
    this_class: String,
    super_class: Option<String>,
    interfaces: Vec<Interface>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    attributes: Vec<Attribute>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Version {
    magic: w4,
    major: w2,
    minor: w2,
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

#[derive(Debug, Copy, Clone)]
#[derive(Serialize, Deserialize)]
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
