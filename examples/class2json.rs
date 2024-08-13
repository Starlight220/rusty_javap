use rusty_javap::bytecode::reader::{ByteReader, Take};
use rusty_javap::model::class::Class;
use std::fs::read;
use std::fs::write;
use std::path::Path;

fn main() {
    let path = Path::new("./data/Example.class");
    let mut bytes: ByteReader = read(path).expect("Failed to read file:\n").into();

    let class: Class = bytes.take().expect("Failed to parse class\n");

    let outfile = Path::new("./data/Example.class.json");
    write(outfile, serde_json::to_string(&class).unwrap()).unwrap();
}
