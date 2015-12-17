extern crate itertools;

use std::cmp::{min, max};
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use itertools::Itertools;

#[derive(PartialEq, Debug)]
struct Reindeer {
    name: String,
    speed: usize,
    stamina: usize,
    cooldown: usize,
}

impl Reindeer {
    fn new(name: &str, speed: usize, stamina: usize, cooldown: usize) -> Self {
        Reindeer {
            name: name.to_owned(),
            speed: speed,
            stamina: stamina,
            cooldown: cooldown,
        }
    }

    fn parse(line: &str) -> Self {
        let mut splits = line.split_whitespace();
        let name = splits.nth(0).unwrap();
        let speed = splits.nth(2).and_then(|s| s.parse().ok()).unwrap();
        let stamina = splits.nth(2).and_then(|s| s.parse().ok()).unwrap();
        let cooldown = splits.nth(6).and_then(|s| s.parse().ok()).unwrap();
        Self::new(name, speed, stamina, cooldown)
    }

    fn get_distance(&self, time: usize) -> usize {
        let cycle_time = self.stamina + self.cooldown;
        let full_cycles_count = time / cycle_time;
        let full_cycles_time = full_cycles_count * cycle_time;
        let full_cycles_running_time = full_cycles_count * self.stamina;
        let remaining_time = time - full_cycles_time;
        let remaining_running_time = min(remaining_time, self.stamina);
        let running_time = full_cycles_running_time + remaining_running_time;
        self.speed * running_time
    }
}

fn read_file(path: &Path) -> Vec<String> {
    let mut input = String::new();
    let mut file = File::open(path).expect("File could not be found");
    file.read_to_string(&mut input).expect("File could not be read");
    input.lines().into_iter().map(|s| s.to_owned()).collect()
}

fn parse_reindeers<'a, I>(lines: I) -> Vec<Reindeer>
    where I: IntoIterator<Item = &'a String>
{
    lines.into_iter().map(|l| Reindeer::parse(l)).collect()
}

fn main() {
    let input = read_file(&Path::new("input.txt"));
    let reindeers = parse_reindeers(&input);
    let time = 2503;

    let maximum_distance = reindeers.iter().map(|r| r.get_distance(time)).fold1(max);
    println!("Maximum Distance after {} seconds: {} km",
             time,
             maximum_distance.map(|d| d.to_string()).unwrap_or("-".to_owned()));

    let mut winners = (1..time + 1)
                          .flat_map(|t| {
                              reindeers.iter()
                                       .map(|r| (r.name.to_owned(), r.get_distance(t)))
                                       .sorted_by(|&(_, ad), &(_, bd)| Ord::cmp(&bd, &ad))
                                       .into_iter()
                                       .group_by(|&(_, d)| d)
                                       .nth(0)
                                       .unwrap()
                                       .1
                                       .into_iter()
                                       .map(|(r, _)| r)
                                       .collect::<Vec<_>>()
                          })
                          .collect::<Vec<_>>();
    winners.sort();
    let maximum_points = winners.into_iter()
                                .group_by(|r| r.to_owned())
                                .sorted_by(|&(_, ref g1), &(_, ref g2)| {
                                    Ord::cmp(&g2.len(), &g1.len())
                                })
                                .into_iter()
                                .nth(0)
                                .unwrap()
                                .1
                                .len();
    println!("Maximum Points: {}", maximum_points);
}

#[test]
fn test_parsing() {
    assert_eq!(Reindeer::parse("Vixen can fly 8 km/s for 8 seconds, but then must rest for 53 \
                                seconds."),
               Reindeer::new("Vixen", 8, 8, 53));
}

#[test]
fn test_reindeers() {
    let comet = Reindeer::new("Comet", 14, 10, 127);
    let dancer = Reindeer::new("Dancer", 16, 11, 162);
    assert_eq!(comet.get_distance(1), 14);
    assert_eq!(dancer.get_distance(1), 16);
    assert_eq!(comet.get_distance(10), 140);
    assert_eq!(dancer.get_distance(10), 160);
    assert_eq!(comet.get_distance(11), 140);
    assert_eq!(dancer.get_distance(11), 176);
    assert_eq!(comet.get_distance(12), 140);
    assert_eq!(dancer.get_distance(12), 176);

    assert_eq!(comet.get_distance(1000), 1120);
    assert_eq!(dancer.get_distance(1000), 1056);
}
