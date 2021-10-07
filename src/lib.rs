mod exec;
mod filesystem;
mod parser;

use crate::{exec::*, parser::parse_str};
use proc_macro::{TokenStream, TokenTree};

const USE_MSG_STR_CODE: &str =
    "Wrong usage:\nUse ruby_code_str!(\"literal\")\nExample: ruby_code_str!(\"print 'something'\")";

const USE_MSG_TYPE_CODE: &str =
    "Wrong usage:\nUse ruby_code_to!(type \"literal\")\nExample: ruby_code_to!(i32 \"print 2+2\")";

const USE_MSG_AST_CODE: &str =
    "Wrong usage:\nUse ruby_code_ast!(\"literal\")\nExample: ruby_code_ast!(\"puts 'let a = 1;'\")";

const USE_MSG_STR_FILE: &str =
    "Wrong usage:\nUse ruby_code_str!(\"filepath\")\nExample: ruby_file_str!(\"print 'something'\")";

const USE_MSG_TYPE_FILE: &str =
    "Wrong usage:\nUse ruby_code_to!(type \"filepath\")\nExample: ruby_file_to!(i32 \"print 2+2\")";

const USE_MSG_AST_FILE: &str =
    "Wrong usage:\nUse ruby_code_ast!(\"filepath\")\nExample: ruby_file_ast!(\"puts 'let a = 1;'\")";


/// Execute Ruby code and return a string
///
/// # Arguments
///
/// * `input` - A &str literal containing ruby code 
///
/// # Example
/// ```
/// use macro_ruby::ruby_code_str;
///
/// let my_str = ruby_code_str!("puts hi");
/// assert_eq!(my_str, "hi\n");
///
/// ```
/// # Note
/// `puts` adds a trailing '\n', use `print` if you don't want that
#[proc_macro]
pub fn ruby_code_str(input: TokenStream) -> TokenStream {
    let input: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    let code = input.get(0).expect(USE_MSG_STR_CODE);

    let code = match code {
        TokenTree::Literal(literal) => parse_str(&literal.to_string()),
        _ => panic!("{}", USE_MSG_STR_CODE),
    };

    let mut result = String::from("r#\"");
    result.push_str(&execute_code(&code));
    result.push_str("\"#");

    result.parse::<TokenStream>().expect(
        "Couldn't parse ruby output, perhaps check for presence of comments in your ruby code",
    )
}

/// Execute Ruby code and return a different type
///
/// # Arguments
/// * `type` - The return type of the ruby code
/// * `input` - A `&str` literal containing ruby code 
///
/// # Example
/// ```
/// use macro_ruby::ruby_code_to;
///
/// let my_int = ruby_code_to!(i32, "print 500+500");
/// assert_eq!(my_int, 1000);
///
/// ```
///
/// # List of supported types
/// `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`
/// `u64`, `u128`, `usize`, `f32`, `f64`, `char`, `bool`
///
/// # Note
/// If your type isn't supported you may use `ruby_code_ast!`;
#[proc_macro]
pub fn ruby_code_to(input: TokenStream) -> TokenStream {
    let input: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    let return_type = input.get(0).expect(USE_MSG_TYPE_CODE);
    let code = input.get(1).expect(USE_MSG_TYPE_CODE);

    let return_type = match return_type {
        TokenTree::Ident(t) => t.to_string(),
        _ => panic!(
            "Should be type but found '{}': {}",
            return_type, USE_MSG_TYPE_CODE
        ),
    };

    let code = match code {
        TokenTree::Literal(literal) => parse_str(&literal.to_string()),
        TokenTree::Punct(_) => match input.get(2).expect(USE_MSG_TYPE_CODE) {
            TokenTree::Literal(literal) => parse_str(&literal.to_string()),
            _ => panic!(
                "Should be string literal but found '{}':\n{}",
                code, USE_MSG_TYPE_CODE
            ),
        },
        _ => panic!(
            "Should be string literal but found '{}':\n{}",
            code, USE_MSG_TYPE_CODE
        ),
    };

    macro_rules! eval_type {
        ( $($type:ty),* ) => {
            match return_type.as_ref() {
                $(stringify!($type) => {

                    let result = execute_code(&code)
                        .trim_end()
                        .parse::<$type>();

                    let result = match result {
                        Ok(v) => v,
                        Err(e) => {
                                panic!("Couldn't convert to type: {}{}",
                                    stringify!($type),
                                    match e.kind() {
                                        &std::num::IntErrorKind::Empty => "\nPerhaps you forgot to print the result?",
                                        _ => ""
                                    }
                                );
                        }
                    };

                    let mut result = result.to_string();
                    result.push_str(concat!(" as ", stringify!($type)));

                    result.parse::<TokenStream>().expect("Couldn't parse token stream")

                }),*

                "f32" => {
                    let result = execute_code(&code)
                        .trim_end()
                        .parse::<f32>();

                    let result = match result {
                        Ok(v) => v,
                        Err(e) => {
                            panic!(
                                "Couldn't convert to type: f32{}",
                                if e == "".parse::<f32>().unwrap_err() { // weird hack
                                    "\nPerhaps you forgot to print the result?"
                                } else {
                                    ""
                                }
                            )
                        }
                    };

                    let mut result = result.to_string();
                    result.push_str(" as f32");

                    result.parse::<TokenStream>().expect("Couldn't parse token stream")
                },

                "f64" => {
                    let result = execute_code(&code)
                        .trim_end()
                        .parse::<f64>();

                    let result = match result {
                        Ok(v) => v,
                        Err(e) => {
                            panic!(
                                "Couldn't convert to type: f64{}",
                                if e == "".parse::<f64>().unwrap_err() { // weird hack
                                    "\nPerhaps you forgot to print the result?"
                                } else {
                                    ""
                                }
                            )
                        }
                    };

                    let mut result = result.to_string();
                    result.push_str(" as f64");

                    result.parse::<TokenStream>().expect("Couldn't parse token stream")
                },

                "char" => {
                    let result = execute_code(&code)
                        .trim_end()
                        .parse::<char>();

                    let result = match result {
                        Ok(v) => v,
                        Err(e) => {
                            panic!(
                                "Couldn't convert to type: char{}",
                                if e == "".parse::<char>().unwrap_err() { // weird hack
                                    "\nPerhaps you forgot to print the result?"
                                } else {
                                    ""
                                }
                            )
                        }
                    };

                    let mut result = result.to_string();
                    result.push_str(" as char");

                    result.parse::<TokenStream>().expect("Couldn't parse token stream")

                }
                _ => panic!("Unsupported type")
            }
        }
    }

    eval_type!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize)
}

