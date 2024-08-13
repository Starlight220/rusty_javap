use crate::bytecode::reader::{ByteReader, Take};
use crate::bytecode::unresolved::Unresolved;
use crate::bytecode::writer::{ByteWriter, Writeable};
use crate::constant_pool::{Constant, ConstantPool, CpInfo, CpTag};
use crate::model::attrs;
use crate::model::attrs::code;
use crate::model::attrs::local_variable_table::{
    parse_local_variable_table, write_local_variable_table,
};
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
            stringify!(LineNumberTable) => LineNumberTable(
                bytes
                    .take()
                    .map_err(|e| format!("Failed parsing line number table: {}", e))?,
            ),
            stringify!(LocalVariableTable) => {
                LocalVariableTable(parse_local_variable_table(&mut bytes, constant_pool)?)
            }
            stringify!(Code) => {
                let max_stack: w2 = bytes.take()?;
                let max_locals: w2 = bytes.take()?;

                let code_length: w4 = bytes.take()?;
                let mut code_reader =
                    ByteReader::from(bytes.take_bytes(code_length as usize)?.to_vec());
                let mut code: Vec<code::OpcodeInfo> = vec![];
                while !code_reader.is_empty() {
                    code.push(code_reader.take()?);
                }

                let exception_table_length: w2 = bytes.take()?;
                let mut exception_table: Vec<code::ExceptionTableElement> = vec![];
                for _ in 0..exception_table_length {
                    let start_pc: w2 = bytes.take()?;
                    let end_pc: w2 = bytes.take()?;
                    let handler_pc: w2 = bytes.take()?;

                    let catch_type_index: w2 = bytes.take()?;
                    let catch_type = if catch_type_index == 0 {
                        Option::None
                    } else {
                        Option::Some(constant_pool.get_class_name(catch_type_index).map_err(
                            |e| {
                                format!(
                                    "Failed finding exception class name at index {}: {}",
                                    catch_type_index, e
                                )
                            },
                        )?)
                    };
                    exception_table.push(code::ExceptionTableElement {
                        start_pc,
                        end_pc,
                        handler_pc,
                        catch_type,
                    });
                }

                let unresolved_attributes: Vec<UnresolvedAttribute> = bytes.take()?;
                let attributes = unresolved_attributes.resolve(constant_pool)?;

                Code(code::Code {
                    max_stack,
                    max_locals,
                    code,
                    exception_table,
                    attributes,
                })
            }
            stringify!(MethodParameters) => {
                let parameters_count: w1 = bytes.take()?;
                let mut method_parameters: Vec<MethodParameter> =
                    Vec::with_capacity(parameters_count.into());
                for i in 0..parameters_count {
                    let name_index: w2 = bytes.take()?;
                    let name: Option<String> = if name_index == 0 {
                        Option::None
                    } else {
                        Some(constant_pool.get_utf8(name_index).map_err(|e| {
                            format!(
                                "Couldn't get name for parameter #{} from index {}:\n\t{}",
                                i, name_index, e
                            )
                        })?)
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
            stringify!(SourceFile) => (|| {
                Result::<Attribute, String>::Ok(SourceFile(constant_pool.get_utf8(bytes.take()?)?))
            })()
            .map_err(|e| format!("Couldn't get source file name:\n\t{}", e))?,

            &_ => UNIMPLEMENTED_ATTRIBUTE_TODO {
                name,
                info: bytes.deplete(),
            },
        })
    }
    fn name(&self) -> String {
        match self {
            Attribute::ConstantValue(_) => stringify!(ConstantValue).to_string(),
            Attribute::Code(_) => stringify!(Code).to_string(),
            Attribute::SourceFile(_) => stringify!(SourceFile).to_string(),
            Attribute::LocalVariableTable(_) => stringify!(LocalVariableTable).to_string(),
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
        .map_err(|e| format!("Attribute resolution failure:\n {}", e))
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
                        let string_index =
                            constant_pool.push(Constant(CpTag::Utf8, CpInfo::Utf8 { string }));
                        Constant(CpTag::String, CpInfo::String { string_index })
                    }
                };
                constant_pool.push(constant).to_be_bytes().to_vec()
            }
            Attribute::LineNumberTable(line_number_table) => {
                let mut writer = ByteWriter::new();
                writer.write(line_number_table);
                writer.into()
            }
            Attribute::LocalVariableTable(line_number_table) => {
                write_local_variable_table(line_number_table, constant_pool)
            }
            Attribute::Code(code::Code {
                max_stack,
                max_locals,
                code,
                exception_table,
                attributes,
            }) => {
                let mut writer = ByteWriter::new();
                writer.write(max_stack);
                writer.write(max_locals);
                let code_bytes: Vec<w1> = {
                    let mut code_writer = ByteWriter::new();
                    for opcode in code {
                        code_writer.write(opcode);
                    }
                    code_writer.into()
                };
                let code_length = code_bytes.len() as w4;
                writer.write(code_length);
                for byte in code_bytes {
                    writer.write_byte(byte);
                }
                writer.write(exception_table.len() as w2);
                for code::ExceptionTableElement {
                    start_pc,
                    end_pc,
                    handler_pc,
                    catch_type,
                } in exception_table
                {
                    writer.write(start_pc);
                    writer.write(end_pc);
                    writer.write(handler_pc);

                    let catch_type_index: w2 = match catch_type {
                        Option::None => 0,
                        Option::Some(exception_type_name) => {
                            let class_name_index = constant_pool.push(Constant(
                                CpTag::Utf8,
                                CpInfo::Utf8 {
                                    string: exception_type_name,
                                },
                            ));
                            constant_pool.push(Constant(
                                CpTag::Class,
                                CpInfo::Class {
                                    name_index: class_name_index,
                                },
                            ))
                        }
                    };
                    writer.write(catch_type_index);
                }
                let unresolved_attributes: Vec<UnresolvedAttribute> =
                    Unresolved::unresolve(attributes, constant_pool);
                writer.write(unresolved_attributes);

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
