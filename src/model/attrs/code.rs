use std::convert::{TryFrom, TryInto};
use crate::bytecode::reader::{ByteSize, Take};
use crate::bytecode::writer::{ByteWriter, Writeable};
use serde::{Deserialize, Serialize};
use crate::bytecode::reader::ByteReader;
use crate::model::attrs::Attribute;
use crate::{w1, w2, w4};
use crate::constant_pool::{Constant, ConstantPool, CpInfo, CpTag};

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    pub max_stack: w2,
    pub max_locals: w2,
    /// Len: w4
    pub code: Vec<OpcodeInfo>,
    /// Len: w2
    pub exception_table: Vec<exception_table::ExceptionTableElement>,
    /// Len: w2
    pub attributes: Vec<Attribute>,
}


pub mod exception_table {
    use crate::bytecode::reader::Take;
use serde::{Deserialize, Serialize};
    use crate::bytecode::reader::ByteReader;
    use crate::bytecode::writer::ByteWriter;
    use crate::constant_pool::{Constant, ConstantPool, CpInfo, CpTag};
    use crate::typedefs::w2;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ExceptionTableElement {
        pub start_pc: w2,
        pub end_pc: w2,
        pub handler_pc: w2,
        pub catch_type: Option<String>,
    }

    pub fn parse_exception_table(constant_pool: &ConstantPool, bytes: &mut ByteReader) -> Result<Vec<ExceptionTableElement>, String> {
        let exception_table_length: w2 = bytes.take()?;
        let mut exception_table: Vec<ExceptionTableElement> = vec![];
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
            exception_table.push(ExceptionTableElement {
                start_pc,
                end_pc,
                handler_pc,
                catch_type,
            });
        }
        Ok(exception_table)
    }

    pub fn write_exception_table(constant_pool: &mut ConstantPool, exception_table: Vec<ExceptionTableElement>, writer: &mut ByteWriter) {
        writer.write(exception_table.len() as w2);
        for ExceptionTableElement {
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
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassRef(pub String);
impl ClassRef {
    fn decode(bytes: &mut ByteReader, constant_pool: &ConstantPool) -> Result<ClassRef, String> {
        Ok(Self(constant_pool.get_class_name(bytes.take()?)?))
    }
    fn encode(self, constant_pool: &mut ConstantPool, writer: &mut ByteWriter) {
        let class_name_index =
            constant_pool.push(Constant(CpTag::Utf8, CpInfo::Utf8 { string: self.0 }));
        let class_index = constant_pool.push(Constant(
            CpTag::Class,
            CpInfo::Class {
                name_index: class_name_index,
            },
        ));
        writer.write(class_index);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldRef { pub class: ClassRef, pub name: String, pub descriptor: String }
impl FieldRef {
    fn decode(bytes: &mut ByteReader, constant_pool: &ConstantPool) -> Result<Self, String> {
        let field_index:  w2 = bytes.take()?;
        let (class_index, name_and_type_index) = match constant_pool[field_index]
            .as_ref()
            .ok_or(format!("Invalid index: {}", field_index))?
        {
            Constant(CpTag::Fieldref, CpInfo::Fieldref { class_index, name_and_type_index }) => (*class_index, *name_and_type_index),
            Constant(tag, _) => {
                return Err(format!(
                    "Wrong constant type at index {idx}: expected `FieldRef`, found `{found}`",
                    idx = field_index,
                    found = tag
                ))
            }
        };
        let class = ClassRef(constant_pool.get_class_name(class_index)?);
        let (name, descriptor) = constant_pool.get_name_and_type(name_and_type_index)?;
        Ok(Self { class, name, descriptor })
    }

    fn encode(self, constant_pool: &mut ConstantPool, writer: &mut ByteWriter) {
        let class_index: w2 = {
            let mut class_writer = ByteWriter::new();
            self.class.encode(constant_pool, &mut class_writer);
            let buffer: Vec<w1> = class_writer.into();
            w2::read(buffer.as_slice())
        };
        let name_index =
            constant_pool.push(Constant(CpTag::Utf8, CpInfo::Utf8 { string: self.name }));
        let descriptor_index =
            constant_pool.push(Constant(CpTag::Utf8, CpInfo::Utf8 { string: self.descriptor }));
        let name_and_type_index =
            constant_pool.push(Constant(CpTag::NameAndType, CpInfo::NameAndType { name_index, descriptor_index }));
        let field_index = constant_pool.push(Constant(CpTag::Fieldref, CpInfo::Fieldref { class_index , name_and_type_index }));
        writer.write(field_index);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MethodRef { pub class: ClassRef, pub name: String, pub descriptor: String }
impl MethodRef {
    fn decode(bytes: &mut ByteReader, constant_pool: &ConstantPool) -> Result<Self, String> {
        let method_index:  w2 = bytes.take()?;
        let (class_index, name_and_type_index) = match constant_pool[method_index]
            .as_ref()
            .ok_or(format!("Invalid index: {}", method_index))?
        {
            Constant(CpTag::Methodref, CpInfo::Methodref { class_index, name_and_type_index }) => (*class_index, *name_and_type_index),
            Constant(tag, _) => {
                return Err(format!(
                    "Wrong constant type at index {idx}: expected `MethodRef`, found `{found}`",
                    idx = method_index,
                    found = tag
                ))
            }
        };
        let class = ClassRef(constant_pool.get_class_name(class_index)?);
        let (name, descriptor) = constant_pool.get_name_and_type(name_and_type_index)?;
        Ok(Self { class, name, descriptor })
    }

    fn encode(self, constant_pool: &mut ConstantPool, writer: &mut ByteWriter) {
        let class_index: w2 = {
            let mut class_writer = ByteWriter::new();
            self.class.encode(constant_pool, &mut class_writer);
            let buffer: Vec<w1> = class_writer.into();
            w2::read(buffer.as_slice())
        };
        let name_index =
            constant_pool.push(Constant(CpTag::Utf8, CpInfo::Utf8 { string: self.name }));
        let descriptor_index =
            constant_pool.push(Constant(CpTag::Utf8, CpInfo::Utf8 { string: self.descriptor }));
        let name_and_type_index =
            constant_pool.push(Constant(CpTag::NameAndType, CpInfo::NameAndType { name_index, descriptor_index }));
        let field_index = constant_pool.push(Constant(CpTag::Methodref, CpInfo::Methodref { class_index , name_and_type_index }));
        writer.write(field_index);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceMethodRef { class: ClassRef, name: String, descriptor: String }
impl InterfaceMethodRef {
    fn decode(bytes: &mut ByteReader, constant_pool: &ConstantPool) -> Result<Self, String> {
        let method_index:  w2 = bytes.take()?;
        let (class_index, name_and_type_index) = match constant_pool[method_index]
            .as_ref()
            .ok_or(format!("Invalid index: {}", method_index))?
        {
            Constant(CpTag::InterfaceMethodref, CpInfo::InterfaceMethodref { class_index, name_and_type_index }) => (*class_index, *name_and_type_index),
            Constant(tag, _) => {
                return Err(format!(
                    "Wrong constant type at index {idx}: expected `InterfaceMethodref`, found `{found}`",
                    idx = method_index,
                    found = tag
                ))
            }
        };
        let class = ClassRef(constant_pool.get_class_name(class_index)?);
        let (name, descriptor) = constant_pool.get_name_and_type(name_and_type_index)?;
        Ok(Self { class, name, descriptor })
    }

    fn encode(self, constant_pool: &mut ConstantPool, writer: &mut ByteWriter) {
        let class_index: w2 = {
            let mut class_writer = ByteWriter::new();
            self.class.encode(constant_pool, &mut class_writer);
            let buffer: Vec<w1> = class_writer.into();
            w2::read(buffer.as_slice())
        };
        let name_index =
            constant_pool.push(Constant(CpTag::Utf8, CpInfo::Utf8 { string: self.name }));
        let descriptor_index =
            constant_pool.push(Constant(CpTag::Utf8, CpInfo::Utf8 { string: self.descriptor }));
        let name_and_type_index =
            constant_pool.push(Constant(CpTag::NameAndType, CpInfo::NameAndType { name_index, descriptor_index }));
        let field_index = constant_pool.push(Constant(CpTag::InterfaceMethodref, CpInfo::InterfaceMethodref { class_index , name_and_type_index }));
        writer.write(field_index);
    }
}

macro_rules! opcodes {
    ($($opname:ident = $opcode:literal $({ $($fieldname:ident: $fieldtype:ty),+ })?;)*) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Serialize, Deserialize, Copy, Clone)]
        #[repr(u8)]
        pub enum Opcodes {
            $($opname = $opcode,)*
        }

        impl From<Opcodes> for w1 {
            fn from(value: Opcodes) -> Self {
                value as Self
            }
        }
        impl TryFrom<w1> for Opcodes {
            type Error = String;

            fn try_from(value: w1) -> Result<Self, Self::Error> {
                match value {
                    $($opcode => Result::Ok(Self::$opname),)*
                    _ => Result::Err(format!("Invalid opcode {}", value)),
                }
            }
        }

        #[allow(non_camel_case_types)]
        #[derive(Debug, Serialize, Deserialize)]
        pub enum OpcodeInfo {
            $($opname $({ $($fieldname: $fieldtype),+ })? ,)*
        }

        impl OpcodeInfo {
            pub fn decode_opcode_info(
                bytes: &mut ByteReader,
                constants: &ConstantPool
            ) -> Result<Self, String> {
                trait Decode<T> {
                    fn decode(
                        bytes: &mut ByteReader,
                        _constants: &ConstantPool
                    ) -> Result<T, String>;
                }
                impl<T> Decode<T> for T where ByteReader: Take<T> {
                    fn decode(
                        bytes: &mut ByteReader,
                        _constants: &ConstantPool
                    ) -> Result<T, String> {
                        bytes.take()
                    }
                }
                let opcode: w1 = bytes.take()?;
                Ok(match opcode.try_into()? {
                    $(Opcodes::$opname => OpcodeInfo::$opname$({ $($fieldname: <$fieldtype>::decode(bytes, constants)?),+ })? ,)*
                })
            }
            pub fn encode_opcode_info(
                self,
                constant_pool: &mut ConstantPool,
                writer: &mut ByteWriter
            ) {
                trait Encode {
                    fn encode(self,
                        _constant_pool: &mut ConstantPool,
                        writer: &mut ByteWriter);
                }
                impl<T> Encode for T where T: Writeable {
                    fn encode(self,
                        _constant_pool: &mut ConstantPool,
                        writer: &mut ByteWriter) {
                        writer.write(self);
                    }
                }
                match self {
                    $(OpcodeInfo::$opname $({ $($fieldname),+ })? => {
                        let opcode: w1 = Opcodes::$opname.into();
                        writer.write(opcode);
                        $( $(<$fieldtype>::encode($fieldname, constant_pool, writer);)+ )?
                    },)*
                }
            }
        }
    };
}

// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-6.html#jvms-6.5
opcodes! {
    aaload = 0x32;
    aastore = 0x53;
    aconst_null = 0x1;
    aload = 0x19 { index: w1 };
    aload_0 = 0x2a;
    aload_1 = 0x2b;
    aload_2 = 0x2c;
    aload_3 = 0x2d;
    anewarray = 0xbd {class: ClassRef}; // Constant pool index of class
    areturn = 0xb0;
    arraylength = 0xbe;
    astore = 0x3a {index: w1};
    astore_0 = 0x4b;
    astore_1 = 0x4c;
    astore_2 = 0x4d;
    astore_3 = 0x4e;
    athrow = 0xbf;
    baload = 0x33;
    bastore = 0x54;
    bipush = 0x10;
    caload = 0x34;
    castore = 0x55;
    checkcast = 0xc0 {class: ClassRef}; // Constant pool index of class
    d2f = 0x90;
    d2i = 0x8e;
    d2l = 0x8f;
    dadd = 0x63;
    daload = 0x31;
    dastore = 0x52;
    dcmpg = 0x98;
    dcmpl = 0x97;
    dconst_0 = 0xe;
    dconst_1 = 0xf;
    ddiv = 0x6f;
    dload = 0x18 {index: w1};
    dload_0 = 0x26;
    dload_1 = 0x27;
    dload_2 = 0x28;
    dload_3 = 0x29;
    dmul = 0x6b;
    dneg = 0x77;
    drem = 0x73;
    dreturn = 0xaf;
    dstore = 0x39 {index: w1};
    dstore_0 = 0x47;
    dstore_1 = 0x48;
    dstore_2 = 0x49;
    dstore_3 = 0x4a;
    dsub = 0x67;
    dup = 0x59;
    dup_x1 = 0x5a;
    dup_x2 = 0x5b;
    dup2 = 0x5c;
    dup2_x1 = 0x5d;
    dup2_x2 = 0x5e;
    f2d = 0x8d;
    f2i = 0x8b;
    f2l = 0x8c;
    fadd = 0x62;
    faload = 0x30;
    fastore = 0x51;
    fcmpg = 0x96;
    fcmpl = 0x95;
    fconst_0 = 0xb;
    fconst_1 = 0xc;
    fconst_2 = 0xd;
    fdiv = 0x6e;
    fload = 0x17 {index: w1};
    fload_0 = 0x22;
    fload_1 = 0x23;
    fload_2 = 0x24;
    fload_3 = 0x25;
    fmul = 0x6a;
    fneg = 0x76;
    frem = 0x72;
    freturn = 0xae;
    fstore = 0x38 {index: w1};
    fstore_0 = 0x43;
    fstore_1 = 0x44;
    fstore_2 = 0x45;
    fstore_3 = 0x46;
    fsub = 0x66;
    getfield = 0xb4 {field: FieldRef}; // Constant pool index of fieldref
    getstatic = 0xb2 {field: FieldRef}; // Constant pool index of fieldref
    goto = 0xa7 {branch: w2};
    goto_w = 0xc8 {branch: w4};
    i2b = 0x91;
    i2c = 0x92;
    i2d = 0x87;
    i2f = 0x86;
    i2l = 0x85;
    i2s = 0x93;
    iadd = 0x60;
    iaload = 0x2e;
    iand = 0x7e;
    iastore = 0x4f;
    iconst_m1 = 0x2;
    iconst_0 = 0x3;
    iconst_1 = 0x4;
    iconst_2 = 0x5;
    iconst_3 = 0x6;
    iconst_4 = 0x7;
    iconst_5 = 0x8;
    idiv = 0x6c;

    if_acmpeq = 0xa5;
    if_acmpne = 0xa6;
    if_icmpeq = 0x9f;
    if_icmpne = 0xa0;
    if_icmplt = 0xa1;
    if_icmpge = 0xa2;
    if_icmpgt = 0xa3;
    if_icmple = 0xa4;
    ifeq = 0x99 {branch: w2};
    ifne = 0x9a {branch: w2};
    iflt = 0x9b {branch: w2};
    ifge = 0x9c {branch: w2};
    ifgt = 0x9d {branch: w2};
    ifle = 0x9e {branch: w2};
    ifnonnull = 0xc7 {branch: w2};
    ifnull = 0xc6 {branch: w2};
    iinc = 0x84 {index: w1, constant: w1};
    iload = 0x15 {index: w1};
    iload_0 = 0x1a;
    iload_1 = 0x1b;
    iload_2 = 0x1c;
    iload_3 = 0x1d;
    imul = 0x68;
    ineg = 0x74;
    instanceof = 0xc1 {class: ClassRef}; // Constant pool index of class
    invokedynamic = 0xba {index: w2, _zero: w2}; // Constant pool index of ?; zero
    invokeinterface = 0xb9 {method: InterfaceMethodRef, count: w1, _zero: w1}; // Constant pool index of interface method ref; nargs; zero
    invokespecial = 0xb7 {method: MethodRef}; // Constant pool index of method ref
    invokestatic = 0xb8 {method: MethodRef}; // Constant pool index of method ref
    invokevirtual = 0xb6 {method: MethodRef}; // Constant pool index of method ref
    ior = 0x80;
    irem = 0x70;
    ireturn = 0xac;
    ishl = 0x78;
    ishr = 0x7a;
    istore = 0x36 {index: w1};
    istore_0 = 0x3b;
    istore_1 = 0x3c;
    istore_2 = 0x3d;
    istore_3 = 0x3e;
    isub = 0x64;
    iushr = 0x7c;
    ixor = 0x82;
    jsr = 0xa8 {branch: w2};
    jsr_w = 0xc9 {branch: w4};
    l2d = 0x8a;
    l2f = 0x89;
    l2i = 0x88;
    ladd = 0x61;
    laload = 0x2f;
    land = 0x7f;
    lastore = 0x50;
    lcmp = 0x94;
    lconst_0 = 0x9;
    lconst_1 = 0xa;

    ldc = 0x12 {index: w1}; // Constant pool index of constant
    ldc_w = 0x13 {index: w2}; // Constant pool index of constant
    ldc2_w = 0x14 {index: w2}; // Constant pool index of constant

    ldiv = 0x6d;
    lload = 0x16 {index: w1};
    lload_0 = 0x1e;
    lload_1 = 0x1f;
    lload_2 = 0x20;
    lload_3 = 0x21;
    lmul = 0x69;
    lneg = 0x75;

    // TODO lookupswitch = 0xab
    lor = 0x81;
    lrem = 0x71;
    lreturn = 0xad;
    lshl = 0x79;
    lshr = 0x7b;
    lstore = 0x37 {index: w1};
    lstore_0 = 0x3f;
    lstore_1 = 0x40;
    lstore_2 = 0x41;
    lstore_3 = 0x42;
    lsub = 0x65;
    lushr = 0x7d;
    lxor = 0x83;

    monitorenter = 0xc2;
    monitorexit = 0xc3;

    multianewarray = 0xc5 {class: ClassRef, dimensions: w1};
    new = 0xbb {class: ClassRef}; // Constant pool index of class
    // (https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-6.html#jvms-6.5.newarray)
    newarray = 0xbc {atype: w1}; // TODO: create enum
    nop = 0x00;
    pop = 0x57;
    pop2 = 0x58;
    putfield = 0xb5 {field: FieldRef}; // Constant pool index of field ref
    putstatic = 0xb3 {field: FieldRef}; // Constant pool index of field ref

    ret = 0xa9;
    r#return = 0xb1;

    saload = 0x35;
    sastore = 0x56;
    sipush = 0x11 {short: w2};
    swap = 0x5f;

    // TODO tableswitch = 0xaa;
    // TODO wide = 0xc4;
}
