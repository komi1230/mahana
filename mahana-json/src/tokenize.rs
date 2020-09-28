pub fn tokenize(data: String) -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    let mut cs = data.chars();
    let numbers: Vec<char> = (0..10)
        .map(|item| std::char::from_digit(item as u32, 10).unwrap())
        .collect();

    loop {
        let chr = cs.next();

        match chr {
            None => break,
            Some(c) => {
                // brace
                if c == '{' || c == '}' || c == ':' {
                    content.push(c.to_string());
                }

                // string
                if c == '\"' {
                    let mut word = String::new();
                    word.push(c);
                    while let Some(next_c) = cs.next() {
                        if next_c == '\"' {
                            word.push(next_c);
                            break;
                        } else {
                            word.push(next_c);
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
                    }
                }

                // number
                if numbers.contains(&c) {
                    let mut word = String::new();
                    word.push(c);
                    while let Some(next_c) = cs.next() {
                        if !numbers.contains(&next_c) {
                            break;
                        } else {
                            word.push(next_c);
                        }
                    }
                    content.push(word);
                }
            }
        }
    }
    content
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
    fn test_tokenize() {
        let data = get_sample();
        let tokens = tokenize(data);
        let answer: Vec<String> = [
            "{",
            "\"name\"",
            ":",
            "\"Tanaka\"",
            "\"age\"",
            ":",
            "26",
            "}",
        ]
        .iter()
        .map(|item| item.to_string())
        .collect();
        assert_eq!(tokens, answer);
    }
}
