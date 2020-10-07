use std::collections::HashMap;
use std::str::Chars;

use crate::util::{expect_token, read_number};
use crate::Value;

pub fn parse_number(c: char, cs: &mut Chars) -> Result<(Value, Option<char>), String> {
    let mut num = String::new();
    num.push(c);
    while let Some(next_c) = cs.next() {
        // space has no meaning
        if next_c == ' ' {
            break;
        }
        // these are special token
        if next_c == ',' || next_c == ']' || next_c == '}' {
            if let Ok(n) = read_number(&num) {
                return Ok((Value::Number(n), Some(next_c)));
            } else {
                break;
            }
        }
        // maybe kind of number character
        num.push(next_c);
    }

    if let Ok(c) = expect_token(cs) {
        if c == ',' || c == ']' || c == '}' {
            if let Ok(n) = read_number(&num) {
                return Ok((Value::Number(n), Some(c)));
            }
        }
    }
    Err("Parse Error".to_string())
}

pub fn parse_string(cs: &mut Chars) -> Result<(Value, Option<char>), String> {
    let mut word = String::new();
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
    if let Ok(next_c) = expect_token(cs) {
        return Ok((Value::String(word), Some(next_c)));
    }
    Err("Parse Error".to_string())
}

pub fn parse_null(c: char, cs: &mut Chars) -> Result<(Value, Option<char>), String> {
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
    if let Ok(next_c) = expect_token(cs) {
        return Ok((Value::Null, Some(next_c)));
    }
    Err("Parse Error".to_string())
}

pub fn parse_bool(c: char, cs: &mut Chars) -> Result<(Value, Option<char>), String> {
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
    if let Ok(next_c) = expect_token(cs) {
        if token == "true".to_string() {
            return Ok((Value::Boolean(true), Some(next_c)));
        } else {
            return Ok((Value::Boolean(false), Some(next_c)));
        }
    }
    Err("Parse Error".to_string())
}

pub fn parse_array(cs: &mut Chars) -> Result<(Value, Option<char>), String> {
    let numbers: Vec<char> = (0..9)
        .map(|item| std::char::from_digit(item as u32, 10).unwrap())
        .collect();
    let mut content: Vec<Value> = Vec::new();
    while let Some(c) = cs.next() {
        // string
        if c == '"' {
            if let Ok((result, next_c)) = parse_string(cs) {
                content.push(result);
                if next_c == Some(',') {
                    continue;
                }
                if next_c == Some(']') {
                    break;
                }
                return Err("Parse Error".to_string());
            } else {
                return Err("Parse Error".to_string());
            }
        }

        // number
        if numbers.contains(&c) {
            if let Ok((result, next_c)) = parse_number(c, cs) {
                content.push(result);
                if next_c == Some(',') {
                    continue;
                }
                if next_c == Some(']') {
                    break;
                }
                return Err("Parse Error".to_string());
            } else {
                return Err("Parse Error".to_string());
            }
        }

        // boolean
        if c == 't' || c == 'f' {
            if let Ok((result, next_c)) = parse_bool(c, cs) {
                content.push(result);
                if next_c == Some(',') {
                    continue;
                }
                if next_c == Some(']') {
                    break;
                }
                return Err("Parse Error".to_string());
            } else {
                return Err("Parse Error".to_string());
            }
        }

        // null
        if c == 'n' {
            if let Ok((result, next_c)) = parse_null(c, cs) {
                content.push(result);
                if next_c == Some(',') {
                    continue;
                }
                if next_c == Some(']') {
                    break;
                }
                return Err("Parse Error".to_string());
            } else {
                return Err("Parse Error".to_string());
            }
        }

        // array
        if c == '[' {
            if let Ok((result, next_c)) = parse_array(cs) {
                content.push(result);
                if next_c == Some(',') {
                    continue;
                }
                if next_c == Some(']') {
                    break;
                }
                return Err("Parse Error".to_string());
            } else {
                return Err("Parse Error".to_string());
            }
        }

        // object
        if c == '{' {
            if let Ok((result, next_c)) = parse_object(cs) {
                content.push(result);
                if next_c == Some(',') {
                    continue;
                }
                if next_c == Some(']') {
                    break;
                }
                return Err("Parse Error".to_string());
            } else {
                return Err("Parse Error".to_string());
            }
        }

        // end of array
        if c == ']' {
            break;
        }
    }

    // wait for special token
    if let Ok(next_c) = expect_token(cs) {
        return Ok((Value::Array(content), Some(next_c)));
    }
    Err("Parse Error".to_string())
}

