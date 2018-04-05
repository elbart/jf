extern crate encoding_rs;
extern crate serde_json;

use std::io::{Read, Write, BufWriter, Stdout};

use encoding_rs::UTF_8;
use serde_json::{Error, Map, Value};

fn main() {
    let mut stdin = std::io::stdin();
    let json = read_json_to_string(&mut stdin).expect("Failed to read from stdin");

    let mut writer = std::io::BufWriter::new(std::io::stdout());

    match parse_json(&json) {
        Ok(x) => flatten_value(&x, "", &mut writer),
        Err(_) => (),
    }
}

fn flatten_value(data: &Value, path: &str, writer: &mut BufWriter<Stdout>) {
    if is_scalar(data) {
        write!(writer, "{} = {};\n", path, data.to_string());
    } else if data.is_object() {
        flatten_object(data.as_object().unwrap(), path, writer)
    } else if data.is_array() {
        flatten_array(data.as_array().unwrap(), path, writer)
    }
}

fn is_scalar(value: &Value) -> bool {
    if value.is_boolean() || value.is_null() || value.is_number() || value.is_string() {
        true
    } else {
        false
    }
}

fn flatten_object(obj: &Map<String, Value>, path: &str, writer: &mut BufWriter<Stdout>) {
    for (k, v) in obj {
        if v.is_object() {
            write!(writer, "{}.{} = {{}};\n", path, k);
        } else if v.is_array() {
            write!(writer, "{}.{} = [];\n", path, k);
        }

        flatten_value(v, format!("{}.{}", path, k).as_str(), writer);
    }
}

fn flatten_array(arr: &Vec<Value>, path: &str, writer: &mut BufWriter<Stdout>) {
    for (idx, v) in arr.iter().enumerate() {
        if v.is_object() {
            write!(writer, "{}[{}] = {{}};\n", path, idx);
        } else if v.is_array() {
            write!(writer, "{}[{}] = [];\n", path, idx);
        }
        flatten_value(v, format!("{}[{}]", path, idx).as_str(), writer);
    }
}

fn read_json_to_string(reader: &mut Read) -> Result<String, std::io::Error> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let (res, _) = UTF_8.decode_with_bom_removal(&buffer);
    Ok(res.to_string())
}

fn parse_json(json: &str) -> Result<Value, Error> {
    serde_json::from_str::<Value>(json)
}
