use serde::Serialize;
use serde_json_exercise as json;

#[test]
fn string() {
    assert_eq!(
        json::to_string(&"droddyrox").expect("Failed to serialize"),
        "\"droddyrox\""
    );
}

#[test]
fn escape_string() {
    let string = "droddy\"rox\"";
    let json = json::to_string(&string).expect("Failed to serialize");
    assert_eq!(json, "\"droddy\\\"rox\\\"\"");
}

#[test]
fn escape_ascii_control() {
    let string = "\x0F";
    let json = json::to_string(&string).expect("Failed to serialize");
    assert_eq!(json, "\"\\u000f\"");
}

#[test]
fn seq() {
    assert_eq!(
        json::to_string(&[1, 2, 3]).expect("Failed to serialize"),
        "[1,2,3]"
    );
}

#[test]
fn map() {
    let map = std::collections::HashMap::from([("a", 1), ("b", 2)]);
    let string = json::to_string(&map).expect("Failed to serialize");
    // HashMap ordering is not guarenteed
    assert!(string == "{\"a\":1,\"b\":2}" || string == "{\"b\":2,\"a\":1}");
}

#[test]
fn bad_key() {
    let map = std::collections::HashMap::from([(1, 1), (2, 2)]);
    assert!(json::to_string(&map).is_err());
}

#[test]
fn tuple() {
    assert_eq!(
        json::to_string(&(1, 2, 3)).expect("Failed to serialize"),
        "[1,2,3]"
    );
}

#[test]
fn tuple_struct() {
    #[derive(Serialize)]
    struct Point(i32, i32);
    assert_eq!(
        json::to_string(&Point(1, 2)).expect("Failed to serialize"),
        "[1,2]"
    );
}

#[test]
fn newtype_variant() {
    #[derive(Serialize)]
    enum Name {
        First(String),
    }
    assert_eq!(
        json::to_string(&Name::First("droddyrox".to_string())).expect("Failed to serialize"),
        "{\"First\":\"droddyrox\"}"
    );
}

#[test]
fn tuple_variant() {
    #[derive(Serialize)]
    enum Color {
        Rgb(u8, u8, u8),
    }
    assert_eq!(
        json::to_string(&Color::Rgb(0, 0, 0)).expect("Failed to serialize"),
        "{\"Rgb\":[0,0,0]}"
    );
}

#[test]
fn struct_variant() {
    #[derive(Serialize)]
    enum Color {
        Rgb { r: u8, g: u8, b: u8 },
    }
    assert_eq!(
        json::to_string(&Color::Rgb { r: 0, g: 0, b: 0 }).expect("Failed to serialize"),
        "{\"Rgb\":{\"r\":0,\"g\":0,\"b\":0}}"
    );
}

#[test]
fn newtype_struct() {
    #[derive(Serialize)]
    struct Age(u8);
    assert_eq!(json::to_string(&Age(0)).expect("Failed to serialize"), "0");
}
