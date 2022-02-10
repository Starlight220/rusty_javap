use std::fmt::{Display, Formatter};
use crate::access::MethodAccessModifier;
use crate::attributes::{Attribute, UnresolvedAttribute};
use crate::{ByteReader, Take, Unresolved, w2};
use crate::constant_pool::ConstantPool;
use crate::container;

pub struct UnresolvedMethod {
    access_flags: Vec<MethodAccessModifier>,
    name_index: w2,
    descriptor_index: w2,
    // TODO: add a descriptor struct?
    attributes: Vec<UnresolvedAttribute>,
}

impl Take<UnresolvedMethod> for ByteReader {
    fn take(&mut self) -> Result<UnresolvedMethod, String> {
        let access_flags = self.take()?;
        let name_index = self.take()?;
        let descriptor_index = self.take()?;
        let attributes = self.take()?;

        Ok(UnresolvedMethod {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}

impl Take<Vec<UnresolvedMethod>> for ByteReader {
    fn take(&mut self) -> Result<Vec<UnresolvedMethod>, String> {
        let field_count: w2 = self.take()?;
        let mut result = vec![];
        for _ in 0..field_count {
            result.push(self.take()?);
        }
        Ok(result)
    }
}

impl Unresolved for UnresolvedMethod {
    type Resolved = Method;
    type NeededToResolve = ConstantPool;

    fn resolve(self, constant_pool: &Self::NeededToResolve) -> Result<Self::Resolved, String> {
        Ok(Method {
            access_flags: self.access_flags,
            name: constant_pool.get_utf8(self.name_index)?,
            descriptor: constant_pool.get_utf8(self.descriptor_index)?,
            attributes: self.attributes.resolve(constant_pool)?,
        })
    }
}

#[derive(Debug)]
pub struct Method {
    access_flags: Vec<MethodAccessModifier>,
    name: String,
    descriptor: String, // TODO: add a descriptor struct?
    attributes: Vec<Attribute>,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\t{}:", self.name)?;
        writeln!(f, "\t\tDescriptor:\t{}", self.descriptor)?;
        writeln!(f, "\t\tAccess:\t{:?}", self.access_flags)?;
        writeln!(f, "\t\tAttributes:\t{:?}", self.attributes)?;
        write!(f, "")
    }
}

container!(Methods, Method);
