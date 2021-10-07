pub fn parse_str(string: &str) -> String {
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

