use std::str::Chars;

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
pub fn expect_comma(mut cs: Chars) -> Result<Chars, String> {
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
