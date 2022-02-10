use crate::Take;
use crate::{w1, w2, w4, w8, ByteReader};
use std::fmt::{Display, Formatter};
use std::ops::Index;

// TODO: replace discriminators with fields
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum CpTag {
    /// string (usually referenced by other constants)
    Utf8 = 1,
    /// 4-byte int
    Integer = 3,
    /// 4-byte float
    Float = 4,
    /// 8-byte long
    Long = 5,
    /// 8-byte double
    Double = 6,
    /// class
    Class = 7,
    /// String object
    String = 8,
    /// field
    Fieldref = 9,
    /// instance method (FIXME: make sure that statics don't count)
    Methodref = 10,
    /// interface method (FIXME: what about statics here?)
    InterfaceMethodref = 11,
    /// field/method signature
    NameAndType = 12,
    /// method handle (FIXME: find out what this means)
    MethodHandle = 15,
    /// method type (FIXME: find out what this means)
    MethodType = 16,
    /// dynamically-computed constant
    Dynamic = 17,
    /// dynamically-computed callsite
    InvokeDynamic = 18,
    /// module
    Module = 19,
    /// package
    Package = 20,
}

#[derive(Debug)]
pub enum CpInfo {
    /// string (usually referenced by other constants)
    Utf8 { string: String }, // FIXME: find a different type for bytes
    /// 4-byte int
    Integer { int: w4 },
    /// 4-byte float
    Float { float: f32 },
    /// 8-byte long
    Long { long: w8 },
    /// 8-byte double
    Double { double: f64 },
    /// class
    Class { name_index: w2 },
    /// String object
    String { string_index: w2 },
    /// field
    Fieldref {
        class_index: w2,
        name_and_type_index: w2,
    },
    /// instance method (FIXME: make sure that statics don't count)
    Methodref {
        class_index: w2,
        name_and_type_index: w2,
    },
    /// interface method (FIXME: what about statics here?)
    InterfaceMethodref {
        class_index: w2,
        name_and_type_index: w2,
    },
    /// field/method signature
    NameAndType {
        name_index: w2,
        descriptor_index: w2,
    },
    /// method handle (FIXME: find out what this means)
    MethodHandle {
        reference_kind: w1,
        reference_index: w2,
    }, // FIXME: enum for kind
    /// method type (FIXME: find out what this means)
    MethodType { descriptor_index: w2 },
    /// dynamically-computed constant
    Dynamic {
        bootstrap_method_attr_index: w2,
        name_and_type_index: w2,
    },
    /// dynamically-computed callsite
    InvokeDynamic {
        bootstrap_method_attr_index: w2,
        name_and_type_index: w2,
    },
    /// module
    Module { name_index: w2 },
    /// package
    Package { name_index: w2 },
}

impl CpInfo {
    fn of(tag: &CpTag, bytes: &mut ByteReader) -> Result<CpInfo, String> {
        use CpInfo::*;
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

    fn content_to_string(&self) -> String {
        use CpInfo::*;
        match self {
            Utf8 { string } => string.to_owned(),
            Integer { int: bytes } => format!("{}", bytes),
            Float { float } => format!("{}f", float),
            Long { long } => format!("{}L", long),
            Double { double } => format!("{}", double),
            Class { name_index } => format!("#{}", name_index),
            String { string_index } => format!("#{}", string_index),
            Fieldref {
                class_index,
                name_and_type_index,
            } => format!("#{}.#{}", class_index, name_and_type_index),
            Methodref {
                class_index,
                name_and_type_index,
            } => format!("#{}.#{}", class_index, name_and_type_index),
            InterfaceMethodref {
                class_index,
                name_and_type_index,
            } => format!("#{}.#{}", class_index, name_and_type_index),
            NameAndType {
                name_index,
                descriptor_index,
            } => format!("#{}:#{}", name_index, descriptor_index),
            it @ MethodHandle { .. } => format!("{:?}", it),
            MethodType { descriptor_index } => format!("#{}", descriptor_index),
            it @ Dynamic { .. } => format!("{:?}", it),
            it @ InvokeDynamic { .. } => format!("{:?}", it),
            Module { name_index } => format!("#{}", name_index),
            Package { name_index } => format!("#{}", name_index),
        }
    }
}

#[derive(Debug)]
pub struct Constant(pub CpTag, pub CpInfo);

impl Display for CpTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for CpInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // FIXME: provide proper to_string impl
        let string = self.content_to_string();
        write!(f, "{}", string)
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ref tag = self.0;
        let ref info = self.1;
        f.write_str(
            format!(
                "{tag}\t{info}",
                tag = tag.to_string(),
                info = info.to_string()
            )
            .as_str(),
        )
    }
}

