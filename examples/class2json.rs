use std::fs::write;
use std::fs::read;
use std::path::Path;
use rusty_javap::bytecode::reader::{ByteReader, Take};
use rusty_javap::model::class::Class;

fn main() {
    let path = Path::new("./data/Example.class");
    let mut bytes: ByteReader = read(path).unwrap().into();

    let class: Class = bytes.take().unwrap();

    let outfile = Path::new("./data/Example.class.json");
    write(outfile, serde_json::to_string(&class).unwrap()).unwrap();
}