/// Execute Ruby code and generate Rust code
///
/// # Arguments
/// * `input` - A `&str` literal containing ruby code 
///
/// # Example
/// ```
/// use macro_ruby::ruby_code_ast;
///
/// ruby_code_ast!(i32, r#"
///
/// 3.times do |x|
///     puts "let var#{x} = #{x};"
/// end
///
/// "#);
/// assert_eq!(var0, 0);
/// assert_eq!(var1, 1);
/// assert_eq!(var2, 2);
/// ```
#[proc_macro]
pub fn ruby_code_ast(input: TokenStream) -> TokenStream {
    let input: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    let code = input.get(0).expect(USE_MSG_AST_CODE);

    let code = match code {
        TokenTree::Literal(literal) => parse_str(&literal.to_string()),
        _ => panic!("{}", USE_MSG_AST_CODE),
    };

    execute_code(&code)
        .parse::<TokenStream>()
        .expect("Couldn't parse ruby output")
}

/// Execute Ruby from a file and return a string
///
/// # Arguments
///
/// * `filepath` - A &str literal containing the file path
///
/// # Example
/// Contents of ./src/main.rs
/// ```
/// use macro_ruby::ruby_file_str;
///
/// let my_str = ruby_file_str!("src/file.rb");
/// assert_eq!(my_str, "hi\n");
/// ```
/// ---
/// Contents of ./src/file.rb:
/// ```
/// puts "hi"
///
/// ```
/// # Note
/// `puts` adds a trailing '\n', use `print` if you don't want that
#[proc_macro]
pub fn ruby_file_str(input: TokenStream) -> TokenStream {
    let input: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    let file = input.get(0).expect(USE_MSG_STR_FILE);

    let file = match file {
        TokenTree::Literal(literal) => parse_str(&literal.to_string()),
        _ => panic!("{}", USE_MSG_STR_FILE),
    };

    let mut result = String::from("r#\"");
    result.push_str(&execute_file(&file));
    result.push_str("\"#");

    result.parse::<TokenStream>().expect(
        "Couldn't parse ruby output, perhaps check for presence of comments in your ruby code",
    )
}

