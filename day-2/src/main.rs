use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn parse(input: &str) -> Vec<Vec<usize>> {
    input.lines()
         .map(|l| {
             let mut dimensions = l.split('x')
                                   .map(|c| usize::from_str(c).expect("Could not parse number"))
                                   .collect::<Vec<_>>();
             dimensions.sort();
             dimensions
         })
         .collect()
}

fn calculate_wrapping_paper(input: &Vec<Vec<usize>>) -> usize {
    input.iter()
         .map(|dimensions|
             2 * (dimensions[0] * dimensions[1] +
                  dimensions[1] * dimensions[2] +
                  dimensions[2] * dimensions[0]) +
                  dimensions[0] * dimensions[1])
         .fold(0, |a, b| a + b)
}

fn calculate_ribbons(input: &Vec<Vec<usize>>) -> usize {
    input.iter()
         .map(|dimensions|
             2 * (dimensions[0] + dimensions[1]) +
             (dimensions[0] * dimensions[1] * dimensions[2]))
         .fold(0, |a, b| a + b)
}

fn main() {
    let mut input = String::new();
    let mut file = File::open("input.txt").expect("File could not be found");
    file.read_to_string(&mut input).expect("File could not be read");

    let parsed = parse(&input);

    let wrapping_paper = calculate_wrapping_paper(&parsed);
    let ribbons = calculate_ribbons(&parsed);

    println!("Amount of Wrapping Paper needed: {} square feet", wrapping_paper);
    println!("Amount of Ribbons needed: {} feet", ribbons);
}

#[test]
fn test_wrapping_paper() {
    assert_eq!(calculate_wrapping_paper(&parse("2x3x4")), 58);
    assert_eq!(calculate_wrapping_paper(&parse("1x1x10")), 43);
}

#[test]
fn test_ribbons() {
    assert_eq!(calculate_ribbons(&parse("2x3x4")), 34);
    assert_eq!(calculate_ribbons(&parse("1x1x10")), 14);
}
