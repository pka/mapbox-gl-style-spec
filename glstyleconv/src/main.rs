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
    assert_eq!(format!("{}", json::parse_json(json).unwrap()), toml);

    let json = r#"[1,2,3]"#;
    let toml = "json = [1, 2, 3]\n";
    println!("{}", json::parse_json(json).unwrap());
    assert_eq!(format!("{}", json::parse_json(json).unwrap()), toml);
    assert!(toml::Parser::new(toml).parse().is_some());

    let json = r##"
        {
          "id": "water",
          "source": "mapbox-streets",
          "source-layer": "water",
          "type": "fill",
          "paint": {
            "fill-color": "#00ffff"
          }
        }"##;
    let toml = r##"id = "water"
source = "mapbox-streets"
source-layer = "water"
type = "fill"

[paint]
fill-color = "#00ffff"
"##;
    println!("{}", json::parse_json(json).unwrap());
    assert_eq!(format!("{}", json::parse_json(json).unwrap()), toml);
    assert!(toml::Parser::new(toml).parse().is_some());

    //top-level array shall be embedded in a table
    let json = r##"[
        {
          "id": "water",
          "source": "mapbox-streets",
          "source-layer": "water",
          "type": "fill",
          "paint": {
            "fill-color": "#00ffff"
          }
        }
    ]"##;
    let toml = r##"[[json]]
id = "water"
source = "mapbox-streets"
source-layer = "water"
type = "fill"

[json.paint]
fill-color = "#00ffff"
"##;
    println!("{}", json::parse_json(json).unwrap());
    assert_eq!(format!("{}", json::parse_json(json).unwrap()), toml);
    assert!(toml::Parser::new(toml).parse().is_some());

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
    assert_eq!(format!("{}", json::parse_json(json).unwrap()), toml);
    // Resulting TOML is invalid:
    assert_eq!(toml::Parser::new(toml).parse(), None);
}

#[test]
fn json_file_to_toml() {
    use std::io::prelude::*;
    use std::fs::File;

    let mut file = File::open("testdata/bright-v9-cdn.json").unwrap();
    let mut json = String::new();
    let _ = file.read_to_string(&mut json);
    let toml = format!("{}", json::parse_json(&json).unwrap());
    println!("{}", toml);
    assert!(toml.contains(r##"[[layers]]
id = "background"
interactive = true
type = "background"

[layers.paint]
background-color = "#f8f4f0"

[[layers]]
filter = ["==", "class", "national_park"]
id = "landuse_overlay_national_park"
interactive = true
source = "mapbox"
source-layer = "landuse_overlay"
type = "fill"

[layers.metadata]
"mapbox:group" = "1444849388993.3071"

[layers.paint]
fill-color = "#d8e8c8"
fill-opacity = 0.75

[[layers]]
filter = ["==", "class", "park"]
id = "landuse_park"
interactive = true
source = "mapbox"
source-layer = "landuse"
type = "fill"
"##));

    assert!(toml.contains(r##"[[layers]]
filter = ["==", "scalerank", 1]
id = "country_label_1"
interactive = true
source = "mapbox"
source-layer = "country_label"
type = "symbol"

[layers.layout]
text-field = "{name_en}"
text-font = ["Open Sans Bold"]
text-max-width = 6.25
text-transform = "uppercase"

[layers.layout.text-size]
stops = [[1, 11], [4, 17]]

[layers.metadata]
"mapbox:group" = "1444849242106.713"

[layers.paint]
text-color = "#334"
text-halo-blur = 1
text-halo-color = "rgba(255,255,255,0.8)"
text-halo-width = 2
"##));
}


fn main() {
    println!("Hello, world!");
}
