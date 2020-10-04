fn is_number(seq: &str) -> bool {
    match seq.parse::<i32>() {
        Ok(_) => true,
        Err(_) => match seq.parse::<f64>() {
            Ok(_) => true,
            Err(_) => false,
        },
    }
}

// expect comma
pub fn expect_comma(mut cs: std::str::Chars) -> Result<std::str::Chars, String> {
    while let Some(next_c) = cs.next() {
        if next_c == ' ' {
            continue;
        } else if next_c == ',' {
            break;
        } else {
            return Err("Parse Error".to_string());
        }
    }
    Ok(cs)
}

pub fn tokenize(data: String) -> Result<Vec<String>, String> {
    let mut content: Vec<String> = Vec::new();
    let mut cs = data.chars();
    let special_chars = ['{', '}', '[', ']', ':', ','];
    let mut numbers: Vec<char> = (0..10)
        .map(|item| std::char::from_digit(item as u32, 10).unwrap())
        .collect();
    numbers.push('.');
    numbers.push('e');
    numbers.push('-');

    loop {
        let chr = cs.next();

        match chr {
            None => break,
            Some(c) => {
                // braces, parenthesis, colon and comma
                if special_chars.contains(&c) {
                    content.push(c.to_string());
                }

                // string
                if c == '\"' {
                    let mut word = String::new();
                    word.push(c);
                    while let Some(next_c) = cs.next() {
                        match next_c {
                            // escape
                            '\\' => {
                                word.push(next_c);
                                match cs.next() {
                                    Some(x) => word.push(x),
                                    None => return Err("Parse Error".to_string()),
                                };
                                continue;
                            }
                            // double quotation
                            '\"' => {
                                word.push(next_c);
                                break;
                            }
                            _ => {
                                word.push(next_c);
                            }
                        }
                    }
                    content.push(word);
                }

                // boolean
                if c == 't' {
                    let mut word = String::new();
                    word.push(c);
                    for _ in 0..3 {
                        word.push(cs.next().unwrap());
                    }
                    if word == "true".to_string() {
                        content.push(word);
                    } else {
                        return Err("Parse Error".to_string());
                    }
                }
                if c == 'f' {
                    let mut word = String::new();
                    word.push(c);
                    for _ in 0..4 {
                        word.push(cs.next().unwrap());
                    }
                    if word == "false".to_string() {
                        content.push(word);
                    } else {
                        return Err("Parse Error".to_string());
                    }
                }

                // null
                if c == 'n' {
                    let mut word = String::new();
                    word.push(c);
                    for _ in 0..3 {
                        word.push(cs.next().unwrap());
                    }
                    if word == "null".to_string() {
                        content.push(word);
                    } else {
                        return Err("Parse Error".to_string());
                    }
                }

                // number
                if numbers.contains(&c) {
                    let mut word = String::new();
                    word.push(c);
                    while let Some(next_c) = cs.next() {
                        if !numbers.contains(&next_c) {
                            if next_c == ',' {
                                if is_number(word.as_str()) {
                                    content.push(word);
                                } else {
                                    return Err("Parse Error".to_string());
                                }
                                content.push(next_c.to_string());
                                break;
                            } else {
                                return Err("Parse Error".to_string());
                            }
                        } else {
                            word.push(next_c);
                        }
                    }
                }
            }
        }
    }
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    fn get_sample() -> String {
        let mut file = File::open("./sample/small.json").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    }

    #[test]
    fn test_get_sample() {
        let data = get_sample();
        assert!(data.len() > 0);
    }

    #[test]
    fn test_is_number() {
        assert!(is_number("3.2"));
        assert!(is_number("-20"));
        assert!(!is_number("10.9.8"));
        assert!(!is_number("hello"))
    }

    #[test]
    fn test_tokenize() {
        let data = get_sample();
        let tokens = tokenize(data);
        let answer: Vec<String> = [
            "{",
            "\"name\"",
            ":",
            "\"Tanaka\"",
            ",",
            "\"age\"",
            ":",
            "26.3",
            ",",
            "\"animal\"",
            ":",
            "true",
            ",",
            "\"color_list\"",
            ":",
            "[",
            "\"red\"",
            ",",
            "\"green\"",
            ",",
            "\"blue\"",
            "]",
            ",",
            "}",
        ]
        .iter()
        .map(|item| item.to_string())
        .collect();
        assert_eq!(tokens.unwrap(), answer);
    }
}
