use crate::bytecode::reader::{ByteReader, Take};
use crate::constant_pool::{double_utils, Constant, ConstantPool, CpInfo, CpTag};
use crate::typedefs::{w1, w2};
use core::result::Result;
use core::result::Result::Ok;

use super::writer::{ByteWriter, Writeable};

impl Take<CpTag> for ByteReader {
    fn take(&mut self) -> Result<CpTag, String> {
        use CpTag::{
            Class, Double, Dynamic, Fieldref, Float, Integer, InterfaceMethodref, InvokeDynamic,
            Long, MethodHandle, MethodType, Methodref, Module, NameAndType, Package, String, Utf8,
        };
        let i: w1 = self.take()?;
        match i {
            1 => Ok(Utf8),
            3 => Ok(Integer),
            4 => Ok(Float),
            5 => Ok(Long),
            6 => Ok(Double),
            7 => Ok(Class),
            8 => Ok(String),
            9 => Ok(Fieldref),
            10 => Ok(Methodref),
            11 => Ok(InterfaceMethodref),
            12 => Ok(NameAndType),
            15 => Ok(MethodHandle),
            16 => Ok(MethodType),
            17 => Ok(Dynamic),
            18 => Ok(InvokeDynamic),
            19 => Ok(Module),
            20 => Ok(Package),
            it @ _ => Err(format!("Unexpected Constant type ID `{it}`!", it = it)),
        }
    }
}

impl Writeable for CpTag {
    fn write(self, writer: &mut ByteWriter) {
        use CpTag::{
            Class, Double, Dynamic, Fieldref, Float, Integer, InterfaceMethodref, InvokeDynamic,
            Long, MethodHandle, MethodType, Methodref, Module, NameAndType, Package, String, Utf8,
        };
        let byte: w1 = match self {
            Utf8 => 1u8,
            Integer => 3u8,
            Float => 4u8,
            Long => 5u8,
            Double => 6u8,
            Class => 7u8,
            String => 8u8,
            Fieldref => 9u8,
            Methodref => 10u8,
            InterfaceMethodref => 11u8,
            NameAndType => 12u8,
            MethodHandle => 15u8,
            MethodType => 16u8,
            Dynamic => 17u8,
            InvokeDynamic => 18u8,
            Module => 19u8,
            Package => 20u8,
        };
        writer.write(byte)
    }
}

impl Take<ConstantPool> for ByteReader {
    fn take(&mut self) -> Result<ConstantPool, String> {
        let constants_pool_count: w2 = self.take()?;
        let mut pool = ConstantPool::new();
        let mut skip: bool = false;
        for _offset in 1..(constants_pool_count) {
            if skip {
                skip = false;
                pool.push_empty();
                continue;
            }
            let tag = self.take()?;

            // Long and Double "swallow" another index
            skip = match tag {
                CpTag::Double | CpTag::Long => true,
                _ => false,
            };

            let info = CpInfo::of(&tag, self)?;
            let constant = Constant(tag, info);
            pool.push(constant);
        }
        Ok(pool)
    }
}
impl Writeable for ConstantPool {
    fn write(self, writer: &mut ByteWriter) {
        let mut inner_writer: ByteWriter = ByteWriter::new();
        for index in 0..self.len() {
            match &self[index as w2] {
                Option::None => {}
                Option::Some(Constant(tag, info)) => {
                    inner_writer.write(*tag);
                    inner_writer.write(info.clone());
                }
            }
        }
        let constant_pool_bytes: Vec<w1> = inner_writer.into();
        writer.write(self.len() as w2);
        for byte in constant_pool_bytes {
            writer.write_byte(byte);
        }
    }
}

