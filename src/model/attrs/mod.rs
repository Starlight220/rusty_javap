pub mod constant_value;
pub mod method_parameters;

use crate::model::attrs::constant_value::ConstantValue;
use crate::model::attrs::method_parameters::MethodParameters;
use crate::w2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
    MethodParameters(MethodParameters),
    // Module,
    // ModulePackages,
    // ModuleMainClass,
    // NestHost,
    // NestMembers,
    #[allow(non_camel_case_types)]
    UNIMPLEMENTED_ATTRIBUTE_TODO {
        name: String,
        info: Vec<u8>,
    }, // FIXME
}
