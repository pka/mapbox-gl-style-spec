extern crate lalrpop_util;
extern crate toml;

pub mod json;

#[test]
fn json() {
    use lalrpop_util::ParseError;

    assert!(json::parse_json(r#"{"circle-radius": {"stops": [1, 0]}}"#).is_ok());
    assert!(json::parse_json(r#"{"circle-radius": {"property": "temperature","stops": [[{"zoom": 0, "value": 0}, 0],[{"zoom": 20, "value": 5}, 20]]}}"#).is_ok());
    assert!(json::parse_json(r#"{"string": "\u12345\\\"\r"}"#).is_ok());
    assert_eq!(json::parse_json(r#"{zoom: 0}"#), Err(ParseError::InvalidToken { location: 1 }));
}

#[test]
fn value_to_toml() {
    assert_eq!(json::parse_value("-1"), Ok(toml::Value::Integer(-1)));
    assert_eq!(json::parse_value("3.5"), Ok(toml::Value::Float(3.5)));
    assert_eq!(json::parse_value("2.5e10"), Ok(toml::Value::Float(2.5e10)));
    assert_eq!(json::parse_value("2e-10"), Ok(toml::Value::Float(2e-10)));
    assert_eq!(json::parse_value("true"), Ok(toml::Value::Boolean(true)));
    assert_eq!(json::parse_value("false"), Ok(toml::Value::Boolean(false)));
}

#[test]
fn json_to_toml() {
    let json = r#"{"circle-radius": {"stops": [-1, 3.5, 2e10]}}"#;
    assert_eq!(format!("{:?}", json::parse_json(json)),
        r#"Ok(Table({"circle-radius": Table({"stops": Array([Integer(-1), Float(3.5), Float(20000000000)])})}))"#
    );

    let toml = r#"[circle-radius]
stops = [-1, 3.5, 20000000000.0]
"#;
    assert_eq!(format!("{}", json::parse_json(json).unwrap()),
        toml
    );

    let json = r#"{"circle-radius": {"property": "temperature","stops": [[{"zoom": 0, "value": 0}, 0],[{"zoom": 20, "value": 5}, 20]]}}"#;
    let toml = r#"[circle-radius]
property = "temperature"
stops = [[value = 0
zoom = 0
, 0], [value = 5
zoom = 20
, 20]]
"#;
    println!("{}", json::parse_json(json).unwrap());
    assert_eq!(format!("{}", json::parse_json(json).unwrap()),
        toml
    );
}

fn main() {
    println!("Hello, world!");
}
