use crate::bytecode::reader::{ByteReader, Take};
use crate::bytecode::unresolved::Unresolved;
use crate::bytecode::writer::{ByteWriter, Writeable};
use crate::constant_pool::{Constant, ConstantPool, CpInfo, CpTag};
use crate::model::attrs::{self, Attribute};
use crate::{model, w1, w2, w4};

impl Attribute {
    fn create(
        name: String,
        info: Vec<w1>,
        constant_pool: &ConstantPool,
    ) -> Result<Attribute, String> {
        let mut bytes = ByteReader::from(info);
        use Attribute::*;
        Ok(match name.as_str() {
            stringify!(ConstantValue) => {
                ConstantValue(model::attrs::constant_value::ConstantValue::String(
                    constant_pool.get_constant_as_string(bytes.take()?)?,
                ))
            }
            stringify!(Synthetic) => Synthetic,
            stringify!(Deprecated) => Deprecated,
            stringify!(Signature) => Signature {
                signature_index: bytes.take()?,
            },
            stringify!(SourceFile) => SourceFile(constant_pool.get_utf8(bytes.take()?)?),

            &_ => UNIMPLEMENTED_ATTRIBUTE_TODO {
                name,
                info: bytes.deplete(),
            },
        })
    }
    fn name(&self) -> String {
        match self {
            Attribute::ConstantValue(_) => stringify!(ConstantValue).to_string(),
            Attribute::SourceFile(_) => stringify!(SourceFile).to_string(),
            Attribute::Synthetic => stringify!(Synthetic).to_string(),
            Attribute::Deprecated => stringify!(Deprecated).to_string(),
            Attribute::Signature { .. } => stringify!(Signature).to_string(),
            Attribute::UNIMPLEMENTED_ATTRIBUTE_TODO { name, .. } => name.clone(),
        }
    }
}

#[derive(Debug)]
pub struct UnresolvedAttribute {
    name_index: w2,
    _attribute_length: w4,
    info: Vec<w1>,
}

impl Take<Vec<UnresolvedAttribute>> for ByteReader {
    fn take(&mut self) -> Result<Vec<UnresolvedAttribute>, String> {
        let attribute_count: w2 = self.take()?;
        let mut attributes = vec![];
        for _ in 0..attribute_count {
            let name_index = self.take()?;
            let attribute_length = self.take()?;
            let mut info = vec![];
            for _ in 0..attribute_length {
                info.push(self.take()?);
            }
            attributes.push(UnresolvedAttribute {
                name_index,
                _attribute_length: attribute_length,
                info,
            })
        }
        Ok(attributes)
    }
}

impl Writeable for Vec<UnresolvedAttribute> {
    fn write(self, writer: &mut ByteWriter) {
        writer.write(self.len() as w2);
        for UnresolvedAttribute {
            name_index,
            _attribute_length,
            info,
        } in self
        {
            writer.write(name_index);
            writer.write(_attribute_length);
            for byte in info {
                writer.write(byte);
            }
        }
    }
}

impl Unresolved for UnresolvedAttribute {
    type Resolved = Attribute;
    type NeededToResolve = ConstantPool;

    fn resolve(self, constant_pool: &Self::NeededToResolve) -> Result<Self::Resolved, String> {
        Ok(Attribute::create(
            constant_pool.get_utf8(self.name_index)?,
            self.info,
            constant_pool,
        )?)
    }

    fn unresolve(resolved: Self::Resolved, constant_pool: &mut Self::NeededToResolve) -> Self {
        let name_index = constant_pool.push(Constant(
            CpTag::Utf8,
            CpInfo::Utf8 {
                string: resolved.name(),
            },
        ));

        let info: Vec<w1> = match resolved {
            Attribute::ConstantValue(it) => {
                let constant = match it {
                    attrs::constant_value::ConstantValue::Integer(int) => {
                        Constant(CpTag::Integer, CpInfo::Integer { int })
                    }
                    attrs::constant_value::ConstantValue::Long(long) => {
                        Constant(CpTag::Long, CpInfo::Long { long })
                    }
                    attrs::constant_value::ConstantValue::Float(float) => {
                        Constant(CpTag::Float, CpInfo::Float { float })
                    }
                    attrs::constant_value::ConstantValue::Double(double) => {
                        Constant(CpTag::Double, CpInfo::Double { double })
                    }
                    attrs::constant_value::ConstantValue::String(string) => {
                        let string_index = constant_pool.push(Constant(CpTag::Utf8, CpInfo::Utf8 { string }));
                        Constant(CpTag::String, CpInfo::String { string_index })
                    }
                };
                constant_pool.push(constant).to_be_bytes().to_vec()
            }
            Attribute::SourceFile(source_file_name) => constant_pool
                .push(Constant(
                    CpTag::Utf8,
                    CpInfo::Utf8 {
                        string: source_file_name,
                    },
                ))
                .to_be_bytes()
                .to_vec(),
            Attribute::Synthetic => vec![],
            Attribute::Deprecated => vec![],
            Attribute::Signature { signature_index } => signature_index.to_be_bytes().to_vec(),
            Attribute::UNIMPLEMENTED_ATTRIBUTE_TODO { info, .. } => info.clone(),
        };
        Self {
            name_index,
            _attribute_length: info.len() as w4,
            info,
        }
    }
}
