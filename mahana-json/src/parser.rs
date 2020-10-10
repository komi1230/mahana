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

    if let Some(c) = expect_token(cs) {
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
            }
            continue;
        }
        // normal character
        word.push(next_c);
    }
    // wait for special token
    if let Some(next_c) = expect_token(cs) {
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
    if let Some(next_c) = expect_token(cs) {
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
    if token != "false".to_string() && token != "true".to_string() {
        return Err("Parse Error".to_string());
    }

    // expect special token
    if let Some(next_c) = expect_token(cs) {
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
    if let Some(next_c) = expect_token(cs) {
        return Ok((Value::Array(content), Some(next_c)));
    }
    Err("Parse Error".to_string())
}

fn parse_object_key(cs: &mut Chars) -> Option<String> {
    if let Some(c) = expect_token(cs) {
        if c == '"' {
            if let Ok((result, next_c)) = parse_string(cs) {
                match next_c {
                    Some(x) if x == ':' => {
                        if let Value::String(seq) = result {
                            return Some(seq);
                        }
                    }
                    _ => return None,
                }
            }
        }
    }
    None
}

pub fn parse_object(cs: &mut Chars) -> Result<(Value, Option<char>), String> {
    let numbers: Vec<char> = (0..9)
        .map(|item| std::char::from_digit(item as u32, 10).unwrap())
        .collect();
    let mut content: HashMap<String, Value> = HashMap::new();

    while let Some(c) = expect_token(cs) {
        // termination
        if c == '}' {
            break;
        }

        // get key
        let mut key = String::new();
        if c == '"' {
            if let Ok((result, next_c)) = parse_string(cs) {
                if let Value::String(seq) = result {
                    if next_c == Some(':') {
                        key = seq;
                    }
                } else {
                    return Err("Parse Error".to_string());
                }
            }
        } else {
            return Err("Parse Error".to_string());
        }

        // get value
        let init_c = if let Some(c) = expect_token(cs) {
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

    // wait for special token
    if let Some(next_c) = expect_token(cs) {
        return Ok((Value::Object(content), Some(next_c)));
    } else {
        return Ok((Value::Object(content), None));
    }
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

    #[test]
    fn test_parse_string() {
        // case1
        let mut cs1 = "hoge\",".chars();
        assert_eq!(
            parse_string(&mut cs1).unwrap(),
            (Value::String("hoge".to_string()), Some(','))
        );

        // case2
        let mut cs2 = "   \"    ]".chars();
        assert_eq!(
            parse_string(&mut cs2).unwrap(),
            (Value::String("   ".to_string()), Some(']'))
        );

        // case3
        let mut cs3 = ",".chars();
        assert!(parse_string(&mut cs3).is_err());
    }

    #[test]
    fn test_parse_null() {
        // case1
        let c1 = 'n';
        let mut cs1 = "ull, ".chars();
        assert_eq!(parse_null(c1, &mut cs1).unwrap(), (Value::Null, Some(',')));

        // case2
        let c2 = 'n';
        let mut cs2 = "hoge    ]".chars();
        assert!(parse_null(c2, &mut cs2).is_err());
    }

    #[test]
    fn test_parse_bool() {
        // case1
        let c1 = 't';
        let mut cs1 = "rue,".chars();
        assert_eq!(
            parse_bool(c1, &mut cs1).unwrap(),
            (Value::Boolean(true), Some(','))
        );

        // case2
        let c2 = 'f';
        let mut cs2 = "alse    }".chars();
        assert_eq!(
            parse_bool(c2, &mut cs2).unwrap(),
            (Value::Boolean(false), Some('}'))
        );

        // case3
        let c3 = 't';
        let mut cs3 = "hoge   \"]".chars();
        assert!(parse_bool(c3, &mut cs3).is_err());
    }

    #[test]
    fn test_parse_array() {
        // case1  == ... [10, 30,] }
        let mut cs1 = "10, 30,]}".chars();
        assert_eq!(
            parse_array(&mut cs1).unwrap(),
            (
                Value::Array(vec![
                    Value::Number(Number::Int(10)),
                    Value::Number(Number::Int(30))
                ]),
                Some('}')
            )
        );

        // case2
        let mut cs2 = "null, false, \"hello\"     ,      ]        ,".chars();
        assert_eq!(
            parse_array(&mut cs2).unwrap(),
            (
                Value::Array(vec![
                    Value::Null,
                    Value::Boolean(false),
                    Value::String("hello".to_string())
                ]),
                Some(',')
            )
        );

        // case3
        let mut cs3 = "]]".chars();
        assert_eq!(
            parse_array(&mut cs3).unwrap(),
            (Value::Array(vec![]), Some(']'))
        );

        // case4
        let mut cs4 = "{}]  ,".chars();
        assert!(parse_array(&mut cs4).is_ok());

        // case5
        let mut cs5 = "{\"hello\": 10}, ] , ".chars();
        assert!(parse_array(&mut cs5).is_ok());
    }

    #[test]
    fn test_parse_object() {
        // case1
        let mut cs1 = "}".chars();
        assert_eq!(parse_object(&mut cs1).unwrap().1, None);

        // case2
        let mut cs2 = "   }   , ".chars();
        assert_eq!(parse_object(&mut cs2).unwrap().1, Some(','));

        // case3
        let mut cs3 = "\"hello\": 10}, ] , ".chars();
        assert_eq!(parse_object(&mut cs3).unwrap().1, Some(','));
    }
}
