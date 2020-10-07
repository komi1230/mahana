mod parser;
mod util;

use std::collections::HashMap;

use parser::parse_object;

#[derive(Debug, PartialEq)]
pub enum Number {
    Int(i32),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(Number),
    Boolean(bool),
    Null,
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

pub fn parse(seq: String) -> Result<HashMap<String, Value>, String> {
    let mut cs = seq.chars();
    if let Some(c) = cs.next() {
        if c == '{' {
            if let Ok((result, _c)) = parse_object(&mut cs) {
                if let Value::Object(data) = result {
                    return Ok(data);
                } else {
                    return Err("Parse Error".to_string());
                }
            } else {
                return Err("Parse Error".to_string());
            }
        } else {
            return Err("Parse Error".to_string());
        }
    } else {
        return Err("Parse Error".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn get_json() -> String {
        let file_path = "sample/small.json";
        let content = fs::read_to_string(file_path).unwrap();
        content
    }

    #[test]
    fn hello() {
        println!("{}", get_json());
    }
}
