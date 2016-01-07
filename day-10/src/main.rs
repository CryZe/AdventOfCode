extern crate itertools;

use itertools::Itertools;
use std::str::FromStr;

fn get_next_number_itertools(number: &Vec<u8>) -> Vec<u8>
{
    let mut result = Vec::with_capacity(2 * number.len());
    for (key, group) in number.into_iter().group_by(|&n| *n) {
        result.push(group.len() as u8);
        result.push(key);
    }
    result
}

fn get_next_number_fast(number: Vec<u8>) -> Vec<u8>
{
    let mut result = Vec::with_capacity(2 * number.len());
    let mut iter = number.into_iter();
    let mut maybe_digit = iter.next();
    while let Some(digit) = maybe_digit {
        let mut repeat = 1;
        maybe_digit = None;
        while let Some(other_digit) = iter.next() {
            if other_digit != digit {
                maybe_digit = Some(other_digit);
                break;
            }
            repeat += 1;
        }
        result.push(repeat);
        result.push(digit);
    }
    return result;
}

fn get_number_after_n_iterations<'a, I>(number: I, n: usize) -> Vec<u8>
    where I: IntoIterator<Item = &'a u8>
{
    (0..n).fold(number.into_iter().cloned().collect(),
                |n, _| get_next_number_fast(n))
}

fn main() {
    let n = match std::env::args().nth(1).and_then(|arg| usize::from_str(&arg).ok()) {
        Some(arg) => arg,
        None => {
            println!("Call with: day-10 <iterations>");
            return;
        }
    };

    let input = [3, 1, 1, 3, 3, 2, 2, 1, 1, 3];

    let n_iterations = get_number_after_n_iterations(&input, n);
    println!("Length of Number after {} iterations: {}",
             n,
             n_iterations.len());
}

#[test]
fn test_next_number_itertools() {
    assert_eq!(get_next_number_itertools(&vec![1]), vec![1, 1]);
    assert_eq!(get_next_number_itertools(&vec![1, 1]), vec![2, 1]);
    assert_eq!(get_next_number_itertools(&vec![2, 1]), vec![1, 2, 1, 1]);
    assert_eq!(get_next_number_itertools(&vec![1, 2, 1, 1]), vec![1, 1, 1, 2, 2, 1]);
    assert_eq!(get_next_number_itertools(&vec![1, 1, 1, 2, 2, 1]), vec![3, 1, 2, 2, 1, 1]);
}

#[test]
fn test_next_number_fast() {
    assert_eq!(get_next_number_fast(vec![1]), vec![1, 1]);
    assert_eq!(get_next_number_fast(vec![1, 1]), vec![2, 1]);
    assert_eq!(get_next_number_fast(vec![2, 1]), vec![1, 2, 1, 1]);
    assert_eq!(get_next_number_fast(vec![1, 2, 1, 1]), vec![1, 1, 1, 2, 2, 1]);
    assert_eq!(get_next_number_fast(vec![1, 1, 1, 2, 2, 1]), vec![3, 1, 2, 2, 1, 1]);
}

#[test]
fn test_number_after_n_iterations() {
    assert_eq!(get_number_after_n_iterations(&vec![1], 0), vec![1]);
    assert_eq!(get_number_after_n_iterations(&vec![1], 1), vec![1, 1]);
    assert_eq!(get_number_after_n_iterations(&vec![1], 2), vec![2, 1]);
    assert_eq!(get_number_after_n_iterations(&vec![1], 3), vec![1, 2, 1, 1]);
    assert_eq!(get_number_after_n_iterations(&vec![1], 4), vec![1, 1, 1, 2, 2, 1]);
    assert_eq!(get_number_after_n_iterations(&vec![1], 5), vec![3, 1, 2, 2, 1, 1]);
}
