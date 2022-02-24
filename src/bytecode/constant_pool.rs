use core::result::Result;
use core::result::Result::Ok;
use crate::bytecode::reader::{ByteReader, Take};
use crate::constant_pool::{Constant, ConstantPool, CpInfo, CpTag};
use crate::typedefs::{w1, w2};

impl Take<CpTag> for ByteReader {
    fn take(&mut self) -> Result<CpTag, String> {
        use CpTag::{Class, Double, Dynamic, Fieldref, Float, Integer, InterfaceMethodref, InvokeDynamic, Long, MethodHandle, Methodref, MethodType, Module, NameAndType, Package, String, Utf8};
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

impl CpInfo {
    fn of(tag: &CpTag, bytes: &mut ByteReader) -> Result<CpInfo, String> {
        use crate::constant_pool::CpInfo::*;
        use crate::constant_pool::double_utils;
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