pub fn parse_object(cs: &mut Chars) -> Result<(Value, Option<char>), String> {
    let numbers: Vec<char> = (0..9)
        .map(|item| std::char::from_digit(item as u32, 10).unwrap())
        .collect();
    let mut content: HashMap<String, Value> = HashMap::new();

    loop {
        // get key
        let mut key = String::new();
        while let Some(c) = cs.next() {
            if c == '"' {
                if let Ok((result, next_c)) = parse_string(cs) {
                    if let Value::String(seq) = result {
                        key = seq;
                    }
                    if next_c == Some(':') {
                        break;
                    } else {
                        while let Some(x) = cs.next() {
                            if x == ':' {
                                break;
                            }
                        }
                        break;
                    }
                }
            }
        }
        // get value
        let init_c = if let Some(c) = cs.next() {
            c
        } else {
            return Err("Parse Error".to_owned());
        };
        // string
        if init_c == '"' {
            if let Ok((value, next_c)) = parse_string(cs) {
                content.insert(key.to_owned(), value);
                if next_c == Some(',') {
                    continue;
                }
                if next_c == Some('}') {
                    break;
                }
                return Err("Parse Error".to_string());
            } else {
                return Err("Parse Error".to_string());
            }
        }
        // number
        if numbers.contains(&init_c) {
            if let Ok((value, next_c)) = parse_number(init_c, cs) {
                content.insert(key.to_owned(), value);
                if next_c == Some(',') {
                    continue;
                }
                if next_c == Some('}') {
                    break;
                }
                return Err("Parse Error".to_string());
            } else {
                return Err("Parse Error".to_string());
            }
        }
        // boolean
        if init_c == 't' || init_c == 'f' {
            if let Ok((value, next_c)) = parse_bool(init_c, cs) {
                content.insert(key.to_owned(), value);
                if next_c == Some(',') {
                    continue;
                }
                if next_c == Some('}') {
                    break;
                }
                return Err("Parse Error".to_string());
            } else {
                return Err("Parse Error".to_string());
            }
        }
        // null
        if init_c == 'n' {
            if let Ok((value, next_c)) = parse_null(init_c, cs) {
                content.insert(key.to_owned(), value);
                if next_c == Some(',') {
                    continue;
                }
                if next_c == Some('}') {
                    break;
                }
                return Err("Parse Error".to_string());
            } else {
                return Err("Parse Error".to_string());
            }
        }
        // array
        if init_c == '[' {
            if let Ok((value, next_c)) = parse_array(cs) {
                content.insert(key.to_owned(), value);
                if next_c == Some(',') {
                    continue;
                }
                if next_c == Some('}') {
                    break;
                }
                return Err("Parse Error".to_string());
            } else {
                return Err("Parse Error".to_string());
            }
        }
        // object
        if init_c == '{' {
            if let Ok((value, next_c)) = parse_object(cs) {
                content.insert(key.to_owned(), value);
                if next_c == Some(',') {
                    continue;
                }
                if next_c == Some('}') {
                    break;
                }
                return Err("Parse Error".to_string());
            } else {
                return Err("Parse Error".to_string());
            }
        }
        // end of array
        if init_c == '}' {
            break;
        }
    }

    // termination condition
    if let None = cs.next() {
        return Ok((Value::Object(content), None));
    }

    // wait for special token
    if let Ok(next_c) = expect_token(cs) {
        return Ok((Value::Object(content), Some(next_c)));
    }
    Err("Parse Error".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Number, Value};

    #[test]
    fn test_parse_number() {
        // case1
        let c1 = '1';
        let mut cs1 = ".23,".chars();
        assert_eq!(
            parse_number(c1, &mut cs1).unwrap(),
            (Value::Number(Number::Float(1.23)), Some(','))
        );
        assert_eq!(cs1.next(), None);

        // case2
        let c2 = '3';
        let mut cs2 = ".14 ], ".chars();
        assert_eq!(
            parse_number(c2, &mut cs2).unwrap(),
            (Value::Number(Number::Float(3.14)), Some(']'))
        );
        assert_eq!(cs2.next(), Some(','));

        // case3
        let c3 = '4';
        let mut cs3 = "}  ,".chars();
        assert_eq!(
            parse_number(c3, &mut cs3).unwrap(),
            (Value::Number(Number::Int(4)), Some('}'))
        );
        assert_eq!(cs3.next(), Some(' '));

        // case4 (bad)
        let c4 = '4';
        let mut cs4 = " 3,".chars();
        assert!(parse_number(c4, &mut cs4).is_err());
    }
}
