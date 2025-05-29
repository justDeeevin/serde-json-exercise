use serde_json_exercise as json;

#[test]
fn string() {
    assert_eq!(
        json::from_str::<String>("\"droddyrox\"").expect("Failed to deserialize"),
        "droddyrox"
    );
}

#[test]
fn whitespace() {
    let input = r#""droddyrox" "#;
    assert_eq!(
        json::from_str::<String>(input).expect("Failed to deserialize"),
        "droddyrox"
    );
}

#[test]
fn escape_string() {
    let input = r#""droddy\"rox\"""#;
    println!("{input}");
    let json = json::from_str::<String>(input).expect("Failed to deserialize");
    assert_eq!(json, "droddy\"rox\"");
}

#[test]
fn escape_ascii_control() {
    let input = r#""\u000f""#;
    let json = json::from_str::<String>(input).expect("Failed to deserialize");
    assert_eq!(json, "\x0F");
}

#[test]
fn unclosed_string() {
    let input = r#""droddyrox"#;
    assert!(json::from_str::<String>(input).is_err());
}

#[test]
fn seq() {
    let input = r#"[1,2,3]"#;
    let json = json::from_str::<Vec<u8>>(input).expect("Failed to deserialize");
    assert_eq!(json, vec![1, 2, 3]);
}

#[test]
fn map() {
    let input = r#"{"a":"droddy","b":"rox"}"#;
    let json = json::from_str::<std::collections::HashMap<String, String>>(input)
        .expect("Failed to deserialize");
    assert_eq!(json.get("a"), Some(&"droddy".to_string()));
    assert_eq!(json.get("b"), Some(&"rox".to_string()));
}
