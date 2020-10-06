use std::collections::HashMap;
use std::str::Chars;

use crate::util::{expect_token, read_number};
use crate::Value;

fn parse_number(c: char, mut cs: Chars) -> Result<(Value, char, Chars), String> {
    let mut num = String::new();
    num.push(c);
    while let Some(next_c) = cs.next() {
        // space has no meaning
        if next_c == ' ' {
            continue;
        }
        // these are special token
        if next_c == ',' || next_c == ']' || next_c == '}' {
            if let Ok(n) = read_number(num) {
                return Ok((Value::Number(n), next_c, cs));
            } else {
                break;
            }
        }
        // maybe kind of number character
        num.push(next_c);
    }
    Err("Parse Error".to_string())
}

fn parse_string(c: char, mut cs: Chars) -> Result<(Value, char, Chars), String> {
    let mut word = String::new();
    word.push(c);
    // process strin
    while let Some(next_c) = cs.next() {
        // end of string
        if next_c == '"' {
            break;
        }
        // escape character
        if next_c == '\\' {
            word.push(next_c);
            if let Some(x) = cs.next() {
                word.push(x);
            } else {
                return Err("Parse Error".to_string());
            }
            continue;
        }
        // normal character
        word.push(next_c);
    }
    // wait for special token
    if let Ok((next_c, new_cs)) = expect_token(cs) {
        return Ok((Value::String(word), next_c, new_cs));
    }
    Err("Parse Error".to_string())
}

fn parse_null(c: char, mut cs: Chars) -> Result<(Value, char, Chars), String> {
    let mut token = String::new();
    token.push(c);
    for _ in 0..3 {
        if let Some(next_c) = cs.next() {
            token.push(next_c);
        } else {
            break;
        }
    }
    if token != "null".to_string() {
        return Err("Parse Error".to_string());
    }
    // wait for special token
    if let Ok((next_c, new_cs)) = expect_token(cs) {
        return Ok((Value::Null, next_c, new_cs));
    }
    Err("Parse Error".to_string())
}

fn parse_bool(c: char, mut cs: Chars) -> Result<(Value, char, Chars), String> {
    let mut token = String::new();
    token.push(c);
    if c == 't' {
        for _ in 0..3 {
            if let Some(next_c) = cs.next() {
                token.push(next_c);
            } else {
                break;
            }
        }
    }
    if c == 'f' {
        for _ in 0..4 {
            if let Some(next_c) = cs.next() {
                token.push(next_c);
            } else {
                break;
            }
        }
    }

    // token should be "true" or "false"
    if token != "false".to_string() || token != "true".to_string() {
        return Err("Parse Error".to_string());
    }

    // expect special token
    if let Ok((next_c, new_cs)) = expect_token(cs) {
        if token == "true".to_string() {
            return Ok((Value::Boolean(true), next_c, new_cs));
        } else {
            return Ok((Value::Boolean(false), next_c, new_cs));
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
            if let Ok((result, _next_c, tmp_cs)) = parse_string(c, cs) {
                cs = tmp_cs;
                content.push(result);
            } else {
                return Err("Parse Error".to_string());
            }
        }

        // number
        if numbers.contains(&c) {
            if let Ok((result, _next_c, tmp_cs)) = parse_number(c, cs) {
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
    while let Some(c) = cs.next() {
        // ignore space
        if c == ' ' {
            continue;
        }
    }
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
