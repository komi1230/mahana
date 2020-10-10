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

pub fn expect_token(cs: &mut Chars) -> Option<char> {
    while let Some(next_c) = cs.next() {
        if next_c == ' ' || next_c == '\n' {
            continue;
        } else {
            return Some(next_c);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Number;

    #[test]
    fn test_parse_number() {
        let test_num1 = "32";
        let test_num2 = "1.23";
        let test_num3 = "hello";
        let test_num4 = "";
        assert_eq!(read_number(test_num1).unwrap(), Number::Int(32));
        assert_eq!(read_number(test_num2).unwrap(), Number::Float(1.23));
        assert!(read_number(test_num3).is_err());
        assert!(read_number(test_num4).is_err());
    }

    #[test]
    fn test_expect_token() {
        // case1
        let mut cs1 = "   ,".chars();
        assert_eq!(expect_token(&mut cs1).unwrap(), ',');

        // case2
        let mut cs2 = "".chars();
        assert!(expect_token(&mut cs2).is_none());

        // case3
        let mut cs3 = "     ".chars();
        assert!(expect_token(&mut cs3).is_none());
    }
}
