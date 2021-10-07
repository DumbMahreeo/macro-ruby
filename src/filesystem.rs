use std::fs::read_to_string;

pub fn read_file(filepath: &str) -> String {
    let mut err_msg = String::from("Couldn't read file: ");
    err_msg.push_str(filepath);
    read_to_string(filepath).expect(&err_msg)
}
