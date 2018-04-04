extern crate encoding_rs;
extern crate serde_json;

use std::io::{self, BufRead, Read};

use encoding_rs::UTF_8;
use serde_json::{Error, Map, Value};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut stdin = std::io::stdin();
    let json = string_from_reader(&mut stdin).expect("Failed to read from stdin");

    let d = match parse_json(&json) {
        Ok(x) => flatten_value(&x, ""),
        Err(e) => "Invalid JSON".to_string(),
    };

    println!("{}", d);
}

fn flatten_value(data: &Value, path: &str) -> String {
    if is_scalar(data) {
        format!("{} = {};\n", path, data.to_string())
    } else if data.is_object() {
        flatten_object(data.as_object().unwrap(), path)
    } else if data.is_array() {
        flatten_array(data.as_array().unwrap(), path)
    } else {
        "".to_string()
    }
}

fn is_scalar(value: &Value) -> bool {
    if value.is_boolean() || value.is_null() || value.is_number() || value.is_string() {
        true
    } else {
        false
    }
}

fn flatten_object(obj: &Map<String, Value>, path: &str) -> String {
    let mut s = String::new();
    for (k, v) in obj {
        if v.is_object() {
            s.push_str(format!("{}.{} = {{}};\n", path, k).as_str())
        } else if v.is_array() {
            s.push_str(format!("{}.{} = [];\n", path, k).as_str())
        }

        s.push_str(flatten_value(v, format!("{}.{}", path, k).as_str()).as_str());
    }

    return s;
}

fn flatten_array(arr: &Vec<Value>, path: &str) -> String {
    let mut s = String::new();

    for (idx, v) in arr.iter().enumerate() {
        if v.is_object() {
            s.push_str(format!("{}[{}] = {{}};\n", path, idx).as_str())
        } else if v.is_array() {
            s.push_str(format!("{}[{}] = [];\n", path, idx).as_str())
        }
        s.push_str(flatten_value(v, format!("{}[{}]", path, idx).as_str()).as_str());
    }

    return s;
}

/// Safely decode from a Read trait to a String by correctly handling potential
/// UTF-8 BOMs, etc. To do this, we go via the encoding_rs rather than reading
/// directly to a Rust string which leaves the BOM in place, causing Serde to
/// fail when it sees a codepoint it knows isn't valid JSON.
fn string_from_reader(reader: &mut Read) -> Result<String, std::io::Error> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let (res, _) = UTF_8.decode_with_bom_removal(&buffer);
    Ok(res.to_string())
}

fn parse_json(json: &str) -> Result<Value, Error> {
    serde_json::from_str::<Value>(json)
}
