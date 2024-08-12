use crate::bytecode::attributes::UnresolvedAttribute;
use crate::bytecode::reader::{ByteReader, Take};
use crate::bytecode::unresolved::Unresolved;
use crate::bytecode::writer::{ByteWriter, Writeable};
use crate::constant_pool::{Constant, ConstantPool, CpInfo, CpTag};
use crate::model::field::{Field, FieldAccessModifier};
use crate::w2;

pub struct UnresolvedField {
    access_flags: Vec<FieldAccessModifier>,
    name_index: w2,
    descriptor_index: w2,
    // TODO: add a descriptor struct?
    attributes: Vec<UnresolvedAttribute>,
}

impl Take<Vec<UnresolvedField>> for ByteReader {
    fn take(&mut self) -> Result<Vec<UnresolvedField>, String> {
        let field_count: w2 = self.take()?;
        let mut result = vec![];
        for _ in 0..field_count {
            result.push(self.take()?);
        }
        Ok(result)
    }
}

impl Take<UnresolvedField> for ByteReader {
    fn take(&mut self) -> Result<UnresolvedField, String> {
        let access_flags = self.take()?;
        let name_index = self.take()?;
        let descriptor_index = self.take()?;
        let attributes = self.take()?;

        Ok(UnresolvedField {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}

impl Writeable for Vec<UnresolvedField> {
    fn write(self, writer: &mut ByteWriter) {
        writer.write(self.len() as w2);
        for method in self {
            writer.write(method.access_flags);
            writer.write(method.name_index);
            writer.write(method.descriptor_index);
            writer.write(method.attributes);
        }
    }
}

impl Unresolved for UnresolvedField {
    type Resolved = Field;
    type NeededToResolve = ConstantPool;

    fn resolve(self, constant_pool: &Self::NeededToResolve) -> Result<Self::Resolved, String> {
        Ok(Field::new(
            self.access_flags,
            constant_pool.get_utf8(self.name_index)?,
            constant_pool.get_utf8(self.descriptor_index)?,
            self.attributes.resolve(constant_pool)?,
        ))
    }
    fn unresolve(resolved: Self::Resolved, constant_pool: &mut Self::NeededToResolve) -> Self {
        constant_pool.push(Constant(
            CpTag::Utf8,
            CpInfo::Utf8 {
                string: resolved.name,
            },
        ));
        let name_index = constant_pool.len();

        constant_pool.push(Constant(
            CpTag::Utf8,
            CpInfo::Utf8 {
                string: resolved.descriptor,
            },
        ));
        let descriptor_index = constant_pool.len();

        Self {
            access_flags: resolved.access_flags,
            name_index: name_index as w2,
            descriptor_index: descriptor_index as w2,
            attributes: Unresolved::unresolve(resolved.attributes, constant_pool),
        }
    }
}
