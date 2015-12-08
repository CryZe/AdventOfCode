use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::char;

use ParseState::*;

enum ParseState {
    NoString,
    Normal,
    Escaped,
    EscapedHexadecimal,
    EscapedHexadecimal1(char),
}

type IntermediateScanResult = Result<Option<char>, &'static str>;
type ParseResult = Result<String, &'static str>;

fn read_file(path: &Path) -> Vec<String> {
    let mut input = String::new();
    let mut file = File::open(path).expect("File could not be found");
    file.read_to_string(&mut input).expect("File could not be read");
    input.lines().into_iter().map(|s| s.to_owned()).collect()
}

fn scan_no_string(state: &mut ParseState, c: char) -> IntermediateScanResult {
    match c {
        '"' => {
            *state = Normal;
            Ok(None)
        }
        _ => Err(r#"Unknown symbol outside of String. Expected '"'."#),
    }
}

fn scan_normal(state: &mut ParseState, c: char) -> IntermediateScanResult {
    match c {
        '"' => {
            *state = NoString;
            Ok(None)
        }
        '\\' => {
            *state = Escaped;
            Ok(None)
        }
        _ => Ok(Some(c)),
    }
}

fn scan_escaped(state: &mut ParseState, c: char) -> IntermediateScanResult {
    match c {
        '"' | '\\' => {
            *state = Normal;
            Ok(Some(c))
        }
        'x' => {
            *state = EscapedHexadecimal;
            Ok(None)
        }
        _ => Err(r#"Unknown Escape Sequence. Expected '"', '\' or 'x'."#),
    }
}

fn scan_escaped_hex(state: &mut ParseState, c: char) -> IntermediateScanResult {
    match c {
        '0'...'9' | 'A'...'F' | 'a'...'f' => {
            *state = EscapedHexadecimal1(c);
            Ok(None)
        }
        _ => Err(r#"Unknown Hexadecimal Escape Sequence. Expected a digit."#),
    }
}

fn scan_escaped_hex1(state: &mut ParseState, c: char, digit: char) -> IntermediateScanResult {
    match c {
        '0'...'9' | 'A'...'F' | 'a'...'f' => {
            *state = Normal;
            Ok(digit.to_digit(16)
                    .and_then(|d1| c.to_digit(16).and_then(|d2| char::from_u32(d1 * 16 + d2))))
        }
        _ => Err(r#"Unknown Hexadecimal Escape Sequence. Expected a digit."#),
    }
}

fn parse_string(input: &str) -> ParseResult {
    let mut error = None;
    let parsed = input.chars()
                      .into_iter()
                      .scan(NoString, |state, c| {
                          let result = match state {
                              &mut NoString => scan_no_string(state, c),
                              &mut Normal => scan_normal(state, c),
                              &mut Escaped => scan_escaped(state, c),
                              &mut EscapedHexadecimal => scan_escaped_hex(state, c),
                              &mut EscapedHexadecimal1(digit) => scan_escaped_hex1(state, c, digit),
                          };
                          match result {
                              Ok(inner) => Some(inner),
                              Err(err) => {
                                  error = Some(err);
                                  None
                              }
                          }
                      })
                      .filter_map(|c| c)
                      .collect();

    match error {
        Some(err) => Err(err),
        None => Ok(parsed),
    }
}

fn parse_strings<'a, I>(lines: I) -> Vec<ParseResult>
    where I: IntoIterator<Item = &'a String>
{
    lines.into_iter().map(|l| parse_string(l)).collect()
}

fn encode_string(input: &str) -> String {
    ['"']
        .iter()
        .cloned()
        .chain(input.chars().flat_map(|c| {
            match c {
                '\\' | '"' => vec!['\\', c],
                _ => vec![c],
            }
        }))
        .chain(['"'].iter().cloned())
        .collect()
}

fn encode_strings<'a, I>(lines: I) -> Vec<String>
    where I: IntoIterator<Item = &'a String>
{
    lines.into_iter().map(|l| encode_string(l)).collect()
}

fn main() {
    let lines = read_file(Path::new("input.txt"));
    let parsed = parse_strings(&lines);

    let input_chars = lines.iter().fold(0, |a, l| a + l.chars().count());
    let parsed_chars = parsed.into_iter()
                             .fold(0, |a, r| a + r.ok().map(|l| l.chars().count()).unwrap_or(0));

    println!("Input Chars: {}", input_chars);
    println!("Parsed Chars: {}", parsed_chars);
    println!("Overhead: {}", input_chars - parsed_chars);

    let encoded = encode_strings(&lines);
    let encoded_chars = encoded.iter().fold(0, |a, l| a + l.chars().count());

    println!("Encoded Chars: {}", encoded_chars);
    println!("Overhead: {}", encoded_chars - input_chars);
}

#[test]
fn test_parsing() {
    assert_eq!(parse_string(r#""""#), Ok(r#""#.to_owned()));
    assert_eq!(parse_string(r#""abc""#), Ok(r#"abc"#.to_owned()));
    assert_eq!(parse_string(r#""aaa\"aaa""#), Ok(r#"aaa"aaa"#.to_owned()));
    assert_eq!(parse_string(r#""aaa\\aaa""#), Ok(r#"aaa\aaa"#.to_owned()));
    assert_eq!(parse_string(r#""\x27""#), Ok(r#"'"#.to_owned()));

    assert_eq!(parse_string(r#"a"#).ok(), None);
    assert_eq!(parse_string(r#""abc"a"#).ok(), None);
    assert_eq!(parse_string(r#""aaa\aaa""#).ok(), None);
    assert_eq!(parse_string(r#""\xg7""#).ok(), None);
}

#[test]
fn test_encoding() {
    assert_eq!(encode_string(r#""""#), r#""\"\"""#);
    assert_eq!(encode_string(r#""abc""#), r#""\"abc\"""#);
    assert_eq!(encode_string(r#""aaa\"aaa""#), r#""\"aaa\\\"aaa\"""#);
    assert_eq!(encode_string(r#""aaa\\aaa""#), r#""\"aaa\\\\aaa\"""#);
    assert_eq!(encode_string(r#""\x27""#), r#""\"\\x27\"""#);
}
