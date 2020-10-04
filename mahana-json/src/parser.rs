use std::collections::HashMap;

use crate::{Number, Value};

pub fn parse_arr(mut cs: std::str::Chars) -> Result<(Value, std::str::Chars), String> {
    let numbers: Vec<char> = (0..9)
        .map(|item| std::char::from_digit(item as u32, 10).unwrap())
        .collect();
    let mut content: Vec<Value> = Vec::new();
    while let Some(c) = cs.next() {
        // string
        if c == '"' {
            let mut word = String::new();
            while let Some(next_c) = cs.next() {
                if next_c == '"' {
                    content.push(Value::String(word));
                    break;
                } else {
                    word.push(c);
                }
            }
        }
        // number
        if numbers.contains(&c) {
            let mut num = String::new();
            while let Some(next_c) = cs.next() {
                if next_c == ',' {
                    break;
                } else if next_c == ']' {
                    return Ok((Value::Array(content), cs));
                } else {
                    num.push(next_c);
                }
            }
            if let Ok(i) = num.parse::<i32>() {
                content.push(Value::Number(Number::Int(i)))
            } else if let Ok(f) = num.parse::<f64>() {
                content.push(Value::Number(Number::Float(f)));
            } else {
                return Err("Parse Error".to_string());
            }
        }
        // array
        if c == '[' {
            if let Ok((result, tmp_cs)) = parse_arr(cs.clone()) {
                cs = tmp_cs;
                content.push(result);
            } else {
                return Err("Parse Error".to_string());
            }
        }
        // object
        if c == '{' {
            if let Ok((result, tmp_cs)) = parse_object(cs.clone()) {
                cs = tmp_cs;
                content.push(result);
            } else {
                return Err("Parse Error".to_string());
            }
        }
    }
    Ok((Value::Array(content), cs))
}

pub fn parse_object(mut cs: std::str::Chars) -> Result<(Value, std::str::Chars), String> {
    let content: HashMap<String, Value> = HashMap::new();

    Err("Hello".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_array() {
        let test_input = "[12, 34, \"hoge\"]".chars();
        assert!(parse_arr(test_input).is_ok());
    }
}
