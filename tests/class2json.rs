use rusty_javap::bytecode::reader::{ByteReader, Take};
use rusty_javap::model::class::Class;


#[test]
fn classfile2json() {
    let bytes = include_bytes!("./Example.class");
    let mut reader: ByteReader = bytes.to_vec().into();

    let class: Class = reader.take().expect("Failed to parse class\n");

    let class_json_content = include_bytes!("./Example.class.json");
    let json_class_original: serde_json::Value = serde_json::from_slice(class_json_content).unwrap();
    let json_class_written = serde_json::to_value(class).unwrap();
    assert_eq!(json_class_original, json_class_written)
}
