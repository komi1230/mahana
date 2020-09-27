#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    fn get_sample() -> String {
        let mut file = File::open("./sample/index.json").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    }
}
