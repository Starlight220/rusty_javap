mod access;
mod constant_pool;
mod reader;
mod typedefs;

use crate::constant_pool::read_constants;
use crate::reader::*;
use crate::typedefs::*;
use std::fs::read;
use std::path::Path;
use crate::access::read_access;

/// https://medium.com/swlh/an-introduction-to-jvm-bytecode-5ef3165fae70
/// https://en.wikipedia.org/wiki/List_of_Java_bytecode_instructions
/// https://blogs.oracle.com/javamagazine/post/understanding-java-method-invocation-with-invokedynamic
/// https://docs.oracle.com/javase/specs/jvms/se12/html/jvms-4.html

fn main() {
    read_class_file(Path::new("./data/Example.class"));
}

fn read_class_file(path: &Path) {
    let mut bytes: ByteReader = ByteReader::from(read(path).unwrap());

    read_version(&mut bytes);
    read_constants(&mut bytes);
    read_access(&mut bytes);
}

fn read_version(bytes: &mut ByteReader) {
    println!(
        "\
        magic = {magic:#X};\n\
        minor = {minor};\n\
        major = {major};\n\
        ",
        magic = bytes.take::<w4>(),
        minor = bytes.take::<w2>(),
        major = bytes.take::<w2>()
    );
}
