use rusty_javap::model::class::Class;

#[test]
fn json2class2json() {
    let class_json_content = include_bytes!("./Example.class.json");
    let json_class: serde_json::Value = serde_json::from_slice(class_json_content).unwrap();

    let class: Class = serde_json::from_value(json_class.clone()).unwrap();

    let written_json: serde_json::Value = serde_json::to_value(class).unwrap();
    assert_eq!(json_class, written_json)
}

