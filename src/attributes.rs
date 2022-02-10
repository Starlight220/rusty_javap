use crate::constant_pool::ConstantPool;
use crate::{w1, w2, w4, ByteReader, Take, Unresolved};

#[derive(Debug)]
pub enum Attribute {
    ConstantValue { constantvalue: w2 },
    // Code(UnresolvedCode),
    // Exceptions,
    // SourceFile,
    // LineNumberTable,
    // LocalVariableTable,
    // InnerClasses,
    Synthetic,
    Deprecated,
    // EnclosingMethod,
    Signature { signature_index: w2 },
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
    UNIMPLEMENTED_ATTRIBUTE_TODO // FIXME
}

impl Attribute {
    fn create(name: String, info: Vec<w1>) -> Result<Attribute, String> {
        let mut bytes = ByteReader::from(info);
        use Attribute::*;
        Ok(match name.as_str() {
            stringify!(ConstantValue) => ConstantValue {
                constantvalue: bytes.take()?,
            },
            stringify!(Synthetic) => Synthetic,
            stringify!(Deprecated) => Deprecated,
            stringify!(Signature) => Signature {
                signature_index: bytes.take()?,
            },

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
    type Resolved = Vec<Attribute>;
    type NeededToResolve = ConstantPool;

    fn resolve(self, constant_pool: &Self::NeededToResolve) -> Result<Self::Resolved, String> {
        let mut resolved = vec![];
        for attr in self {
            resolved.push(Attribute::create(
                constant_pool.get_utf8(attr.name_index)?,
                attr.info,
            )?)
        }
        Ok(resolved)
    }
}

#[derive(Debug)]
pub struct UnresolvedCode {
    max_stack: w2,
    max_locals: w2,
    code_length: w4,
    code: Vec<w1>,
    exception_table_length: w2,
    exception_table: Vec<Exception>,
    attributes: Vec<UnresolvedAttribute>,
}

#[derive(Debug)]
pub struct Exception {
    start_pc: w2,
    end_pc: w2,
    handler_pc: w2,
    catch_type: w2,
}
