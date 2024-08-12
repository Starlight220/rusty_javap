use rusty_javap::bytecode::writer::ByteWriter;
use rusty_javap::model::class::Class;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./data/Example.class.json");
    let class_json = fs::read(path).unwrap();
    let class: Class = serde_json::from_slice(&class_json).unwrap();

    let outfile = Path::new("./data/Example.out.class");
    let mut writer: ByteWriter = ByteWriter::new();
    writer.write::<Class>(class);
    let class_bytes: Vec<u8> = writer.into();
    fs::write(outfile, class_bytes).unwrap();
}
