use crate::{Command, CommandDispatcher};
use crate::parsers::escaped_string::parse_string;
use crate::parsers::tokenize::{Token, tokenize};

#[test]
fn command() {
    let dispatcher = CommandDispatcher::builder()
        .command("test", Command::builder().build())
        .build();

    dispatcher.run_command("   test\n onion").unwrap();
}

#[test]
fn double_command() {
    let dispatcher = CommandDispatcher::builder()
        .command("test", Command::builder().build())
        .build();

    dispatcher.run_command("test 1; test 2").unwrap();
}

#[test]
fn command_with_prefix() {
    let dispatcher = CommandDispatcher::builder()
        .prefix("/")
        .command("test", Command::builder().build())
        .build();
    dispatcher.run_command("/test").unwrap();
}

#[test]
fn string_parse() {
    let sample = r#""hello\nworld \t tab""#;
    let result = parse_string::<nom::error::Error<&str>>(sample);
    assert_eq!(result, Ok(("", String::from("hello\nworld \t tab"))))
}

#[test]
fn string_parse_remainder() {
    let sample = r#""hel lo" world"#;
    let result = parse_string::<nom::error::Error<&str>>(sample);
    assert_eq!(result, Ok((" world", String::from("hel lo"))))
}

#[test]
fn unclosed_quoted_string() {
    let sample = r#""hello"#;
    let result = parse_string::<nom::error::Error<&str>>(sample);
    assert!(result.is_err())
}

#[test]
fn test_tokenize() {
    let sample = r#"test "epic parameter"; bye=2 ; hello="good day"   "#;
    let parsed_sample = vec![
        Token::Simple("test".to_string()),
        Token::Simple("epic parameter".to_string()),
        Token::End,
        Token::Named("bye".to_string(), "2".to_string()),
        Token::End,
        Token::Named("hello".to_string(), "good day".to_string())
    ];
    assert_eq!(tokenize(sample), Ok(("   ", parsed_sample)));
}
