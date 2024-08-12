use rusty_javap::bytecode::writer::ByteWriter;
use rusty_javap::model::class::Class;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./data/Example.class.json");
    let class_json = fs::read(path).unwrap();
    let class: Class = serde_json::from_slice(&class_json).unwrap();

    let outfile = Path::new("./data/Example.out.class");
    let mut class_bytes: ByteWriter = ByteWriter::new();
    class_bytes.write::<Class>(&class);
    fs::write(outfile, serde_json::to_string(&class).unwrap()).unwrap();
}