impl Writeable for CpInfo {
    fn write(self, writer: &mut ByteWriter) {
        match self {
            CpInfo::Package { name_index } => writer.write(name_index),
            CpInfo::Utf8 { string } => {
                writer.write(string.len() as w2);
                for byte in string.bytes() {
                    writer.write(byte);
                }
            }
            CpInfo::Integer { int } => writer.write(int),
            CpInfo::Float { float } => writer.write(f32::to_bits(float)),
            CpInfo::Long { long } => {
                let (high, low) = double_utils::long2bytes(long);
                writer.write(high);
                writer.write(low);
            }
            CpInfo::Double { double } => {
                let (high, low) = double_utils::double2bytes(double);
                writer.write(high);
                writer.write(low);
            }
            CpInfo::Class { name_index } => writer.write(name_index),
            CpInfo::String { string_index } => writer.write(string_index),
            CpInfo::Fieldref {
                class_index,
                name_and_type_index,
            } => {
                writer.write(class_index);
                writer.write(name_and_type_index);
            }
            CpInfo::Methodref {
                class_index,
                name_and_type_index,
            } => {
                writer.write(class_index);
                writer.write(name_and_type_index);
            }
            CpInfo::InterfaceMethodref {
                class_index,
                name_and_type_index,
            } => {
                writer.write(class_index);
                writer.write(name_and_type_index);
            }
            CpInfo::NameAndType {
                name_index,
                descriptor_index,
            } => {
                writer.write(name_index);
                writer.write(descriptor_index);
            }
            CpInfo::MethodHandle {
                reference_kind,
                reference_index,
            } => {
                writer.write(reference_kind);
                writer.write(reference_index);
            }
            CpInfo::MethodType { descriptor_index } => writer.write(descriptor_index),
            CpInfo::Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                writer.write(bootstrap_method_attr_index);
                writer.write(name_and_type_index);
            }
            CpInfo::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                writer.write(bootstrap_method_attr_index);
                writer.write(name_and_type_index);
            }
            CpInfo::Module { name_index } => writer.write(name_index),
        }
    }
}

impl CpInfo {
    fn of(tag: &CpTag, bytes: &mut ByteReader) -> Result<CpInfo, String> {
        use crate::constant_pool::double_utils;
        use crate::constant_pool::CpInfo::*;
        return Ok(match tag {
            CpTag::Package => Package {
                name_index: bytes.take()?,
            },
            CpTag::Utf8 => {
                let length: w2 = bytes.take()?;
                let mut vec: Vec<w1> = vec![];
                for _ in 0..length {
                    vec.push(bytes.take()?)
                }
                Utf8 {
                    string: std::string::String::from_utf8_lossy(vec.as_slice()).to_string(),
                }
            }
            CpTag::Integer => Integer { int: bytes.take()? },
            CpTag::Float => Float {
                float: f32::from_bits(bytes.take()?),
            },
            CpTag::Long => Long {
                long: double_utils::long(bytes.take()?, bytes.take()?),
            },
            CpTag::Double => Double {
                double: double_utils::double(bytes.take()?, bytes.take()?),
            },
            CpTag::Class => Class {
                name_index: bytes.take()?,
            },
            CpTag::String => String {
                string_index: bytes.take()?,
            },
            CpTag::Fieldref => Fieldref {
                class_index: bytes.take()?,
                name_and_type_index: bytes.take()?,
            },
            CpTag::Methodref => Methodref {
                class_index: bytes.take()?,
                name_and_type_index: bytes.take()?,
            },
            CpTag::InterfaceMethodref => InterfaceMethodref {
                class_index: bytes.take()?,
                name_and_type_index: bytes.take()?,
            },
            CpTag::NameAndType => NameAndType {
                name_index: bytes.take()?,
                descriptor_index: bytes.take()?,
            },
            CpTag::MethodHandle => MethodHandle {
                reference_kind: bytes.take()?,
                reference_index: bytes.take()?,
            },
            CpTag::MethodType => MethodType {
                descriptor_index: bytes.take()?,
            },
            CpTag::Dynamic => Dynamic {
                bootstrap_method_attr_index: bytes.take()?,
                name_and_type_index: bytes.take()?,
            },
            CpTag::InvokeDynamic => InvokeDynamic {
                bootstrap_method_attr_index: bytes.take()?,
                name_and_type_index: bytes.take()?,
            },
            CpTag::Module => Module {
                name_index: bytes.take()?,
            },
        });
    }
}
