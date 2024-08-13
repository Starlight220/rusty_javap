use crate::bytecode::access::impl_rw_for_modifiers;
use crate::bytecode::reader::{ByteReader, Take};
use crate::bytecode::writer::{ByteWriter, Writeable};
use crate::w2;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;

use serde::{Deserialize, Serialize};

pub type MethodParameters = Vec<MethodParameter>;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum MethodParameterAccessFlags {
    FINAL = 0x0010,
    SYNTHETIC = 0x1000,
    MANDATED = 0x8000,
}

impl MethodParameterAccessFlags {
    pub fn variants() -> Vec<Self> {
        use MethodParameterAccessFlags::*;
        vec![FINAL, SYNTHETIC, MANDATED]
    }
}

impl Display for MethodParameterAccessFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl_rw_for_modifiers!(MethodParameterAccessFlags);

#[derive(Debug, Deserialize, Serialize)]
pub struct MethodParameter {
    pub name: Option<String>,
    pub access_flags: Vec<MethodParameterAccessFlags>,
}
