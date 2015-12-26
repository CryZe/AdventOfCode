extern crate permutohedron;

use permutohedron::Heap;
use std::collections::HashMap;
use std::ops::Index;
use std::convert::AsMut;
use std::borrow::ToOwned;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

struct Table<'p, 'n: 'p> {
    persons: Vec<&'p Person<'n>>,
}

impl<'p, 'n> Index<isize> for Table<'p, 'n> {
    type Output = Person<'n>;
    fn index(&self, index: isize) -> &Self::Output {
        let index = (index + self.persons.len() as isize) as usize;
        let index = index % self.persons.len();
        &self.persons[index]
    }
}

impl<'p, 'n> AsMut<[&'p Person<'n>]> for Table<'p, 'n> {
    fn as_mut(&mut self) -> &mut [&'p Person<'n>] {
        &mut self.persons[..]
    }
}

impl<'p, 'n> ToOwned for Table<'p, 'n> {
    type Owned = Table<'p, 'n>;
    fn to_owned(&self) -> Self::Owned {
        Table::new(self.persons.clone())
    }
}

impl<'p, 'n> Table<'p, 'n> {
    fn new(persons: Vec<&'p Person<'n>>) -> Self {
        Table { persons: persons }
    }

    fn calculate_happiness(&self) -> isize {
        self.persons.iter().fold(0, |a, p| a + p.calculate_happiness(self))
    }
}

#[derive(Eq, PartialEq)]
struct Person<'n> {
    name: &'n str,
    next_to_info: HashMap<&'n str, isize>,
}

impl<'n> Person<'n> {
    fn new(name: &'n str, next_to_info: HashMap<&'n str, isize>) -> Self {
        Person {
            name: name,
            next_to_info: next_to_info,
        }
    }

    fn calculate_happiness<'p>(&self, table: &Table<'p, 'n>) -> isize {
        let index = table.persons.iter().position(|p| p == &self).unwrap() as isize;
        let person_left = &table[index - 1];
        let person_right = &table[index + 1];
        self.next_to_info.get(person_left.name).unwrap_or(&0) +
        self.next_to_info.get(person_right.name).unwrap_or(&0)
    }
}

fn find_max_happiness(persons: &[Person]) -> isize {
    let mut vec = Vec::with_capacity(persons.len());
    vec.extend(persons.iter());
    let mut table = Table::new(vec);
    let permutations = Heap::new(&mut table);
    permutations.map(|t| t.calculate_happiness()).max().unwrap() as isize
}

fn read_file(path: &Path) -> String {
    let mut input = String::new();
    let mut file = File::open(path).expect("File could not be found.");
    file.read_to_string(&mut input).expect("File could not be read.");
    input
}

fn parse_persons(input: &str) -> Vec<Person> {
    let mut persons = HashMap::new();

    for line in input.lines() {
        let mut splits = line.split_whitespace();
        let name = splits.nth(0).unwrap();
        let gain_lose = splits.nth(1).unwrap();
        let mut units = splits.nth(0).unwrap().parse().unwrap();
        let other = splits.nth(6).unwrap();
        let other = &other[..other.len() - 1];
        if gain_lose == "lose" {
            units *= -1;
        }

        let entry = persons.entry(name);
        let mut person = entry.or_insert_with(|| Person::new(name, HashMap::new()));
        person.next_to_info.insert(other, units);
    }

    persons.into_iter().map(|(_, v)| v).collect()
}

fn main() {
    let input = read_file(Path::new("input.txt"));
    let mut persons = parse_persons(&input);

    let max_happiness = find_max_happiness(&persons);
    println!("Max Happiness without me: {}", max_happiness);

    persons.push(Person::new("Me", HashMap::new()));
    let max_happiness = find_max_happiness(&persons);
    println!("Max Happiness with me: {}", max_happiness);
}
