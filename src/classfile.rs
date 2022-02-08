use std::fmt::{Display, Formatter};
use crate::access::AccessModifier;
use crate::constant_pool::ConstantPool;
use crate::versions::Version;
use crate::{ByteReader, Take, w2};

#[derive(Debug)]
pub struct Class {
    version: Version,
    constant_pool: ConstantPool,
    access_flags: Vec<AccessModifier>,
    this_class: String,
    super_class: Option<String>,
}

impl Take<Class> for ByteReader {
    fn take(&mut self) -> Result<Class, String> {
        let version = self.take()?;
        let constant_pool: ConstantPool = self.take()?;
        let access_flags = self.take()?;

        let this_class_index: w2 = self.take()?;
        let this_class = constant_pool.get_class_name(this_class_index)?;

        let super_class_index: w2 = self.take()?;
        let super_class = if super_class_index == 0 {
            Option::None
        } else {
            Option::Some(constant_pool.get_class_name(super_class_index)?)
        };
        Ok(Class {
            version,
            constant_pool,
            access_flags,
            this_class,
            super_class
        })
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.version)?;
        writeln!(f, "{}", self.constant_pool)?;
        writeln!(f, "{:?}", self.access_flags)?; // TODO
        writeln!(f, "Class: {}", self.this_class)?;
        writeln!(f, "Superclass: {}", self.super_class.as_ref().unwrap_or(&"None".to_string()))?;
        write!(f, "")
    }
}
