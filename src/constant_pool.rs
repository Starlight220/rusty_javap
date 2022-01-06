use crate::{w1, w2, w4, ByteReader};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

// TODO: replace discriminators with fields
#[derive(Debug)]
enum CpTag {
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

struct InvalidCpTag;

impl CpTag {
    #[allow(unused)]
    fn arg_byte_count(&self) -> usize {
        use CpTag::*;
        return match self {
            Utf8 => {
                let length: usize = 2;
                let bytes: usize = 1 * length;
                length + bytes
            }

            Integer | Float => {
                let bytes: usize = 4;
                bytes
            }

            Long | Double => {
                let high_bytes: usize = 4;
                let low_bytes: usize = 4;
                high_bytes + low_bytes
            }

            Class => {
                let name_index: usize = 2;
                name_index
            }

            String => {
                let string_index: usize = 2;
                string_index
            }

            Fieldref | Methodref | InterfaceMethodref => {
                let class_index: usize = 2;
                let name_and_type_index: usize = 2;
                class_index + name_and_type_index
            }

            NameAndType => {
                let name_index: usize = 2;
                let descriptor_index: usize = 2;
                name_index + descriptor_index
            }

            MethodHandle => {
                let reference_kind: usize = 1;
                let reference_index: usize = 2;
                reference_kind + reference_index
            }

            MethodType => {
                let descriptor_index: usize = 2;
                descriptor_index
            }

            Dynamic | InvokeDynamic => {
                let bootstrap_method_attr_index: usize = 2;
                let name_and_type_index: usize = 2;
                bootstrap_method_attr_index + name_and_type_index
            }

            Module | Package => {
                let name_index: usize = 2;
                name_index
            }
        };
    }
}

#[derive(Debug)]
enum CpInfo {
    /// string (usually referenced by other constants)
    Utf8 { length: w2, bytes: Vec<w1> }, // FIXME: find a different type for bytes
    /// 4-byte int
    Integer { bytes: w4 },
    /// 4-byte float
    Float { bytes: w4 },
    /// 8-byte long
    Long { high_bytes: w4, low_bytes: w4 },
    /// 8-byte double
    Double { high_bytes: w4, low_bytes: w4 },
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
    fn of(tag: &CpTag, bytes: &mut ByteReader) -> CpInfo {
        use CpInfo::*;
        return match tag {
            CpTag::Package => Package {
                name_index: bytes.take::<w2>(),
            },
            CpTag::Utf8 => {
                let length: w2 = bytes.take();
                let mut vec: Vec<w1> = vec![];
                for _ in 0..length {
                    vec.push(bytes.take::<w1>())
                }
                Utf8 { length, bytes: vec }
            }
            CpTag::Integer => Integer {
                bytes: bytes.take::<w4>(),
            },
            CpTag::Float => Float {
                bytes: bytes.take::<w4>(),
            },
            CpTag::Long => {
                let high_bytes: w4 = bytes.take();
                let low_bytes: w4 = bytes.take();
                Long {
                    high_bytes,
                    low_bytes,
                }
            }
            CpTag::Double => {
                let high_bytes: w4 = bytes.take();
                let low_bytes: w4 = bytes.take();
                Double {
                    high_bytes,
                    low_bytes,
                }
            }
            CpTag::Class => Class {
                name_index: bytes.take::<w2>(),
            },
            CpTag::String => String {
                string_index: bytes.take::<w2>(),
            },
            CpTag::Fieldref => {
                let class_index: w2 = bytes.take();
                let name_and_type_index: w2 = bytes.take();
                Fieldref {
                    class_index,
                    name_and_type_index,
                }
            }
            CpTag::Methodref => {
                let class_index: w2 = bytes.take();
                let name_and_type_index: w2 = bytes.take();
                Methodref {
                    class_index,
                    name_and_type_index,
                }
            }
            CpTag::InterfaceMethodref => {
                let class_index: w2 = bytes.take();
                let name_and_type_index: w2 = bytes.take();
                InterfaceMethodref {
                    class_index,
                    name_and_type_index,
                }
            }
            CpTag::NameAndType => {
                let name_index: w2 = bytes.take();
                let descriptor_index: w2 = bytes.take();
                NameAndType {
                    name_index,
                    descriptor_index,
                }
            }
            CpTag::MethodHandle => {
                let reference_kind: w1 = bytes.take();
                let reference_index: w2 = bytes.take();
                MethodHandle {
                    reference_kind,
                    reference_index,
                }
            }
            CpTag::MethodType => MethodType {
                descriptor_index: bytes.take(),
            },
            CpTag::Dynamic => {
                let bootstrap_method_attr_index: w2 = bytes.take();
                let name_and_type_index: w2 = bytes.take();
                Dynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                }
            }
            CpTag::InvokeDynamic => {
                let bootstrap_method_attr_index: w2 = bytes.take();
                let name_and_type_index: w2 = bytes.take();
                InvokeDynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                }
            }
            CpTag::Module => Module {
                name_index: bytes.take::<w2>(),
            },
        };
    }
}

