extern crate md5;

use md5::*;

pub fn print_hash(digest: &Digest) {
    print!("Hash: ");
    for b in digest {
        print!("{:X}", b);
    }
    println!("");
}

fn hash_has_n_zeroes(input: &str, number: usize, zero_count: usize) -> bool {
    let mut full_input = input.to_owned();
    full_input.push_str(&number.to_string());

    let input_bytes = full_input.as_bytes();

    let digest = compute(input_bytes);

    digest.iter()
          .take_while(|&b| *b == 0)
          .chain(digest.iter().skip_while(|&b| *b == 0).take(1))
          .map(|&b| match b { 0 => 2, 0...15 => 1, _ => 0 })
          .fold(0, |a, b| a + b)
          >= zero_count
}

fn find_first_number(input: &str, zero_count: usize) -> usize {
    (1..).skip_while(|&n| !hash_has_n_zeroes(input, n, zero_count)).next().unwrap()
}

fn main() {
    let input = "ckczppom";
    println!("First Number with 5 Zeroes: {}", find_first_number(input, 5));
    println!("First Number with 6 Zeroes: {}", find_first_number(input, 6));
    println!("First Number with 7 Zeroes: {}", find_first_number(input, 7));
}

#[test]
fn test_md5_correctness()
{
    assert_eq!(compute("abcdef609043".as_bytes()), [0x00, 0x00, 0x01, 0xdb, 0xbf, 0xa3, 0xa5, 0xc8, 0x3a, 0x2d, 0x50, 0x64, 0x29, 0xc7, 0xb0, 0x0e])
}

#[test]
fn test()
{
    assert_eq!(find_first_number("abcdef", 5), 609043);
    assert_eq!(find_first_number("pqrstuv", 5), 1048970);
}
