use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::env::args;

struct Sue {
    properties: HashMap<String, usize>,
}

impl Sue {
    fn new(properties: HashMap<String, usize>) -> Self {
        Sue { properties: properties }
    }

    fn parse(line: &str) -> Self {
        let index = line.find(": ").unwrap();
        let collection = line.split_at(index + 2).1;
        let items = collection.split(", ");
        let properties = items.map(parse_property).collect();
        Self::new(properties)
    }

    fn matches_property_list_part1(&self, property_list: &HashMap<String, usize>) -> bool {
        self.properties.iter().all(|(property, value)| {
            property_list.get(property).map(|v| v == value).unwrap_or(true)
        })
    }

    fn matches_property_list_part2(&self, property_list: &HashMap<String, usize>) -> bool {
        self.properties.iter().all(|(property, sue_value)| {
            let cmp_func = |v| {
                if property == "cats" || property == "trees" {
                    sue_value > v
                } else if property == "pomeranians" || property == "goldfish" {
                    sue_value < v
                } else {
                    sue_value == v
                }
            };
            property_list.get(property).map(cmp_func).unwrap_or(true)
        })
    }
}

fn parse_property(property: &str) -> (String, usize) {
    let mut splits = property.split(": ");
    (splits.nth(0).unwrap().to_owned(),
     splits.nth(0).unwrap().parse().unwrap())
}

fn read_file(path: &Path) -> Vec<String> {
    let mut input = String::new();
    let mut file = File::open(path).expect("File could not be found.");
    file.read_to_string(&mut input).expect("File could not be read.");
    input.lines().into_iter().map(|s| s.to_owned()).collect()
}

fn parse_sues(lines: &[String]) -> Vec<Sue> {
    lines.iter().map(|l| Sue::parse(l)).collect()
}

fn parse_property_list(lines: &[String]) -> HashMap<String, usize> {
    lines.iter().map(|l| parse_property(l)).collect()
}

fn find_sue_part1(sues: &[Sue], property_list: &HashMap<String, usize>) -> Option<usize> {
    sues.iter()
        .enumerate()
        .find(|&(_, s)| s.matches_property_list_part1(property_list))
        .map(|(i, _)| i + 1)
}

fn find_sue_part2(sues: &[Sue], property_list: &HashMap<String, usize>) -> Option<usize> {
    sues.iter()
        .enumerate()
        .find(|&(_, s)| s.matches_property_list_part2(property_list))
        .map(|(i, _)| i + 1)
}

fn main() {
    let mut path = PathBuf::from(args().nth(0).unwrap());

    path.pop();
    path.push("input.txt");
    let sues = parse_sues(&read_file(&path));

    path.pop();
    path.push("search.txt");
    let property_list = parse_property_list(&read_file(&path));

    let sue_id_part1 = find_sue_part1(&sues, &property_list);
    if let Some(actual_sue_id) = sue_id_part1 {
        println!("Part 1: Sue {} is the one that gave you the present.",
                 actual_sue_id);
    } else {
        println!("Part 1: No Sue matches the description.");
    }

    let sue_id_part2 = find_sue_part2(&sues, &property_list);
    if let Some(actual_sue_id) = sue_id_part2 {
        println!("Part 2: Sue {} is the one that gave you the present.",
                 actual_sue_id);
    } else {
        println!("Part 2: No Sue matches the description.");
    }
}

#[test]
fn test_matches_property_list_part1() {
    let property_list = parse_property_list(&["children: 3".to_owned(),
                                              "cats: 7".to_owned(),
                                              "vizslas: 7".to_owned(),
                                              "cars: 8".to_owned()]);
    let sue_matches = Sue::parse("Sue 1: children: 3, cars: 8, vizslas: 7");
    let sue_doesnt_match = Sue::parse("Sue 1: children: 3, cats: 8, vizslas: 7");

    assert!(sue_matches.matches_property_list_part1(&property_list));
    assert!(!sue_doesnt_match.matches_property_list_part1(&property_list));
}

#[test]
fn test_find_sue_part1() {
    let property_list = parse_property_list(&["children: 3".to_owned(),
                                              "cats: 7".to_owned(),
                                              "vizslas: 7".to_owned(),
                                              "cars: 8".to_owned()]);
    let sue_matches = Sue::parse("Sue 1: children: 3, cars: 8, vizslas: 7");
    let sue_doesnt_match = Sue::parse("Sue 1: children: 3, cats: 8, vizslas: 7");

    let sues = [sue_doesnt_match, sue_matches];
    assert_eq!(find_sue_part1(&sues, &property_list), Some(2));
}

#[test]
fn test_find_sue_part2() {
    let property_list = parse_property_list(&["children: 3".to_owned(),
                                              "trees: 7".to_owned(),
                                              "vizslas: 7".to_owned(),
                                              "cars: 8".to_owned()]);
    let sue_matches = Sue::parse("Sue 1: children: 3, cars: 8, tress: 8");
    let sue_doesnt_match = Sue::parse("Sue 1: children: 3, cars: 8, trees: 7");

    let sues = [sue_doesnt_match, sue_matches];
    assert_eq!(find_sue_part2(&sues, &property_list), Some(2));
}