struct Constant(CpTag, CpInfo);

impl TryFrom<w1> for CpTag {
    type Error = InvalidCpTag;

    fn try_from(i: w1) -> Result<Self, InvalidCpTag> {
        use CpTag::*;
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
            _ => Err(InvalidCpTag),
        }
    }
}
impl Display for CpTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for CpInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // FIXME: provide proper to_string impl
        use CpInfo::*;
        let string = match self {
            Utf8 {
                length: _length,
                bytes,
            } => std::string::String::from_utf8_lossy(bytes.as_slice()).to_string(),
            Integer { bytes } => {
                format!("{}", bytes)
            }
            it @ Float { .. } => {
                format!("{:?}", it) // FIXME
            }
            Long {
                high_bytes,
                low_bytes,
            } => {
                format!("{}L", ((*high_bytes as u64) << 32) + *low_bytes as u64)
            }
            it @ Double { .. } => {
                format!("{:?}", it) // FIXME
            }
            Class { name_index } => {
                format!("#{}", name_index)
            }
            String { string_index } => {
                format!("#{}", string_index)
            }
            Fieldref {
                class_index,
                name_and_type_index,
            } => {
                format!("#{}.#{}", class_index, name_and_type_index)
            }
            Methodref {
                class_index,
                name_and_type_index,
            } => {
                format!("#{}.#{}", class_index, name_and_type_index)
            }
            InterfaceMethodref {
                class_index,
                name_and_type_index,
            } => {
                format!("#{}.#{}", class_index, name_and_type_index)
            }
            NameAndType {
                name_index,
                descriptor_index,
            } => {
                format!("#{}:#{}", name_index, descriptor_index)
            }
            it @ MethodHandle { .. } => {
                format!("{:?}", it) // FIXME
            }
            MethodType { descriptor_index } => {
                format!("#{}", descriptor_index)
            }
            it @ Dynamic { .. } => {
                format!("{:?}", it) // FIXME
            }
            it @ InvokeDynamic { .. } => {
                format!("{:?}", it) // FIXME
            }
            Module { name_index } => {
                format!("#{}", name_index)
            }
            Package { name_index } => {
                format!("#{}", name_index)
            }
        };
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

pub fn read_constants(bytes: &mut ByteReader) {
    let constants_pool_count: w2 = bytes.take();
    println!(
        "\
        Constant Pool [{constant_pool_count}]:\n\
        ",
        constant_pool_count = constants_pool_count
    );
    for offset in 1..(constants_pool_count) {
        let tag = CpTag::try_from(bytes.take::<w1>()).ok().unwrap();
        let info = CpInfo::of(&tag, bytes);
        let constant = Constant(tag, info);

        println!(
            "\t#{offset} = {constant}",
            offset = offset,
            constant = constant
        );
    }
}
