mod parser;
mod util;

use std::collections::HashMap;

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
