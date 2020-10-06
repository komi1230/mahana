use std::str::Chars;

use crate::Number;

pub fn read_number(seq: String) -> Result<Number, String> {
    match seq.parse::<i32>() {
        Ok(i) => Ok(Number::Int(i)),
        Err(_) => match seq.parse::<f64>() {
            Ok(f) => Ok(Number::Float(f)),
            Err(_) => Err("Not number".to_string()),
        },
    }
}

pub fn expect_token(mut cs: Chars) -> Result<(char, Chars), String> {
    let tokens = [',', ']', '}'];
    while let Some(next_c) = cs.next() {
        if next_c == ' ' {
            continue;
        } else if tokens.contains(&next_c) {
            return Ok((next_c, cs));
        } else {
            break;
        }
    }
    Err("Parse Error".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let test_num1 = "32".to_string();
        let test_num2 = "1.23".to_string();
        let test_num3 = "hello".to_string();
        assert!(read_number(test_num1).is_ok());
        assert!(read_number(test_num2).is_ok());
        assert!(read_number(test_num3).is_err());
    }
}
