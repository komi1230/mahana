use std::collections::HashMap;

use crate::value::{Number, Value};

pub fn parse(seq: String) -> Result<HashMap<String, Value>, String> {
    let result: HashMap<String, Value> = HashMap::new();
    let cs = seq.chars();
    let braces = ['{', '}'];
    Ok(result)
}
