use std::process::Command;
use crate::filesystem::read_file;

pub fn execute_code(code: &str) -> String {
    let output = Command::new("mruby")
        .arg("-e")
        .arg(code)
        .output()
        .expect("Couldn't get output from mruby");

    String::from_utf8(output.stdout).expect("Non utf-8 output from mruby")
}

pub fn execute_file(file: &str) -> String {
    let output = Command::new("mruby")
        .arg("-e")
        .arg(read_file(file))
        .output()
        .expect("Couldn't get output from mruby");

    String::from_utf8(output.stdout).expect("Non utf-8 output from mruby")
}
