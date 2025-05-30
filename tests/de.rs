use serde::Deserialize;
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

#[test]
fn float() {
    let input = "[-11.22, 1]";
    let json = json::from_str::<Vec<f32>>(input).expect("Failed to deserialize");
    assert_eq!(json, [-11.22, 1.0]);
}

#[test]
fn option() {
    let input = "null";
    let json = json::from_str::<Option<u8>>(input).expect("Failed to deserialize");
    assert_eq!(json, None);

    let input = "1";
    let json = json::from_str::<Option<u8>>(input).expect("Failed to deserialize");
    assert_eq!(json, Some(1));
}

#[test]
fn d_enum() {
    #[derive(Debug, PartialEq, Deserialize)]
    enum Test {
        A(u8),
        B { a: u8, b: u8 },
    }
    let input = r#"{"A":1}"#;
    let json = json::from_str::<Test>(input).expect("Failed to deserialize");
    assert_eq!(json, Test::A(1));
    let input = r#"{"B":{"a":1,"b":2}}"#;
    let json = json::from_str::<Test>(input).expect("Failed to deserialize");
    assert_eq!(json, Test::B { a: 1, b: 2 });
}
