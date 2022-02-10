use crate::constant_pool::ConstantPool;
use crate::{container, w1, w2, w4, ByteReader, Take, Unresolved};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Attribute {
    ConstantValue(String),
    // Code(UnresolvedCode),
    // Exceptions,
    SourceFile(String),
    // LineNumberTable,
    // LocalVariableTable,
    // InnerClasses,
    Synthetic,
    Deprecated,
    // EnclosingMethod,
    Signature {
        signature_index: w2,
    },
    // SourceDebugExtension,
    // LocalVariableTypeTable,
    // RuntimeVisibleAnnotations { num_annotations: w2}, // TODO: needs annotations
    // RuntimeInvisibleAnnotations { num_annotations: w2}, // TODO: needs annotations
    // StackMapTable { entries: Vec<StackMapFrame>}, } // TODO
    // BootstrapMethods,
    // AnnotationDefault,
    // RuntimeVisibleTypeAnnotations { num_annotations: w2}, // TODO: needs annotations
    // RuntimeInvisibleTypeAnnotations { num_annotations: w2}, // TODO: needs annotations
    // MethodParameters,
    // Module,
    // ModulePackages,
    // ModuleMainClass,
    // NestHost,
    // NestMembers,
    #[allow(non_camel_case_types)]
    UNIMPLEMENTED_ATTRIBUTE_TODO, // FIXME
}

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
                ConstantValue(constant_pool.get_constant_as_string(bytes.take()?)?)
            }
            stringify!(Synthetic) => Synthetic,
            stringify!(Deprecated) => Deprecated,
            stringify!(Signature) => Signature {
                signature_index: bytes.take()?,
            },
            stringify!(SourceFile) => SourceFile(constant_pool.get_utf8(bytes.take()?)?),

            &_ => UNIMPLEMENTED_ATTRIBUTE_TODO,
        })
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

impl Unresolved for Vec<UnresolvedAttribute> {
    type Resolved = Attributes;
    type NeededToResolve = ConstantPool;

    fn resolve(self, constant_pool: &Self::NeededToResolve) -> Result<Self::Resolved, String> {
        let mut resolved = vec![];
        for attr in self {
            resolved.push(Attribute::create(
                constant_pool.get_utf8(attr.name_index)?,
                attr.info,
                constant_pool,
            )?)
        }
        Ok(resolved.into())
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Attribute::*;
        match self {
            ConstantValue(contents_str) => {
                write!(f, "{}({})", stringify!(ConstantValue), contents_str)
            }
            SourceFile(file) => write!(f, "{}({})", stringify!(SourceFile), file),
            Synthetic => write!(f, "{}", stringify!(Synthetic)),
            Deprecated => write!(f, "{}", stringify!(Deprecated)),
            Signature { signature_index } => {
                write!(f, "{}(#{})", stringify!(Signature), signature_index)
            }
            UNIMPLEMENTED_ATTRIBUTE_TODO => write!(f, "UNIMPLEMENTED_TODO"),
        }
    }
}

container!(Attributes, Attribute);
