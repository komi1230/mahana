use std::io::{self, Write};
use std::process::Command;

fn main() {
    let output = Command::new("wasm-pack")
        .arg("--version")
        .output()
        .expect("wasm-pack is not installed.");

    println!("Status Code : {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap()
}
