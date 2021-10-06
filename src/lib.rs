use proc_macro::{TokenStream, TokenTree};
use std::process::Command;

const USE_MSG: &str =
    "Wrong usage:\nUse eval_ruby_str!(\"literal\")\nExample: macro!(i32, \"2+2\")";
const USE_MSG_TYPE: &str =
    "Wrong usage:\nUse eval_ruby_as!(type \"literal\")\nExample: macro!(i32, \"2+2\")";

fn execute(code: &str) -> String {
    let output = Command::new("mruby")
        .arg("-e")
        .arg(code)
        .output()
        .expect("Couldn't get output from mruby");

    String::from_utf8(output.stdout).expect("Non utf-8 output from mruby")
}

fn parse_str(string: &str) -> String {
    macro_rules! check_ends {
        ($char:literal) => {
            check_ends!($char, $char)
        };

        ($start:literal, $end:literal) => {
            string.starts_with($start) && string.ends_with($end)
        };
    }

    if check_ends!('"') {
        string[1..string.len() - 1].to_string()
    } else if check_ends!("r\"", "\"") {
        string[2..string.len() - 1].to_string()
    } else if check_ends!("r#\"", "\"#") {
        string[3..string.len() - 2].to_string()
    } else {
        String::new()
    }
}

#[proc_macro]
pub fn eval_ruby_str(input: TokenStream) -> TokenStream {
    let input: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    let code = input.get(0).expect(USE_MSG);

    let code = match code {
        TokenTree::Literal(literal) => parse_str(literal.to_string().as_ref()),
        _ => panic!("{}", USE_MSG),
    };

    let mut result = String::from("r#\"");
    result.push_str(execute(&code).as_ref());
    result.push_str("\"#");

    result
        .parse::<TokenStream>()
        .expect("Couldn't parse ruby output (perhaps check if you got comments in your ruby code)")
}

#[proc_macro]
pub fn eval_ruby_as(input: TokenStream) -> TokenStream {
    let input: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    let return_type = input.get(0).expect(USE_MSG_TYPE);
    let code = input.get(1).expect(USE_MSG_TYPE);

    let return_type = match return_type {
        TokenTree::Ident(t) => t.to_string(),
        _ => panic!("{}", USE_MSG_TYPE),
    };

    let code = match code {
        TokenTree::Literal(literal) => parse_str(literal.to_string().as_ref()),
        _ => panic!("{}", USE_MSG_TYPE),
    };

    macro_rules! eval_type {
        ( $($type:ty),* ) => {
            match return_type.as_ref() {
                $(stringify!($type) => {

                    let result: $type = execute(&code).parse().expect(concat!("Couldn't convert to type: ", stringify!($type)));
                    let mut result = result.to_string();
                    result.push_str(concat!(" as ", stringify!($type)));

                    result.parse::<TokenStream>().expect("Couldn't parse token stream")

                }),*
                _ => panic!("Unsupported type")
            }
        }
    }

    eval_type!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, usize, char)
}

#[proc_macro]
pub fn eval_ruby_ast(input: TokenStream) -> TokenStream {
    let input: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    let code = input.get(0).expect(USE_MSG);

    let code = match code {
        TokenTree::Literal(literal) => parse_str(literal.to_string().as_ref()),
        _ => panic!("{}", USE_MSG),
    };

    execute(&code)
        .parse::<TokenStream>()
        .expect("Couldn't parse ruby output")
}
