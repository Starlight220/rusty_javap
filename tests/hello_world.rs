use rusty_javap::bytecode::writer::{ByteWriter, Writeable};
use rusty_javap::model::attrs::code::OpcodeInfo::{getstatic, invokevirtual, ldc, r#return, swap};
use rusty_javap::model::attrs::code::{ClassRef, Code, FieldRef, MethodRef};
use rusty_javap::model::attrs::Attribute;
use rusty_javap::model::class::{Class, ClassAccessModifier, Version};
use rusty_javap::model::method::{Method, MethodAccessModifier};
use std::fs::File;
use std::io::Write;
use std::process::Command;

#[test]
fn hello_world() {
    let code = Code {
        max_stack: 3,
        max_locals: 1,
        code: vec![
            // TODO: fix ldc to work properly with constants rather than guessing indexes
            ldc { index: 2 }, // Assume that the current class is at index 2
            invokevirtual { method: MethodRef { class: ClassRef("java/lang/Class".to_string()), name: "getName".to_string(), descriptor: "()Ljava/lang/String;".to_string() }},
            getstatic { field: FieldRef { class: ClassRef("java/lang/System".to_string()), name: "out".to_string(), descriptor: "Ljava/io/PrintStream;".to_string()} },
            swap,
            invokevirtual { method: MethodRef { class: ClassRef("java/io/PrintStream".to_string()), name: "println".to_string(), descriptor: "(Ljava/lang/String;)V".to_string()}},
            r#return,
        ],
        exception_table: vec![],
        attributes: vec![],
    };
    let main = Method {
        access_flags: vec![MethodAccessModifier::PUBLIC,  MethodAccessModifier::STATIC],
        name: "main".to_string(),
        descriptor: "([Ljava/lang/String;)V".to_string(),
        attributes: vec![Attribute::Code(code)],
    };
    let class = Class {
        version: Version::new(0xCAFEBABE, 53, 0),
        access_flags: vec![ClassAccessModifier::PUBLIC],
        this_class: "HelloWorld".to_string(),
        super_class: Some("java/lang/Object".to_string()),
        interfaces: vec![],
        fields: vec![],
        methods: vec![main],
        attributes: vec![],
    };

    let mut writer = ByteWriter::new();
    class.write(&mut writer);

    let class_bytes: Vec<u8> = writer.into();
    let path = "HelloWorld.class";
    let mut file = File::create(path).expect("Couldn't create class file");
    file.write_all(&class_bytes).expect("Couldn't write class file");

    let output = Command::new("java").args(["HelloWorld"]).output().unwrap();
    let stdout = String::from_utf8_lossy(output.stdout.as_slice());
    println!("Got output: {}", stdout);
    if output.stderr.len() > 0 {
        eprintln!("Got stderr: {}", String::from_utf8_lossy(output.stderr.as_slice()));
    }
    assert!(output.status.success());
    assert_eq!(stdout, "HelloWorld\n");
}
