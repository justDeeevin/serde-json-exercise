pub fn escape(c: char) -> String {
    match c {
        '"' => "\\\"".to_string(),
        '\\' => "\\\\".to_string(),
        '\x08' => "\\b".to_string(),
        '\x0c' => "\\f".to_string(),
        '\n' => "\\n".to_string(),
        '\r' => "\\r".to_string(),
        '\t' => "\\t".to_string(),
        '\x00'..='\x1F' => format!("\\u{:04x}", c as u8),
        _ => c.to_string(),
    }
}
