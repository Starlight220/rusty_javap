use crate::access::AccessModifier;
use crate::constant_pool::Constant;
use crate::versions::Version;
use crate::{ByteReader, Take};

#[derive(Debug)]
pub struct Class {
    version: Version,
    constant_pool: Vec<Constant>,
    access_flags: Vec<AccessModifier>,
}

impl Take<Class> for ByteReader {
    fn take(&mut self) -> Result<Class, String> {
        Ok(Class {
            version: self.take()?,
            constant_pool: self.take()?,
            access_flags: self.take()?,
        })
    }
}
