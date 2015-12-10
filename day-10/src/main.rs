extern crate itertools;

use itertools::{Unfold, Itertools};
use std::str::FromStr;
use std::iter::Rev;
use std::vec::IntoIter;

fn number_to_digits(number: usize) -> Rev<IntoIter<usize>> {
    Unfold::new(number, |n| {
        if *n > 0 {
            let res = *n % 10;
            *n /= 10;
            Some(res)
        } else {
            None
        }
    })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
}

fn get_next_number<'a, I>(number: I) -> Vec<usize>
    where I: IntoIterator<Item = &'a usize>
{
    number.into_iter()
          .group_by(|&n| *n)
          .flat_map(|(key, group)| {
              number_to_digits(group.len()).into_iter().chain(vec![key].into_iter())
          })
          .collect()
}

fn get_number_after_n_iterations<'a, I>(number: I, n: usize) -> Vec<usize>
    where I: IntoIterator<Item = &'a usize>
{
    (0..n).fold(number.into_iter().cloned().collect(),
                |n, _| get_next_number(&n))
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
fn test_next_number() {
    assert_eq!(get_next_number(&[1]), [1, 1]);
    assert_eq!(get_next_number(&[1, 1]), [2, 1]);
    assert_eq!(get_next_number(&[2, 1]), [1, 2, 1, 1]);
    assert_eq!(get_next_number(&[1, 2, 1, 1]), [1, 1, 1, 2, 2, 1]);
    assert_eq!(get_next_number(&[1, 1, 1, 2, 2, 1]), [3, 1, 2, 2, 1, 1]);

    assert_eq!(get_next_number(&[3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]),
               [1, 2, 3]);
}

#[test]
fn test_number_after_n_iterations() {
    assert_eq!(get_number_after_n_iterations(&[1], 0), [1]);
    assert_eq!(get_number_after_n_iterations(&[1], 1), [1, 1]);
    assert_eq!(get_number_after_n_iterations(&[1], 2), [2, 1]);
    assert_eq!(get_number_after_n_iterations(&[1], 3), [1, 2, 1, 1]);
    assert_eq!(get_number_after_n_iterations(&[1], 4), [1, 1, 1, 2, 2, 1]);
    assert_eq!(get_number_after_n_iterations(&[1], 5), [3, 1, 2, 2, 1, 1]);
}
