use crate::argument::parser::{ArgumentParser, IntArgument, StringArgument};
use crate::parsers::escaped_string::parse_string;
use crate::parsers::tokenize::{tokenize, Token};
use crate::{Command, Dispatcher, Error, InvalidCommandReason};

#[test]
fn command() {
    let dispatcher = Dispatcher::builder()
        .prefix("/")
        .context(|| ())
        .child(Command::literal("hello").exec(|ctx| {
            println!("{ctx:?}");
            Ok(())
        }))
        .build()
        .unwrap();

    dispatcher.run_command("/hello").unwrap();
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
        Token::Named("hello".to_string(), "good day".to_string()),
    ];
    assert_eq!(tokenize(sample), Ok(("   ", parsed_sample)));
}

#[test]
fn string_argument() {
    assert_eq!(
        StringArgument.parse("hello".to_string()),
        Ok(String::from("hello"))
    )
}

#[test]
fn int_argument() {
    assert_eq!(IntArgument.parse("123".to_string()), Ok(123));
    assert_eq!(
        IntArgument.parse("abc".to_string()),
        Err(Error::InvalidCommand(InvalidCommandReason::InvalidArgument))
    )
}
