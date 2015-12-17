use std::io::prelude::*;
use std::fs::File;
use std::env::args;
use std::path::{PathBuf, Path};

fn count_container_combinations_inner(eggnog: usize,
                                      containers: &[usize],
                                      container_count: usize)
                                      -> usize {
    if eggnog == 0 {
        1
    } else if let Some(size) = containers.iter().cloned().take(container_count).nth(0) {
        let unused_combinations = count_container_combinations_inner(eggnog,
                                                                     &containers[1..],
                                                                     container_count);
        if size <= eggnog {
            count_container_combinations_inner(eggnog - size,
                                               &containers[1..],
                                               container_count - 1) +
            unused_combinations
        } else {
            unused_combinations
        }
    } else {
        0
    }
}

fn count_container_combinations(eggnog: usize, containers: &[usize]) -> usize {
    count_container_combinations_inner(eggnog, containers, containers.len())
}

fn count_container_combinations_of_minimum(eggnog: usize, containers: &[usize]) -> Option<usize> {
    (0..containers.len())
        .map(|l| count_container_combinations_inner(eggnog, containers, l))
        .find(|c| *c > 0)
}

fn read_file(path: &Path) -> Vec<usize> {
    let mut input = String::new();
    let mut file = File::open(path).expect("File could not be found.");
    file.read_to_string(&mut input).expect("File could not be read.");
    input.lines()
         .into_iter()
         .map(|s| s.parse().expect("Container Size could not be parsed."))
         .collect()
}

fn main() {
    let mut path = PathBuf::from(args().nth(0).unwrap());
    path.pop();
    path.push("input.txt");
    let containers = read_file(&path);

    let combinations = count_container_combinations(150, &containers);
    println!("There are {} combinations possible in total.", combinations);

    let min_combinations = count_container_combinations_of_minimum(150, &containers);
    println!("There are {} combinations possible of the minimum amount of containers.",
             min_combinations.map(|c| c.to_string()).unwrap_or_else(|| "no".to_owned()));
}

#[test]
fn test_count_container_combinations() {
    let containers = [20, 15, 10, 5, 5];
    let combinations = count_container_combinations(25, &containers);
    assert_eq!(combinations, 4);
}

#[test]
fn test_count_container_combinations_of_minimum() {
    let containers = [20, 15, 10, 5, 5];
    let combinations = count_container_combinations_of_minimum(25, &containers);
    assert_eq!(combinations, Some(3));
}