impl Take<CpTag> for ByteReader {
    fn take(&mut self) -> Result<CpTag, String> {
        use CpTag::*;
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

#[derive(Debug)]
pub struct ConstantPool {
    pool: Vec<Option<Constant>>,
}

impl Index<w2> for ConstantPool {
    type Output = Option<Constant>;

    fn index(&self, index: w2) -> &Self::Output {
        &self.pool[index as usize]
    }
}

impl ConstantPool {
    fn new() -> ConstantPool {
        ConstantPool {
            pool: vec![Option::None],
        }
    }

    pub fn get_class_name(&self, class_index: w2) -> Result<String, String> {
        let index = match self[class_index]
            .as_ref()
            .ok_or(format!("Invalid index: {}", class_index))?
        {
            Constant(CpTag::Class, CpInfo::Class { name_index }) => *name_index,
            Constant(tag, _) => {
                return Err(format!(
                    "Wrong constant type at index {idx}: expected `Class`, found `{found}`",
                    idx = class_index,
                    found = tag
                ))
            }
        };
        self.get_utf8(index)
    }

    pub fn get_utf8(&self, index: w2) -> Result<String, String> {
        match self[index]
            .as_ref()
            .ok_or(format!("Invalid index: {}", index))?
        {
            Constant(CpTag::Utf8, CpInfo::Utf8 { string }) => Ok(string.to_owned()),
            Constant(tag, _) => {
                return Err(format!(
                    "Wrong constant type at index {idx}: expected `Utf8`, found `{found}`",
                    idx = index,
                    found = tag
                ))
            }
        }
    }

    pub fn get_constant_as_string(&self, index: w2) -> Result<String, String> {
        match self[index]
            .as_ref()
            .ok_or(format!("Invalid index: {}", index))?
        {
            Constant(CpTag::String, CpInfo::String { string_index }) => {
                Ok(format!("\"{}\"", self.get_utf8(*string_index)?))
            }
            Constant(CpTag::Double | CpTag::Integer | CpTag::Float | CpTag::Long, it) => {
                Ok(it.content_to_string())
            }
            Constant(tag, _) => {
                return Err(format!(
                    "Wrong constant type at index {idx}: expected {expected}, found `{found}`",
                    idx = index,
                    expected = stringify!([Integer, Float, Long, Double, String]),
                    found = tag
                ))
            }
        }
    }

    fn push_empty(&mut self) {
        self.pool.push(Option::None)
    }

    fn push(&mut self, constant: Constant) {
        self.pool.push(Option::Some(constant))
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

impl Display for ConstantPool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Constant Pool [{count}]:", count = self.pool.len())?;
        for i in 1..self.pool.len() {
            match &self.pool[i] {
                None => {}
                Some(Constant(_, info)) => {
                    writeln!(f, "\t#{offset} = {constant}", offset = i, constant = info)?;
                }
            }
        }
        writeln!(f, "")
    }
}

mod double_utils {
    use crate::{w4, w8};

    pub(crate) fn long(high_bytes: w4, low_bytes: w4) -> w8 {
        ((high_bytes as w8) << 32) + low_bytes as u64
    }
    pub(crate) fn double(high_bytes: w4, low_bytes: w4) -> f64 {
        f64::from_bits(long(high_bytes, low_bytes))
    }
}
