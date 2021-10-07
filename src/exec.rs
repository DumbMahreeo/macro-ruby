use std::process::Command;
pub fn execute(code: &str) -> String {
    let output = Command::new("mruby")
        .arg("-e")
        .arg(code)
        .output()
        .expect("Couldn't get output from mruby");

    String::from_utf8(output.stdout).expect("Non utf-8 output from mruby")
}