/// Execute Ruby from a file and return a different type
///
/// # Arguments
/// * `type` - The return type of the ruby code
/// * `input` - A `&str` literal containing the file path
///
/// # Example
/// Contents of ./src/main.rs
/// ```
/// use macro_ruby::ruby_file_to;
///
/// let my_int = ruby_file_to!("src/file.rb");
/// assert_eq!(my_str, 1000);
/// ```
/// ---
/// Contents of ./src/file.rb:
/// ```
/// puts 500+500
///
/// ```
///
/// # List of supported types
/// `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`
/// `u64`, `u128`, `usize`, `f32`, `f64`, `char`, `bool`
///
/// # Note
/// If your type isn't supported you may use `ruby_file_ast!`;
#[proc_macro]
pub fn ruby_file_to(input: TokenStream) -> TokenStream {
    let input: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    let return_type = input.get(0).expect(USE_MSG_TYPE_FILE);
    let file = input.get(1).expect(USE_MSG_TYPE_FILE);

    let return_type = match return_type {
        TokenTree::Ident(t) => t.to_string(),
        _ => panic!(
            "Should be type but found '{}': {}",
            return_type, USE_MSG_TYPE_FILE
        ),
    };

    let file = match file {
        TokenTree::Literal(literal) => parse_str(&literal.to_string()),
        TokenTree::Punct(_) => match input.get(2).expect(USE_MSG_TYPE_FILE) {
            TokenTree::Literal(literal) => parse_str(&literal.to_string()),
            _ => panic!(
                "Should be string literal but found '{}':\n{}",
                file, USE_MSG_TYPE_FILE
            ),
        },
        _ => panic!(
            "Should be string literal but found '{}':\n{}",
            file, USE_MSG_TYPE_FILE
        ),
    };

    macro_rules! eval_type {
        ( $($type:ty),* ) => {
            match return_type.as_ref() {
                $(stringify!($type) => {

                    let result = execute_file(&file)
                        .trim_end()
                        .parse::<$type>();

                    let result = match result {
                        Ok(v) => v,
                        Err(e) => {
                                panic!("Couldn't convert to type: {}{}",
                                    stringify!($type),
                                    match e.kind() {
                                        &std::num::IntErrorKind::Empty => "\nPerhaps you forgot to print the result?",
                                        _ => ""
                                    }
                                );
                        }
                    };

                    let mut result = result.to_string();
                    result.push_str(concat!(" as ", stringify!($type)));

                    result.parse::<TokenStream>().expect("Couldn't parse token stream")

                }),*

                "f32" => {
                    let result = execute_file(&file)
                        .trim_end()
                        .parse::<f32>();

                    let result = match result {
                        Ok(v) => v,
                        Err(e) => {
                            panic!(
                                "Couldn't convert to type: f32{}",
                                if e == "".parse::<f32>().unwrap_err() { // weird hack
                                    "\nPerhaps you forgot to print the result?"
                                } else {
                                    ""
                                }
                            )
                        }
                    };

                    let mut result = result.to_string();
                    result.push_str(" as f32");

                    result.parse::<TokenStream>().expect("Couldn't parse token stream")
                },

                "f64" => {
                    let result = execute_file(&file)
                        .trim_end()
                        .parse::<f64>();

                    let result = match result {
                        Ok(v) => v,
                        Err(e) => {
                            panic!(
                                "Couldn't convert to type: f64{}",
                                if e == "".parse::<f64>().unwrap_err() { // weird hack
                                    "\nPerhaps you forgot to print the result?"
                                } else {
                                    ""
                                }
                            )
                        }
                    };

                    let mut result = result.to_string();
                    result.push_str(" as f64");

                    result.parse::<TokenStream>().expect("Couldn't parse token stream")
                },

                "bool" => {

                    let result = execute_file(&file)
                        .trim_end()
                        .parse::<bool>()
                        .expect("Couldn't convert to type: bool.\nPerhaps you forgot to print the result?");

                    let result = result.to_string();

                    result.parse::<TokenStream>().expect("Couldn't parse token stream")
                },

                "char" => {
                    let result = execute_file(&file)
                        .trim_end()
                        .parse::<char>();

                    let result = match result {
                        Ok(v) => v,
                        Err(e) => {
                            panic!(
                                "Couldn't convert to type: char{}",
                                if e == "".parse::<char>().unwrap_err() { // weird hack
                                    "\nPerhaps you forgot to print the result?"
                                } else {
                                    ""
                                }
                            )
                        }
                    };

                    let mut result = result.to_string();
                    result.push_str(" as char");

                    result.parse::<TokenStream>().expect("Couldn't parse token stream")

                }
                _ => panic!("Unsupported type")
            }
        }
    }

    eval_type!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize)
}

/// Execute Ruby from a file and generate Rust code
///
/// # Arguments
/// * `input` - A `&str` literal containing ruby code 
///
/// # Example
/// Contents of ./src/main.rs
/// ```
/// use macro_ruby::ruby_file_ast;
///
/// ruby_file_ast!("src/file.rb");
///
/// assert_eq!(var0, 0);
/// assert_eq!(var1, 1);
/// assert_eq!(var2, 2);
/// ```
/// ---
/// Contents of ./src/file.rb:
/// ```
/// 3.times do |x|
///     puts "let var#{x} = #{x};"
/// end
/// ```
#[proc_macro]
pub fn ruby_file_ast(input: TokenStream) -> TokenStream {
    let input: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    let file = input.get(0).expect(USE_MSG_AST_FILE);

    let file = match file {
        TokenTree::Literal(literal) => parse_str(&literal.to_string()),
        _ => panic!("{}", USE_MSG_AST_FILE),
    };

    execute_file(&file)
        .parse::<TokenStream>()
        .expect("Couldn't parse ruby output")
}
