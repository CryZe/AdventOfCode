extern crate serde_json;

use serde_json::Value;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use serde_json::Value::*;

fn add_up_numbers_part1(json: &Value) -> isize {
    match *json {
        I64(num) => num as isize,
        U64(num) => num as isize,
        F64(num) => num as isize,
        Array(ref vec) => vec.into_iter().fold(0, |a, i| a + add_up_numbers_part1(i)),
        Object(ref map) => map.values().fold(0, |a, i| a + add_up_numbers_part1(i)),
        _ => 0,
    }
}

fn add_up_numbers_part2(json: &Value) -> isize {
    match *json {
        I64(num) => num as isize,
        U64(num) => num as isize,
        F64(num) => num as isize,
        Array(ref vec) => vec.into_iter().fold(0, |a, i| a + add_up_numbers_part2(i)),
        Object(ref map) => {
            if map.values().any(|v| {
                match *v {
                    String(ref s) => s == "red",
                    _ => false,
                }
            }) {
                0
            } else {
                map.values().fold(0, |a, i| a + add_up_numbers_part2(i))
            }
        }
        _ => 0,
    }
}

fn parse_and_add_up_numbers_part1(input: &str) -> isize {
    serde_json::from_str::<Value>(input).map(|json| add_up_numbers_part1(&json)).unwrap_or(0)
}

fn parse_and_add_up_numbers_part2(input: &str) -> isize {
    serde_json::from_str::<Value>(input).map(|json| add_up_numbers_part2(&json)).unwrap_or(0)
}

fn read_file(path: &Path) -> String {
    let mut input = String::new();
    let mut file = File::open(path).expect("File could not be found");
    file.read_to_string(&mut input).expect("File could not be read");
    input
}

fn main() {
    let input = read_file(Path::new("input.txt"));

    let number_part1 = parse_and_add_up_numbers_part1(&input);
    println!("Part 1: {}", number_part1);

    let number_part2 = parse_and_add_up_numbers_part2(&input);
    println!("Part 2: {}", number_part2);
}

#[test]
fn test_part1() {
    assert_eq!(parse_and_add_up_numbers_part1(r#"[1,2,3]"#), 6);
    assert_eq!(parse_and_add_up_numbers_part1(r#"{"a":2,"b":4}"#), 6);
    assert_eq!(parse_and_add_up_numbers_part1(r#"[[[3]]]"#), 3);
    assert_eq!(parse_and_add_up_numbers_part1(r#"{"a":{"b":4},"c":-1}"#), 3);
    assert_eq!(parse_and_add_up_numbers_part1(r#"{"a":[-1,1]}"#), 0);
    assert_eq!(parse_and_add_up_numbers_part1(r#"[-1,{"a":1}]"#), 0);
    assert_eq!(parse_and_add_up_numbers_part1(r#"[]"#), 0);
    assert_eq!(parse_and_add_up_numbers_part1(r#"{}"#), 0);
}

#[test]
fn test_part2() {
    assert_eq!(parse_and_add_up_numbers_part2(r#"[1,2,3]"#), 6);
    assert_eq!(parse_and_add_up_numbers_part2(r#"[1,{"c":"red","b":2},3]"#), 4);
    assert_eq!(parse_and_add_up_numbers_part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
    assert_eq!(parse_and_add_up_numbers_part2(r#"[1,"red",5]"#), 6);
}
