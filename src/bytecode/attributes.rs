use crate::bytecode::reader::{ByteReader, Take};
use crate::bytecode::unresolved::Unresolved;
use crate::bytecode::writer::{ByteWriter, Writeable};
use crate::constant_pool::{Constant, ConstantPool, CpInfo, CpTag};
use crate::model::attrs;
use crate::model::attrs::line_number_table::LineNumberTableElement;
use crate::model::attrs::method_parameters::{MethodParameter, MethodParameterAccessFlags};
use crate::model::attrs::Attribute;
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
            stringify!(LineNumberTable) => {
                let line_number_table_length: w2 = bytes.take()?;
                let mut line_number_table: Vec<LineNumberTableElement> =
                    Vec::with_capacity(line_number_table_length.into());
                for _ in 0..line_number_table_length {
                    let start_pc: w2 = bytes.take()?;
                    let line_number: w2 = bytes.take()?;
                    line_number_table.push(LineNumberTableElement {
                        start_pc,
                        line_number,
                    });
                }
                LineNumberTable(line_number_table)
            }
            stringify!(MethodParameters) => {
                let parameters_count: w1 = bytes.take()?;
                let mut method_parameters: Vec<MethodParameter> =
                    Vec::with_capacity(parameters_count.into());
                for _ in 0..parameters_count {
                    let name_index: w2 = bytes.take()?;
                    let name: Option<String> = if name_index == 0 {
                        Option::None
                    } else {
                        Some(constant_pool.get_utf8(name_index)?)
                    };
                    let access_flags: Vec<MethodParameterAccessFlags> = bytes.take()?;
                    method_parameters.push(MethodParameter { name, access_flags })
                }
                MethodParameters(method_parameters)
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
            Attribute::LineNumberTable(_) => stringify!(LineNumberTable).to_string(),
            Attribute::Synthetic => stringify!(Synthetic).to_string(),
            Attribute::Deprecated => stringify!(Deprecated).to_string(),
            Attribute::Signature { .. } => stringify!(Signature).to_string(),
            Attribute::MethodParameters { .. } => stringify!(MethodParameters).to_string(),
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
        Attribute::create(
            constant_pool.get_utf8(self.name_index)?,
            self.info,
            constant_pool,
        )
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
            Attribute::LineNumberTable(line_number_table) => {
                let mut writer = ByteWriter::new();
                writer.write(line_number_table.len() as w2);
                for LineNumberTableElement {
                    start_pc,
                    line_number,
                } in line_number_table
                {
                    writer.write(start_pc);
                    writer.write(line_number);
                }

                writer.into()
            }
            Attribute::MethodParameters(method_parameters) => {
                let mut writer = ByteWriter::new();
                writer.write(method_parameters.len() as w1);
                for MethodParameter { name, access_flags } in method_parameters {
                    let name_index: w2 = name.map_or(0, |name| {
                        constant_pool.push(Constant(CpTag::Utf8, CpInfo::Utf8 { string: name }))
                    });
                    writer.write(name_index);
                    writer.write(access_flags);
                }

                writer.into()
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
