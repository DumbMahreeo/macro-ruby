# macro-ruby
A crate to generate rust code trough Ruby at compile time.
(Tested with mruby 3.0.0)

# Requirements
Have `mruby` in your `PATH`.

# How to use

```rust
    use macro_ruby::ruby_code_str;

    // ruby_code_str! generates a &str based on what has been printed from Ruby.
    assert_eq!(
        ruby_code_str!("puts 'hi'"),
        "hi\n"
    );


    assert_eq!(
        ruby_code_str!("print 'hi'"),
        "hi"
    );
```

```rust
    use macro_ruby::ruby_code_to;

    // ruby_code_to! generates a value which type is based off of input
    // and the content on what has been printed from Ruby.
    assert_eq!(
        ruby_code_to!(i32 "print 500+500"),
        1000
    );


    assert_eq!(
        ruby_code_to!(u8 "print 500+500"), // Will panic because u8 overflows
        1000
    );
```

```rust
    use macro_ruby::ruby_code_ast;

    // ruby_code_ast! generates real rust code based on what has been printed
    ruby_code_ast!(r#"
        puts "let a = 1;"
    "#)

    assert_eq!(a, 1);
```

If you want to execute ruby code from external files you can use the `file` variant of our macros

| Str version | File version |
| --- | --- |
| `ruby_code_str!` | `ruby_file_str!` |
| `ruby_code_to!` | `ruby_file_to!` |
| `ruby_code_ast!` | `ruby_file_ast!` |
