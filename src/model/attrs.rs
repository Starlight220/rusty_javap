use serde::{Serialize, Deserialize};
use crate::model::attrs::constant_value::ConstantValue;

mod constant_value;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum Attribute {
    ConstantValue(ConstantValue),
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
