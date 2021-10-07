use std::process::Command;
use crate::filesystem::read_file;

#[cfg(not(any(feature = "yarv", feature="full")))]
const EXECUTABLE: &str = "mruby";

#[cfg(any(feature = "yarv", feature="full"))]
const EXECUTABLE: &str = "ruby";

pub fn execute_code(code: &str) -> String {
    let output = Command::new(EXECUTABLE)
        .arg("-e")
        .arg(code)
        .output()
        .expect("Couldn't get output from mruby");

    String::from_utf8(output.stdout).expect("Non utf-8 output from mruby")
}

pub fn execute_file(file: &str) -> String {
    execute_code(&read_file(file))
}
