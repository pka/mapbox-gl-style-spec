extern crate lalrpop_util;

pub mod json;
pub mod ast;

use std::io;
use std::fs::File;
use std::env;


fn read_to_toml(input: &mut io::Read) -> String {
    let mut json = String::new();
    let _ = input.read_to_string(&mut json);
    format!("{}", json::parse_json(&json).unwrap())
}

fn json_file_to_toml(fname: &str) -> String {
    let mut file = File::open(fname).unwrap();
    read_to_toml(&mut file)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => println!("{}", read_to_toml(&mut io::stdin())),
        2 => println!("{}", json_file_to_toml(&args[1])),
        _ => {
            println!("usage:
glstyleconv [jsonfilename]");
        }
    }
}


#[cfg(test)]
extern crate toml;

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
    assert_eq!(json::parse_value("-1"), Ok(ast::Value::Integer(-1)));
    assert_eq!(json::parse_value("3.5"), Ok(ast::Value::Float(3.5)));
    assert_eq!(json::parse_value("2.5e10"), Ok(ast::Value::Float(2.5e10)));
    assert_eq!(json::parse_value("2e-10"), Ok(ast::Value::Float(2e-10)));
    assert_eq!(json::parse_value("true"), Ok(ast::Value::Boolean(true)));
    assert_eq!(json::parse_value("false"), Ok(ast::Value::Boolean(false)));
}

#[test]
fn json_to_toml() {
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

    let json = r#"{"circle-radius": {"stops": [[5, 1],[10, 2]]}}"#;
    let toml = r#"[circle-radius]
stops = [{ in = 5, out = 1 }, { in = 10, out = 2 }]
"#;

    let tomlparsed = json::parse_json(json).unwrap();
    println!("{}", tomlparsed);
    assert_eq!(format!("{}", tomlparsed), toml);
    //assert_eq!(tomlparsed, ast::Value::Table(toml::Parser::new(tomlshort).parse().unwrap()));

    let json = r#"{"circle-radius": {"property": "temperature","stops": [[{"zoom": 0, "value": 0}, 0],[{"zoom": 20, "value": 5}, 20]]}}"#;
    let toml = r#"[circle-radius]
property = "temperature"
stops = [{ in = { zoom = 0, value = 0 }, out = 0 }, { in = { zoom = 20, value = 5 }, out = 20 }]
"#;
    let tomlparsed = json::parse_json(json).unwrap();
    println!("{}", tomlparsed);
    assert_eq!(format!("{}", tomlparsed), toml);
    assert!(toml::Parser::new(toml).parse().is_some());
}

#[test]
fn mstudio_to_toml() {
    let toml = json_file_to_toml("testdata/bright-v9-cdn.json");
    println!("{}", toml);
    assert!(toml.contains(r##"[[layers]]
id = "background"
type = "background"
interactive = true

[layers.paint]
background-color = "#f8f4f0"

[[layers]]
id = "landuse_overlay_national_park"
type = "fill"
source = "mapbox"
source-layer = "landuse_overlay"
filter = ["==", "class", "national_park"]
interactive = true

[layers.paint]
fill-color = "#d8e8c8"
fill-opacity = 0.75

[layers.metadata]
"mapbox:group" = "1444849388993.3071"

[[layers]]
id = "landuse_park"
type = "fill"
source = "mapbox"
source-layer = "landuse"
filter = ["==", "class", "park"]
interactive = true
"##));

    assert!(toml.contains(r##"[[layers]]
interactive = true
filter = ["==", "scalerank", 1]
type = "symbol"
source = "mapbox"
id = "country_label_1"
source-layer = "country_label"

[layers.layout]
text-font = ["Open Sans Bold"]
text-field = "{name_en}"
text-max-width = 6.25
text-transform = "uppercase"

[layers.layout.text-size]
stops = [{ in = 1, out = 11 }, { in = 4, out = 17 }]
"##));

assert!(toml.contains(r##"
[layers.paint]
text-color = "#334"
text-halo-color = "rgba(255,255,255,0.8)"
text-halo-width = 2
text-halo-blur = 1
"##));

    assert!(toml.contains(r#"filter = [["all"], ["!=", "class", "river"], ["!=", "class", "stream"], ["!=", "class", "canal"]]"#));
}
