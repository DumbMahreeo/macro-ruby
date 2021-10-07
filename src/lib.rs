mod exec;
mod parser;

use proc_macro::{TokenStream, TokenTree};
use crate::{
    exec::execute,
    parser::parse_str
};

const USE_MSG_STR: &str =
    "Wrong usage:\nUse ruby_code_str!(\"literal\")\nExample: ruby_code_str!(\"print 'something'\")";

const USE_MSG_TYPE: &str =
    "Wrong usage:\nUse ruby_code_to!(type \"literal\")\nExample: ruby_code_to!(i32 \"print 2+2\")";

const USE_MSG_AST: &str =
    "Wrong usage:\nUse ruby_code_ast!(type \"literal\")\nExample: ruby_code_ast!(\"puts 'let a = 1;'\")";

#[proc_macro]
pub fn ruby_code_str(input: TokenStream) -> TokenStream {
    let input: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    let code = input.get(0).expect(USE_MSG_STR);

    let code = match code {
        TokenTree::Literal(literal) => parse_str(literal.to_string().as_ref()),
        _ => panic!("{}", USE_MSG_STR),
    };

    let mut result = String::from("r#\"");
    result.push_str(execute(&code).as_ref());
    result.push_str("\"#");

    result.parse::<TokenStream>().expect(
        "Couldn't parse ruby output, perhaps check for presence of comments in your ruby code",
    )
}

#[proc_macro]
pub fn ruby_code_to(input: TokenStream) -> TokenStream {
    let input: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    let return_type = input.get(0).expect(USE_MSG_TYPE);
    let code = input.get(1).expect(USE_MSG_TYPE);

    let return_type = match return_type {
        TokenTree::Ident(t) => t.to_string(),
        _ => panic!(
            "Should be type but found '{}': {}",
            return_type, USE_MSG_TYPE
        ),
    };

    let code = match code {
        TokenTree::Literal(literal) => parse_str(literal.to_string().as_ref()),
        TokenTree::Punct(_) => match input.get(2).expect(USE_MSG_TYPE) {
            TokenTree::Literal(literal) => parse_str(literal.to_string().as_ref()),
            _ => panic!(
                "Should be string literal but found '{}':\n{}",
                code, USE_MSG_TYPE
            ),
        },
        _ => panic!(
            "Should be string literal but found '{}':\n{}",
            code, USE_MSG_TYPE
        ),
    };

    macro_rules! eval_type {
        ( $($type:ty),* ) => {
            match return_type.as_ref() {
                $(stringify!($type) => {

                    let result = execute(&code)
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
                    let result = execute(&code)
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
                    let result = execute(&code)
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
                    let result = execute(&code)
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

#[proc_macro]
pub fn ruby_code_ast(input: TokenStream) -> TokenStream {
    let input: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    let code = input.get(0).expect(USE_MSG_AST);

    let code = match code {
        TokenTree::Literal(literal) => parse_str(literal.to_string().as_ref()),
        _ => panic!("{}", USE_MSG_AST),
    };

    execute(&code)
        .parse::<TokenStream>()
        .expect("Couldn't parse ruby output")
}
