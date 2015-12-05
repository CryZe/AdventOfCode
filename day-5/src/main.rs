extern crate tabwriter;

use tabwriter::TabWriter;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::iter::{Skip, Zip};
use std::str::Chars;

type PairIterator<'a, 'b> = Zip<Chars<'a>, Skip<Chars<'b>>>;

fn get_pairs(input: &str) -> PairIterator {
    input.chars().zip(input.chars().skip(1))
}

fn contains_three_vowels(input: &str) -> bool {
    input.chars().filter(|&c| match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }).count() >= 3
}

fn contains_letter_pairs(input: &str) -> bool {
    get_pairs(input).any(|p| match p {
        (a, b) if a == b => true,
        _ => false
    })
}

fn contains_naughty_strings(input: &str) -> bool {
    get_pairs(input).any(|p| match p {
        ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => true,
        _ => false
    })
}

fn contains_double_pair(input: &str) -> bool {
    get_pairs(input).enumerate().any(|p| get_pairs(input).enumerate().any(
        |p2| p.1 == p2.1 && isize::abs(p.0 as isize - p2.0 as isize) >= 2))
}

fn contains_letter_pair_with_space(input: &str) -> bool {
    input.chars().zip(input.chars().skip(2)).any(|p| match p {
        (a, b) if a == b => true,
        _ => false
    })
}

fn is_nice_rule1(input: &str) -> bool {
    contains_three_vowels(input) && contains_letter_pairs(input) &&
        !contains_naughty_strings(input)
}

fn is_nice_rule2(input: &str) -> bool {
    contains_double_pair(input) && contains_letter_pair_with_space(input)
}

fn count_how_many_strings_are_nice_rule1<'a, I>(input: I) -> usize
where I: IntoIterator<Item = &'a String> {
    input.into_iter().filter(|l| is_nice_rule1(l)).count()
}

fn count_how_many_strings_are_nice_rule2<'a, I>(input: I) -> usize
where I: IntoIterator<Item = &'a String> {
    input.into_iter().filter(|l| is_nice_rule2(l)).count()
}

fn read_file(path: &Path) -> Vec<String> {
    let mut input = String::new();
    let mut file = File::open(path).expect("File could not be found");
    file.read_to_string(&mut input).expect("File could not be read");
    input.lines().into_iter().map(|s| s.to_owned()).collect()
}

fn main() {
    let input = read_file(Path::new("input.txt"));

    let mut tw = TabWriter::new(Vec::new());

    let count_rule1 = count_how_many_strings_are_nice_rule1(&input);
    let count_rule2 = count_how_many_strings_are_nice_rule2(&input);

    write!(&mut tw, "
Nice Strings according to Rule 1:\t{:3}
Nice Strings according to Rule 2:\t{:3}", count_rule1, count_rule2).unwrap();

    tw.flush().unwrap();

    let written = String::from_utf8(tw.unwrap()).unwrap();

    println!("{}", &written);
}

#[test]
fn test_contains_three_vowels() {
    assert!(contains_three_vowels("aei"));
    assert!(contains_three_vowels("xazegov"));
    assert!(contains_three_vowels("aeiouaeiouaeiou"));
    assert!(!contains_three_vowels("abi"));
    assert!(!contains_three_vowels("xazbgov"));
    assert!(!contains_three_vowels("bdbdbdbdbd"));
}

#[test]
fn test_contains_letter_pairs() {
    assert!(contains_letter_pairs("xx"));
    assert!(contains_letter_pairs("abcdde"));
    assert!(contains_letter_pairs("aabbccdd"));
    assert!(!contains_letter_pairs("aeiouaeiouaeiou"));
    assert!(!contains_letter_pairs("xazegov"));
    assert!(!contains_letter_pairs("oxicszyiqifoyugx"));
}

#[test]
fn test_contains_naughty_strings() {
    assert!(!contains_naughty_strings("xx"));
    assert!(contains_naughty_strings("abcdde"));
    assert!(contains_naughty_strings("aabbccdd"));
    assert!(!contains_naughty_strings("aeiouaeiouaeiou"));
    assert!(contains_naughty_strings("xyazegov"));
}

#[test]
fn test_is_nice_rule1() {
    assert!(is_nice_rule1("ugknbfddgicrmopn"));
    assert!(is_nice_rule1("aaa"));
    assert!(!is_nice_rule1("jchzalrnumimnmhp"));
    assert!(!is_nice_rule1("haegwjzuvuyypxyu"));
    assert!(!is_nice_rule1("dvszwmarrgswjxmb"));
}

#[test]
fn test_contains_double_pair() {
    assert!(contains_double_pair("xyxy"));
    assert!(contains_double_pair("aabcdefgaa"));
    assert!(!contains_double_pair("aaa"));
}

#[test]
fn test_contains_letter_pair_with_space() {
    assert!(contains_letter_pair_with_space("xyx"));
    assert!(contains_letter_pair_with_space("abcdefeghi"));
    assert!(contains_letter_pair_with_space("aaa"));
    assert!(!contains_letter_pair_with_space("xyz"));
    assert!(!contains_letter_pair_with_space("abcdefjghi"));
    assert!(!contains_letter_pair_with_space("aab"));
}

#[test]
fn test_is_nice_rule2() {
    assert!(is_nice_rule2("qjhvhtzxzqqjkmpb"));
    assert!(is_nice_rule2("xxyxx"));
    assert!(!is_nice_rule2("uurcxstgmygtbstg"));
    assert!(!is_nice_rule2("ieodomkazucvgmuy"));
}
