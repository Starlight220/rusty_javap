mod access;
mod attributes;
mod classfile;
mod constant_pool;
mod fields;
mod interfaces;
mod methods;
mod reader;
mod typedefs;
mod versions;

use crate::classfile::Class;
use crate::reader::*;
use crate::typedefs::*;
use std::fs::read;
use std::path::Path;

/// https://medium.com/swlh/an-introduction-to-jvm-bytecode-5ef3165fae70
/// https://en.wikipedia.org/wiki/List_of_Java_bytecode_instructions
/// https://blogs.oracle.com/javamagazine/post/understanding-java-method-invocation-with-invokedynamic
/// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html

fn main() {
    let path = Path::new("./data/Example.class");
    let mut bytes: ByteReader = read(path).unwrap().into();

    match bytes.take() {
        Err(error) => println!("{}", error),
        Ok::<Class, _>(class) => println!("{}", class),
    }
}
