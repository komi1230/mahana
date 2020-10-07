mod parser;
mod util;

use std::collections::HashMap;

use parser::parse_object;
use util::expect_token;

pub enum Number {
    Int(i32),
    Float(f64),
}

pub enum Value {
    String(String),
    Number(Number),
    Boolean(bool),
    Null,
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

// pub fn parse(seq: String) -> Result<HashMap<String, Value>, String> {
//     let cs = seq.chars();
//     if let Some(c) = cs.next() {
//         if c == '{' {}
//     } else {
//         return Err("Parse Error".to_string());
//     }
// }
