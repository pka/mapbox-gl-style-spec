extern crate lalrpop_util;

pub mod json;

#[test]
fn json() {
    use lalrpop_util::ParseError;

    assert!(json::parse_json(r#"{"circle-radius": {"stops": [1, 0]}}"#).is_ok());
    assert!(json::parse_json(r#"{"circle-radius": {"property": "temperature","stops": [[{"zoom": 0, "value": 0}, 0],[{"zoom": 20, "value": 5}, 20]]}}"#).is_ok());
    assert!(json::parse_json(r#"{"string": "\u12345\\\"\r"}"#).is_ok());
    assert_eq!(json::parse_json(r#"{zoom: 0}"#), Err(ParseError::InvalidToken { location: 1 }));
}

fn main() {
    println!("Hello, world!");
}
