use std::str::Chars;

use crate::Number;

pub fn read_number(seq: &str) -> Result<Number, String> {
    match seq.parse::<i32>() {
        Ok(i) => Ok(Number::Int(i)),
        Err(_) => match seq.parse::<f64>() {
            Ok(f) => Ok(Number::Float(f)),
            Err(_) => Err("Not number".to_string()),
        },
    }
}

pub fn expect_token(cs: &mut Chars) -> Result<char, String> {
    while let Some(next_c) = cs.next() {
        if next_c == ' ' {
            continue;
        } else {
            return Ok(next_c);
        }
    }
    Err("Parse Error".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let test_num1 = "32";
        let test_num2 = "1.23";
        let test_num3 = "hello";
        assert!(read_number(test_num1).is_ok());
        assert!(read_number(test_num2).is_ok());
        assert!(read_number(test_num3).is_err());
    }
}
