use std::fmt;

/// Representation of a TOML value.
#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Datetime(String),
    Array(Array),
    Table(Table),
}

/// Type representing a TOML array, payload of the `Value::Array` variant
pub type Array = Vec<Value>;

pub type KeyValuePair = (String, Value);

/// Type representing a TOML table, payload of the `Value::Table` variant
pub type Table = Vec<KeyValuePair>;


struct Printer<'a, 'b:'a> {
    output: &'a mut fmt::Formatter<'b>,
    stack: Vec<&'a str>,
}

struct Key<'a>(&'a [&'a str]);

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::String(ref s) => write_str(f, s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fp) => {
                try!(write!(f, "{}", fp));
                if fp % 1.0 == 0.0 { try!(write!(f, ".0")) }
                Ok(())
            }
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Datetime(ref s) => write!(f, "{}", s),
            Value::Table(ref t) => {
                let mut p = Printer { output: f, stack: Vec::new() };
                p.print(t)
            }
            Value::Array(ref a) => {
                try!(write!(f, "["));
                for (i, v) in a.iter().enumerate() {
                    if i != 0 { try!(write!(f, ", ")); }
                    try!(write!(f, "{}", v));
                }
                write!(f, "]")
            }
        }
    }
}

fn write_str(f: &mut fmt::Formatter, s: &str) -> fmt::Result {
    try!(write!(f, "\""));
    for ch in s.chars() {
        match ch {
            '\u{8}' => try!(write!(f, "\\b")),
            '\u{9}' => try!(write!(f, "\\t")),
            '\u{a}' => try!(write!(f, "\\n")),
            '\u{c}' => try!(write!(f, "\\f")),
            '\u{d}' => try!(write!(f, "\\r")),
            '\u{22}' => try!(write!(f, "\\\"")),
            '\u{5c}' => try!(write!(f, "\\\\")),
            ch => try!(write!(f, "{}", ch)),
        }
    }
    write!(f, "\"")
}

impl<'a, 'b> Printer<'a, 'b> {
    fn print(&mut self, table: &'a Table) -> fmt::Result {
        let mut space_out_first = false;
        for pair in table.iter() {
            let (k, v) = (&pair.0, &pair.1);
            match *v {
                Value::Table(..) => continue,
                Value::Array(ref a) => {
                    if let Some(&Value::Table(..)) = a.first() {
                        continue;
                    }
                }
                _ => {}
            }
            space_out_first = true;
            try!(writeln!(self.output, "{} = {}", Key(&[k]), v));
        }
        for (i, pair) in table.iter().enumerate() {
            let (k, v) = (&pair.0, &pair.1);
            match *v {
                Value::Table(ref inner) => {
                    self.stack.push(k);
                    if space_out_first || i != 0 {
                        try!(write!(self.output, "\n"));
                    }
                    try!(writeln!(self.output, "[{}]", Key(&self.stack)));
                    try!(self.print(inner));
                    self.stack.pop();
                }
                Value::Array(ref inner) => {
                    match inner.first() {
                        Some(&Value::Table(..)) => {}
                        _ => continue
                    }
                    self.stack.push(k);
                    for (j, inner) in inner.iter().enumerate() {
                        if space_out_first || i != 0 || j != 0 {
                            try!(write!(self.output, "\n"));
                        }
                        try!(writeln!(self.output, "[[{}]]", Key(&self.stack)));
                        match *inner {
                            Value::Table(ref inner) => try!(self.print(inner)),
                            _ => panic!("non-heterogeneous toml array"),
                        }
                    }
                    self.stack.pop();
                }
                _ => {},
            }
        }
        Ok(())
    }
}

impl<'a> fmt::Display for Key<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, part) in self.0.iter().enumerate() {
            if i != 0 { try!(write!(f, ".")); }
            let ok = part.chars().all(|c| {
                match c {
                    'a' ... 'z' |
                    'A' ... 'Z' |
                    '0' ... '9' |
                    '-' | '_' => true,
                    _ => false,
                }
            });
            if ok {
                try!(write!(f, "{}", part));
            } else {
                try!(write_str(f, part));
            }
        }
        Ok(())
    }
}
