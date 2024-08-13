pub mod bytecode;
pub mod constant_pool;
pub mod model;
pub mod typedefs;

use crate::model::class::Class;
use crate::typedefs::*;

// https://medium.com/swlh/an-introduction-to-jvm-bytecode-5ef3165fae70
// https://en.wikipedia.org/wiki/List_of_Java_bytecode_instructions
// https://blogs.oracle.com/javamagazine/post/understanding-java-method-invocation-with-invokedynamic
// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html
