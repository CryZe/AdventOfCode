extern crate itertools;

use itertools::Itertools;
use std::str::FromStr;

fn get_next_number<'a, I>(number: I) -> Vec<u8>
    where I: IntoIterator<Item = &'a u8>
{
    number.into_iter()
          .group_by(|&n| *n)
          .flat_map(|(key, group)| {
              vec![group.len() as u8, key]
          })
          .collect()
}

fn get_number_after_n_iterations<'a, I>(number: I, n: usize) -> Vec<u8>
    where I: IntoIterator<Item = &'a u8>
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
