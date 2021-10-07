#[cfg(test)]
mod tests {
    use macro_ruby::*;
    #[test]
    fn str_test() {
        assert_eq!(ruby_code_str!("puts 'hi'"), "hi\n");
        assert_eq!(ruby_code_str!(r"puts 'hi'"), "hi\n");
        assert_eq!(ruby_code_str!(r#"puts 'hi'"#), "hi\n");
        assert_eq!(ruby_code_str!(r#"
                puts 'hi'
                "#), "hi\n");
    }

    #[test]
    fn to_test() {
        assert_eq!(ruby_code_to!(i32 "print 500+500"), 1000);
        assert_eq!(ruby_code_to!(i32, "print 500+500"), 1000);
        assert_eq!(ruby_code_to!(i32: "print 500+500"), 1000);
    }

    #[test]
    fn ast_test() {
        ruby_code_ast!(r#"

            3.times do |x|
                puts "let var#{x} = #{x};"
            end

            "#);

        assert_eq!(var0, 0);
        assert_eq!(var1, 1);
        assert_eq!(var2, 2);
    }
}
