use std::collections::HashMap;
use std::str::Chars;

use crate::util::{expect_token, read_number};
use crate::Value;

fn parse_number(c: char, mut cs: Chars) -> Result<(Value, Chars), String> {
    let mut num = String::new();
    num.push(c);
    while let Some(next_c) = cs.next() {
        if next_c == ',' || next_c == ']' || next_c == '}' {
            break;
        } else {
            num.push(next_c);
        }
    }
    if let Ok(n) = read_number(num) {
        return Ok((Value::Number(n), cs));
    } else {
        return Err("Parse Error".to_string());
    }
}

fn parse_string(c: char, mut cs: Chars) -> Result<(Value, Chars), String> {
    let mut word = String::new();
    word.push(c);
    while let Some(next_c) = cs.next() {
        if next_c == '"' {
            return Ok((Value::String(word), cs));
        } else if next_c == '\\' {
            word.push(next_c);
            if let Some(x) = cs.next() {
                word.push(x);
            } else {
                return Err("Parse Error".to_string());
            }
        } else {
            word.push(next_c);
        }
    }
    Err("Parse Error".to_string())
}

fn parse_arr(mut cs: Chars) -> Result<(Value, Chars), String> {
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
            if let Ok(next_cs) = expect_token(cs) {
                cs = next_cs;
            } else {
                return Err("Parse Error".to_string());
            }
        }

        // number
        if numbers.contains(&c) {
            if let Ok((result, tmp_cs)) = parse_number(c, cs) {
                content.push(result);
                cs = tmp_cs;
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
            if let Ok(next_cs) = expect_token(cs) {
                cs = next_cs;
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
            if let Ok(next_cs) = expect_token(cs) {
                cs = next_cs;
            } else {
                return Err("Parse Error".to_string());
            }
        }
    }
    Ok((Value::Array(content), cs))
}

fn parse_object(mut cs: Chars) -> Result<(Value, Chars), String> {
    let content: HashMap<String, Value> = HashMap::new();
    while let Some(c) = cs.next() {}
    Ok((Value::Object(content), cs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_array() {
        let test_input = "[12, [34], \"hoge\"]".chars();
        assert!(parse_arr(test_input).is_ok());
    }
}
